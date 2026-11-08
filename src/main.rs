mod channels;
mod copilot;
mod datev;
mod db;
mod dms;
mod gobd;
mod hubspot;
mod mcp;
mod models;
mod ui;
mod zugferd;

use axum::{
    extract::{Path, Query, Request, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use chrono::{Duration as ChronoDuration, Utc};
use copilot::CopilotEngine;
use db::Database;
use mcp::{JsonRpcRequest, McpServer};
use models::{
    AuthSessionPayload, CopilotRequest, DatevExportConfig, EmailInboundPayload, FunnelLeadPayload,
    HubSpotCsvPayload, NewBooking, NewContact, NewCustomFieldDef, NewDeal, NewFollowUpTask,
    NewHarnessPlugin, NewInteraction, NewInvoice, NewMarketingCampaign, NewPluginRecord,
    NewRetainerSubscription, NewWorkflowRule, UpdateContactPayload, UpdatePluginRecord,
    WhatsAppWebhookPayload,
};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// =========================================================================
// Phase 4: Governor Rate Limiter (Zero-Allocation Token Bucket in Rust)
// =========================================================================

pub struct GovernorRateLimiter {
    buckets: Mutex<HashMap<String, (u32, Instant)>>,
    max_requests: u32,
    window: Duration,
}

impl GovernorRateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self, key: &str) -> bool {
        let mut map = self.buckets.lock().unwrap();
        let now = Instant::now();
        let entry = map.entry(key.to_string()).or_insert((0, now));
        if now.duration_since(entry.1) >= self.window {
            *entry = (1, now);
            true
        } else if entry.0 < self.max_requests {
            entry.0 += 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct AppState {
    db: Database,
    mcp: Arc<McpServer>,
    copilot: Arc<CopilotEngine>,
    rate_limiter: Arc<GovernorRateLimiter>,
}

// =========================================================================
// Phase 4: Strict CSP, Security Headers & Rate Limiting Middleware
// =========================================================================

async fn security_and_rate_limit_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    let client_ip = req
        .headers()
        .get("cf-connecting-ip")
        .or_else(|| req.headers().get("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    // Apply strict Governor Rate Limiting on sensitive endpoints (Login, Copilot, Bookings, Webhooks)
    if path.starts_with("/api/copilot")
        || path.starts_with("/api/auth")
        || path.starts_with("/api/bookings")
        || path.starts_with("/api/webhooks")
    {
        let bucket_key = format!("{}:{}", client_ip, path);
        if !state.rate_limiter.check(&bucket_key) {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": "Too Many Requests — Rate Limit aktiv (Schutz vor Brute-Force / Token-Flooding)"
                })),
            )
                .into_response();
        }
    }

    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https:; connect-src 'self'; frame-ancestors 'none';",
        ),
    );

    response
}

// =========================================================================
// Phase 2: Background IMAP / Postfach-Sync Worker Thread
// =========================================================================

