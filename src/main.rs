mod channels;
mod copilot;
mod db;
mod dms;
mod mcp;
mod models;
mod ui;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, patch, post},
    Json, Router,
};
use copilot::CopilotEngine;
use db::Database;
use mcp::{JsonRpcRequest, McpServer};
use models::{CopilotRequest, NewContact, NewCustomFieldDef, NewDeal, NewInvoice};
use serde::Deserialize;
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
        .route("/api/contacts", get(list_contacts_handler).post(create_contact_handler))
        .route("/api/deals", get(list_deals_handler).post(create_deal_handler))
        .route("/api/invoices", get(list_invoices_handler).post(create_invoice_handler))
        .route("/api/invoices/:id/status", patch(update_invoice_status_handler))
        .route("/api/documents", get(list_documents_handler))
        .route("/api/custom-fields", get(list_custom_fields_handler).post(add_custom_field_handler))
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
