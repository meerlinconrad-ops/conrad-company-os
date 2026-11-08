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
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, patch, post},
    Json, Router,
};
use copilot::CopilotEngine;
use db::Database;
use mcp::{JsonRpcRequest, McpServer};
use models::{
    CopilotRequest, DatevExportConfig, FunnelLeadPayload, HubSpotCsvPayload, NewContact,
    NewCustomFieldDef, NewDeal, NewInvoice, NewMarketingCampaign, NewRetainerSubscription,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db: Database,
    mcp: Arc<McpServer>,
    copilot: Arc<CopilotEngine>,
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
    let mcp = Arc::new(McpServer::new(db.clone()));
    let copilot = Arc::new(CopilotEngine::new(mcp.clone()));

    let state = AppState { db, mcp, copilot };

    let app = Router::new()
        .route("/", get(dashboard_ui_handler))
        .route("/health", get(health_handler))
        .route("/mcp", post(mcp_handler))
        .route("/api/copilot/chat", post(copilot_chat_handler))
        .route("/api/funnel/lead", post(funnel_lead_handler))
        .route("/api/marketing/campaigns", get(list_marketing_campaigns_handler).post(create_marketing_campaign_handler))
        .route("/api/marketing/roi", get(marketing_roi_handler))
        .route("/api/retainers", get(list_retainers_handler).post(create_retainer_handler))
        .route("/api/retainers/trigger-run", post(trigger_retainer_billing_run_handler))
        .route("/api/contacts", get(list_contacts_handler).post(create_contact_handler))
        .route("/api/deals", get(list_deals_handler).post(create_deal_handler))
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
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
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
    Html(ui::render_app_html())
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
        Ok(_) => (StatusCode::OK, Json(json!({ "success": true, "key": payload.key, "value": payload.value }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "error": err.to_string() }))).into_response(),
    }
}