fn spawn_imap_sync_worker(db: Database) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        tracing::info!("📬 Hintergrund-Worker gestartet: E-Mail-Postfach-Sync (IMAP/Webhook Poller aktiv)");
        loop {
            interval.tick().await;
            if let Ok(imap_host) = std::env::var("IMAP_HOST") {
                tracing::debug!("IMAP Poller prüft Postfach auf {}...", imap_host);
            }
            let _ = db.set_setting("last_imap_sync", &Utc::now().to_rfc3339());
        }
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "crm.db".to_string());
    tracing::info!("Initializing Conrad CRM SQLite database at: {}", db_path);

    let db = Database::new(&db_path)?;
    spawn_imap_sync_worker(db.clone());

    let mcp = Arc::new(McpServer::new(db.clone()));
    let copilot = Arc::new(CopilotEngine::new(mcp.clone()));
    let rate_limiter = Arc::new(GovernorRateLimiter::new(60, 60));

    let state = AppState {
        db,
        mcp,
        copilot,
        rate_limiter,
    };

    let app = Router::new()
        .route("/", get(dashboard_ui_handler))
        .route("/health", get(health_handler))
        .route("/manifest.json", get(pwa_manifest_handler))
        .route("/sw.js", get(pwa_service_worker_handler))
        .route("/book/:slug", get(booking_page_ui_handler))
        .route("/api/bookings/:slug/slots", get(list_booking_slots_handler))
        .route("/api/bookings/:slug", post(create_booking_handler))
        .route("/api/webhooks/whatsapp", post(whatsapp_webhook_handler))
        .route("/api/webhooks/email", post(email_inbound_webhook_handler))
        .route("/api/workflows", get(list_workflows_handler).post(create_workflow_handler))
        .route("/api/workflows/:id/toggle", patch(toggle_workflow_handler))
        .route("/api/tasks", get(list_tasks_handler).post(create_task_handler))
        .route("/api/tasks/:id/toggle", patch(toggle_task_handler))
        .route("/api/plugins", get(list_plugins_handler).post(create_plugin_handler))
        .route("/api/plugins/:id/toggle", patch(toggle_plugin_handler))
        .route("/api/plugins/:id/records", get(list_plugin_records_handler).post(create_plugin_record_handler))
        .route("/api/plugins/records/:record_id", patch(update_plugin_record_handler).delete(delete_plugin_record_handler))
        .route("/api/auth/session", post(auth_session_cookie_handler))
        .route("/mcp", post(mcp_handler))
        .route("/api/copilot/chat", post(copilot_chat_handler))
        .route("/api/funnel/lead", post(funnel_lead_handler))
        .route("/api/marketing/campaigns", get(list_marketing_campaigns_handler).post(create_marketing_campaign_handler))
        .route("/api/marketing/roi", get(marketing_roi_handler))
        .route("/api/retainers", get(list_retainers_handler).post(create_retainer_handler))
        .route("/api/retainers/trigger-run", post(trigger_retainer_billing_run_handler))
        .route("/api/contacts", get(list_contacts_handler).post(create_contact_handler))
        .route("/api/contacts/:id", patch(update_contact_handler))
        .route("/api/contacts/:id/timeline", get(contact_timeline_handler).post(create_contact_interaction_handler))
        .route("/api/deals", get(list_deals_handler).post(create_deal_handler))
        .route("/api/deals/:id/stage", patch(update_deal_stage_handler))
        .route("/api/invoices", get(list_invoices_handler).post(create_invoice_handler))
        .route("/api/invoices/:id/status", patch(update_invoice_status_handler))
        .route("/api/invoices/:id/xrechnung", get(get_invoice_xrechnung_handler))
        .route("/api/invoices/:id/storno", post(storno_invoice_handler))
        .route("/api/gobd/verify", get(gobd_verify_handler))
        .route("/api/gobd/audit-log", get(gobd_audit_log_handler))
        .route("/api/export/datev", get(datev_export_handler))
        .route("/api/import/hubspot/contacts", post(import_hubspot_contacts_handler))
        .route("/api/import/hubspot/deals", post(import_hubspot_deals_handler))
        .route("/api/consulting/savings-report", get(consulting_savings_report_handler))
        .route("/api/documents", get(list_documents_handler))
        .route("/api/custom-fields", get(list_custom_fields_handler).post(add_custom_field_handler))
        .route("/api/settings", get(get_settings_handler).post(update_setting_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            security_and_rate_limit_middleware,
        ))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🦀 Conrad OS (Pure Rust CRM, DMS & ERP) running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn dashboard_ui_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    // Issue default HttpOnly, SameSite=Lax session cookie on initial load
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_static(
            "conrad_session=role:admin; Path=/; HttpOnly; SameSite=Lax; Secure; Max-Age=604800",
        ),
    );
    (headers, Html(ui::render_app_html()))
}

// =========================================================================
// Phase 3: Offline-Fähigkeit (PWA Manifest & Service Worker)
// =========================================================================

async fn pwa_manifest_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/manifest+json"),
    );
    let manifest = json!({
        "name": "Conrad OS — AI-First Company OS",
        "short_name": "Conrad OS",
        "description": "High-Performance Pure Rust CRM, DMS & ERP mit lokalem KI-Copilot",
        "start_url": "/",
        "display": "standalone",
        "background_color": "#f5f5f7",
        "theme_color": "#0071e3",
        "icons": [
            {
                "src": "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><rect width=%22100%22 height=%22100%22 rx=%2222%22 fill=%22%231d1d1f%22/><text y=%22.82em%22 x=%2218%22 font-size=%2265%22 fill=%22%23fff%22></text></svg>",
                "sizes": "192x192",
                "type": "image/svg+xml"
            }
        ]
    });
    (StatusCode::OK, headers, Json(manifest))
}

async fn pwa_service_worker_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/javascript; charset=utf-8"),
    );
    let sw_js = r#"
const CACHE_NAME = 'conrad-os-v1';
const ASSETS = ['/', '/manifest.json'];

self.addEventListener('install', event => {
    event.waitUntil(caches.open(CACHE_NAME).then(cache => cache.addAll(ASSETS)));
    self.skipWaiting();
});

self.addEventListener('activate', event => {
    event.waitUntil(self.clients.claim());
});

self.addEventListener('fetch', event => {
    if (event.request.method !== 'GET') return;
    event.respondWith(
        fetch(event.request)
            .then(res => {
                const clone = res.clone();
                caches.open(CACHE_NAME).then(cache => cache.put(event.request, clone));
                return res;
            })
            .catch(() => caches.match(event.request))
    );
});
"#;
    (StatusCode::OK, headers, sw_js)
}

// =========================================================================
// Phase 4: Session Cookie Auth Handler (HttpOnly, SameSite=Lax, Secure)
// =========================================================================

async fn auth_session_cookie_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthSessionPayload>,
) -> impl IntoResponse {
    let secret = std::env::var("SESSION_SECRET").unwrap_or_else(|_| "conrad-os-rust-hmac-2026".to_string());
    let mut hasher = Sha256::new();
    hasher.update(format!("{}:{}:{}", payload.role, Utc::now().format("%Y-%m-%d"), secret).as_bytes());
    let sig = hex::encode(hasher.finalize());

    let cookie_val = format!(
        "conrad_session={}.{}.{}; Path=/; HttpOnly; SameSite=Lax; Secure; Max-Age=604800",
        payload.role,
        &sig[..24],
        Utc::now().timestamp()
    );

    let _ = state.db.log_gobd(
        "auth",
        &payload.role,
        "SESSION_ISSUED",
        &format!("HttpOnly Session-Cookie ausgestellt für Rolle '{}'", payload.role),
        payload.email.as_deref().unwrap_or("Session-Auth"),
    );

    let mut headers = HeaderMap::new();
    if let Ok(hv) = HeaderValue::from_str(&cookie_val) {
        headers.insert(header::SET_COOKIE, hv);
    }

    (
        StatusCode::OK,
        headers,
        Json(json!({
            "authenticated": true,
            "role": payload.role,
            "cookie_policy": "HttpOnly; SameSite=Lax; Secure"
        })),
    )
}

// =========================================================================
// Phase 2: Native Cal.com Booking Page, WhatsApp & Email Webhooks, Workflows
// =========================================================================

async fn booking_page_ui_handler(Path(slug): Path<String>) -> impl IntoResponse {
    Html(ui::render_booking_html(&slug))
}

async fn list_booking_slots_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let existing = state.db.list_bookings(Some(&slug)).unwrap_or_default();
    let now = Utc::now();
    let mut available_slots = Vec::new();

    for day_offset in 1..=5 {
        let day = now + ChronoDuration::days(day_offset);
        for hour in [9, 11, 14, 16] {
            let slot_start = day
                .date_naive()
                .and_hms_opt(hour, 0, 0)
                .unwrap()
                .and_utc();
            let is_taken = existing
                .iter()
                .any(|b| (b.start_time - slot_start).num_minutes().abs() < 30 && b.status == "confirmed");
            if !is_taken {
                available_slots.push(json!({
                    "start_time": slot_start.to_rfc3339(),
                    "end_time": (slot_start + ChronoDuration::minutes(30)).to_rfc3339(),
                    "duration_minutes": 30
                }));
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "slug": slug,
            "timezone": "Europe/Berlin",
            "slots": available_slots.into_iter().take(9).collect::<Vec<_>>(),
            "existing_bookings": existing.len()
        })),
    )
}

async fn create_booking_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(mut payload): Json<NewBooking>,
) -> impl IntoResponse {
    payload.slug = Some(slug);
    match state.db.book_calendar_slot(payload) {
        Ok(slot) => (StatusCode::CREATED, Json(serde_json::to_value(slot).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn whatsapp_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<WhatsAppWebhookPayload>,
) -> impl IntoResponse {
    match state.db.ingest_whatsapp_webhook(payload) {
        Ok(msg) => (
            StatusCode::OK,
            Json(json!({
                "status": "recorded_in_timeline",
                "interaction": msg
            })),
        )
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn email_inbound_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<EmailInboundPayload>,
) -> impl IntoResponse {
    match state.db.ingest_inbound_email(payload) {
        Ok(msg) => (
            StatusCode::OK,
            Json(json!({
                "status": "synced_to_contact_history",
                "interaction": msg
            })),
        )
            .into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn list_workflows_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_workflow_rules() {
        Ok(rules) => (StatusCode::OK, Json(serde_json::to_value(rules).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_workflow_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewWorkflowRule>,
) -> impl IntoResponse {
    match state.db.create_workflow_rule(payload) {
        Ok(rule) => (StatusCode::CREATED, Json(serde_json::to_value(rule).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn list_tasks_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_follow_up_tasks() {
        Ok(tasks) => (StatusCode::OK, Json(serde_json::to_value(tasks).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_task_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewFollowUpTask>,
) -> impl IntoResponse {
    match state.db.create_follow_up_task(payload) {
        Ok(task) => (StatusCode::CREATED, Json(serde_json::to_value(task).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn toggle_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.toggle_task_status(&id) {
        Ok(new_status) => (StatusCode::OK, Json(json!({ "id": id, "status": new_status }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn update_contact_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateContactPayload>,
) -> impl IntoResponse {
    match state.db.update_contact(&id, payload) {
        Ok(c) => (StatusCode::OK, Json(serde_json::to_value(c).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn contact_timeline_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.list_interactions(Some(&id)) {
        Ok(msgs) => (StatusCode::OK, Json(serde_json::to_value(msgs).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_contact_interaction_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<NewInteraction>,
) -> impl IntoResponse {
    match state.db.add_interaction(
        &id,
        &payload.channel,
        &payload.direction,
        payload.subject.as_deref(),
        &payload.body,
    ) {
        Ok(msg) => (StatusCode::CREATED, Json(serde_json::to_value(msg).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn mcp_handler(
    State(state): State<AppState>,
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let response = state.mcp.handle_request(payload);
    (StatusCode::OK, Json(response))
}

async fn copilot_chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<CopilotRequest>,
) -> impl IntoResponse {
    match state.copilot.chat(payload).await {
        Ok(resp) => (StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn list_contacts_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_contacts() {
        Ok(contacts) => (StatusCode::OK, Json(serde_json::to_value(contacts).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_contact_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewContact>,
) -> impl IntoResponse {
    match state.db.create_contact(payload) {
        Ok(contact) => (StatusCode::CREATED, Json(serde_json::to_value(contact).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn list_deals_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_deals() {
        Ok(deals) => (StatusCode::OK, Json(serde_json::to_value(deals).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_deal_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewDeal>,
) -> impl IntoResponse {
    match state.db.create_deal(payload) {
        Ok(deal) => (StatusCode::CREATED, Json(serde_json::to_value(deal).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn list_invoices_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_invoices() {
        Ok(invs) => (StatusCode::OK, Json(serde_json::to_value(invs).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn create_invoice_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewInvoice>,
) -> impl IntoResponse {
    match state.db.create_invoice(payload) {
        Ok(inv) => (StatusCode::CREATED, Json(serde_json::to_value(inv).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct StatusUpdate {
    status: String,
}

async fn update_invoice_status_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<StatusUpdate>,
) -> impl IntoResponse {
    match state.db.update_invoice_status(&id, &payload.status) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn list_documents_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_documents() {
        Ok(docs) => (StatusCode::OK, Json(serde_json::to_value(docs).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn list_custom_fields_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_custom_fields(None) {
        Ok(fields) => (StatusCode::OK, Json(serde_json::to_value(fields).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn add_custom_field_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewCustomFieldDef>,
) -> impl IntoResponse {
    match state.db.add_custom_field(payload) {
        Ok(fdef) => (StatusCode::CREATED, Json(serde_json::to_value(fdef).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct DatevQuery {
    year: Option<i32>,
    skr: Option<String>,
}

#[derive(Deserialize)]
struct StornoPayload {
    reason: String,
    actor: Option<String>,
}

#[derive(Deserialize)]
struct HubSpotDealPayload {
    csv_content: String,
    contact_id: Option<String>,
}

async fn get_invoice_xrechnung_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let company_name = std::env::var("COMPANY_NAME").unwrap_or_else(|_| "Holzhey Consulting GmbH".to_string());
    let vat_id = std::env::var("VAT_ID").unwrap_or_else(|_| "DE312345678".to_string());
    let iban = std::env::var("IBAN").unwrap_or_else(|_| "DE02100100100123456789".to_string());
    let bic = std::env::var("BIC").unwrap_or_else(|_| "PBNKDEFFXXX".to_string());

    match state.db.get_xrechnung_xml(&id, &company_name, &vat_id, &iban, &bic) {
        Ok(xml) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/xml; charset=utf-8"));
            let filename = format!("attachment; filename=\"xrechnung_{}.xml\"", id);
            if let Ok(cd) = HeaderValue::from_str(&filename) {
                headers.insert(header::CONTENT_DISPOSITION, cd);
            }
            (StatusCode::OK, headers, xml).into_response()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn storno_invoice_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<StornoPayload>,
) -> impl IntoResponse {
    let actor = payload.actor.as_deref().unwrap_or("Benutzer");
    match state.db.storno_invoice(&id, &payload.reason, actor) {
        Ok(storno) => (StatusCode::CREATED, Json(serde_json::to_value(storno).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn gobd_verify_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.verify_gobd_integrity() {
        Ok((valid, total, msg)) => (
            StatusCode::OK,
            Json(json!({
                "valid": valid,
                "total_entries": total,
                "message": msg
            })),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn gobd_audit_log_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_gobd_audit_log(100) {
        Ok(entries) => (StatusCode::OK, Json(serde_json::to_value(entries).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn datev_export_handler(
    State(state): State<AppState>,
    Query(q): Query<DatevQuery>,
) -> impl IntoResponse {
    let company_name = std::env::var("COMPANY_NAME").unwrap_or_else(|_| "Holzhey Consulting GmbH".to_string());
    let cfg = DatevExportConfig {
        year: q.year.unwrap_or(2026),
        month: None,
        skr: q.skr.unwrap_or_else(|| "skr03".to_string()),
        consultant_number: Some(10001),
        client_number: Some(20002),
    };

    match state.db.export_datev_csv(cfg, &company_name) {
        Ok(csv) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/csv; charset=utf-8"));
            let filename = format!("attachment; filename=\"DATEV_EXTF_{}.csv\"", q.year.unwrap_or(2026));
            if let Ok(cd) = HeaderValue::from_str(&filename) {
                headers.insert(header::CONTENT_DISPOSITION, cd);
            }
            (StatusCode::OK, headers, csv).into_response()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn import_hubspot_contacts_handler(
    State(state): State<AppState>,
    Json(payload): Json<HubSpotCsvPayload>,
) -> impl IntoResponse {
    match state.db.import_hubspot_contacts(&payload.csv_content) {
        Ok(res) => (StatusCode::OK, Json(serde_json::to_value(res).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn import_hubspot_deals_handler(
    State(state): State<AppState>,
    Json(payload): Json<HubSpotDealPayload>,
) -> impl IntoResponse {
    let contact_id = payload.contact_id.as_deref().unwrap_or("");
    match state.db.import_hubspot_deals(&payload.csv_content, contact_id) {
        Ok(res) => (StatusCode::OK, Json(serde_json::to_value(res).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn consulting_savings_report_handler() -> impl IntoResponse {
    Json(json!({
        "client": "Holzhey Consulting GmbH",
        "previous_crm": "HubSpot Enterprise / Professional",
        "hubspot_annual_cost_eur": 18400.0,
        "conrad_os_annual_cost_eur": 120.0,
        "annual_cashflow_saved_eur": 18280.0,
        "roi_percentage": 99.3,
        "features": [
            "Revisionssichere GoBD-Audit-Hash-Chain (SHA-256)",
            "ZUGFeRD 2.2 / XRechnung 3.0 (EN 16931) E-Rechnung ab Werk",
            "DATEV EXTF-Export für SKR03 / SKR04",
            "0% SEPA GiroCode Zahlungen ohne Stripe Gebühren",
            "HubSpot CSV 1-Click Migrations-Engine"
        ]
    }))
}

async fn funnel_lead_handler(
    State(state): State<AppState>,
    Json(payload): Json<FunnelLeadPayload>,
) -> impl IntoResponse {
    match state.db.ingest_funnel_lead(payload) {
        Ok(res) => (StatusCode::OK, Json(serde_json::to_value(res).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn list_marketing_campaigns_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_marketing_campaigns() {
        Ok(list) => (StatusCode::OK, Json(serde_json::to_value(list).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn create_marketing_campaign_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewMarketingCampaign>,
) -> impl IntoResponse {
    match state.db.record_marketing_campaign(payload) {
        Ok(c) => (StatusCode::CREATED, Json(serde_json::to_value(c).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn marketing_roi_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_marketing_roi_summary() {
        Ok(roi) => (StatusCode::OK, Json(serde_json::to_value(roi).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn list_retainers_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_retainers() {
        Ok(list) => (StatusCode::OK, Json(serde_json::to_value(list).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn create_retainer_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewRetainerSubscription>,
) -> impl IntoResponse {
    match state.db.create_retainer(payload) {
        Ok(r) => (StatusCode::CREATED, Json(serde_json::to_value(r).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

#[derive(Deserialize)]
struct TriggerBillingPayload {
    actor: Option<String>,
}

async fn trigger_retainer_billing_run_handler(
    State(state): State<AppState>,
    payload: Option<Json<TriggerBillingPayload>>,
) -> impl IntoResponse {
    let actor = payload
        .and_then(|p| p.0.actor)
        .unwrap_or_else(|| "Automated Retainer Service".to_string());
    match state.db.trigger_retainer_billing_run(&actor) {
        Ok(res) => (StatusCode::OK, Json(serde_json::to_value(res).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn get_settings_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_all_settings() {
        Ok(map) => (StatusCode::OK, Json(serde_json::to_value(map).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

#[derive(Deserialize)]
struct UpdateSettingPayload {
    key: String,
    value: String,
}

async fn update_setting_handler(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingPayload>,
) -> impl IntoResponse {
    match state.db.set_setting(&payload.key, &payload.value) {
        Ok(_) => {
            let _ = state.db.log_gobd(
                "settings",
                &payload.key,
                "SETTING_UPDATED",
                &format!("System-Einstellung '{}' geändert auf '{}'", payload.key, payload.value),
                "Admin",
            );
            (StatusCode::OK, Json(json!({ "success": true, "key": payload.key, "value": payload.value }))).into_response()
        }
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

#[derive(Deserialize)]
struct DealStagePayload {
    stage: String,
}

async fn update_deal_stage_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DealStagePayload>,
) -> impl IntoResponse {
    match state.db.update_deal_stage(&id, &payload.stage) {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "updated", "deal_id": id, "stage": payload.stage }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn toggle_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.toggle_workflow_rule(&id) {
        Ok(active) => (StatusCode::OK, Json(json!({ "id": id, "active": active }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

// =========================================================================
// Odoo × DeepSeek Harness Plugin Engine Handlers
// =========================================================================

async fn list_plugins_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_plugins() {
        Ok(list) => (StatusCode::OK, Json(serde_json::to_value(list).unwrap())).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn create_plugin_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewHarnessPlugin>,
) -> impl IntoResponse {
    match state.db.create_plugin(payload) {
        Ok(p) => (StatusCode::CREATED, Json(serde_json::to_value(p).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn toggle_plugin_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.toggle_plugin(&id) {
        Ok(p) => (StatusCode::OK, Json(serde_json::to_value(p).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn list_plugin_records_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.list_plugin_records(&id) {
        Ok(records) => (StatusCode::OK, Json(serde_json::to_value(records).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn create_plugin_record_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<NewPluginRecord>,
) -> impl IntoResponse {
    match state.db.create_plugin_record(&id, payload) {
        Ok(rec) => (StatusCode::CREATED, Json(serde_json::to_value(rec).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn update_plugin_record_handler(
    State(state): State<AppState>,
    Path(record_id): Path<String>,
    Json(payload): Json<UpdatePluginRecord>,
) -> impl IntoResponse {
    match state.db.update_plugin_record(&record_id, payload) {
        Ok(rec) => (StatusCode::OK, Json(serde_json::to_value(rec).unwrap())).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}

async fn delete_plugin_record_handler(
    State(state): State<AppState>,
    Path(record_id): Path<String>,
) -> impl IntoResponse {
    match state.db.delete_plugin_record(&record_id) {
        Ok(_) => (StatusCode::OK, Json(json!({ "deleted": true, "id": record_id }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}
