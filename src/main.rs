mod channels;
mod db;
mod dms;
mod mcp;
mod models;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use db::Database;
use mcp::{JsonRpcRequest, McpServer};
use models::{NewContact, NewDeal};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db: Database,
    mcp: Arc<McpServer>,
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

    let state = AppState { db, mcp };

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/health", get(health_handler))
        .route("/mcp", post(mcp_handler))
        .route("/api/contacts", get(list_contacts_handler).post(create_contact_handler))
        .route("/api/deals", get(list_deals_handler).post(create_deal_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🦀 Conrad CRM running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn dashboard_handler(State(state): State<AppState>) -> impl IntoResponse {
    let contacts = state.db.list_contacts().unwrap_or_default();
    let deals = state.db.list_deals().unwrap_or_default();
    let total_eur: f64 = deals.iter().map(|d| d.value_cents).sum::<i64>() as f64 / 100.0;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <title>Conrad CRM — High Performance Pure Rust CRM</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --bg: #090d16;
            --surface: #111827;
            --border: #1f293d;
            --primary: #3b82f6;
            --accent: #10b981;
            --text: #f9fafb;
            --muted: #9ca3af;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background: var(--bg);
            color: var(--text);
            font-family: 'Plus Jakarta Sans', sans-serif;
            padding: 32px 20px;
        }}
        .container {{ max-width: 1000px; margin: 0 auto; }}
        header {{ margin-bottom: 32px; border-bottom: 1px solid var(--border); padding-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }}
        h1 {{ font-size: 24px; font-weight: 700; display: flex; align-items: center; gap: 10px; }}
        .badge {{ background: rgba(59, 130, 246, 0.2); color: #60a5fa; border: 1px solid rgba(59,130,246,0.4); padding: 4px 10px; border-radius: 999px; font-size: 12px; font-weight: 600; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-bottom: 32px; }}
        .card {{ background: var(--surface); border: 1px solid var(--border); border-radius: 12px; padding: 20px; }}
        .card h3 {{ font-size: 13px; color: var(--muted); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 8px; }}
        .card .val {{ font-size: 28px; font-weight: 700; color: #fff; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 12px; font-size: 14px; }}
        th, td {{ padding: 12px 14px; text-align: left; border-bottom: 1px solid var(--border); }}
        th {{ color: var(--muted); font-size: 12px; text-transform: uppercase; }}
        .mcp-info {{ background: rgba(16, 185, 129, 0.1); border: 1px solid rgba(16, 185, 129, 0.3); border-radius: 8px; padding: 16px; margin-top: 24px; font-size: 13px; }}
        code {{ background: #000; padding: 2px 6px; border-radius: 4px; font-family: monospace; color: #34d399; }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🦀 Conrad CRM <span class="badge">Pure Rust + MCP</span></h1>
            <div style="font-size: 13px; color: var(--muted);">Architecture: SQLite WAL & Axum</div>
        </header>

        <div class="stats-grid">
            <div class="card">
                <h3>Kontakte</h3>
                <div class="val">{}</div>
            </div>
            <div class="card">
                <h3>Pipeline Wert</h3>
                <div class="val">{:.2} €</div>
            </div>
            <div class="card">
                <h3>MCP Tools</h3>
                <div class="val" style="color: var(--accent);">Aktiv (5)</div>
            </div>
        </div>

        <div class="card">
            <h3>Letzte Kontakte</h3>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Unternehmen</th>
                        <th>E-Mail</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>

        <div class="mcp-info">
            <strong>🤖 Native Model Context Protocol (MCP) Server:</strong><br>
            Endpoint: <code>POST /mcp</code> | Kompatibel mit Claude Code, Antigravity, Cursor, Hermes Agent.
        </div>
    </div>
</body>
</html>"#,
        contacts.len(),
        total_eur,
        if contacts.is_empty() {
            "<tr><td colspan='4' style='color: var(--muted); text-align: center; padding: 24px;'>Noch keine Kontakte angelegt. Nutze das MCP Tool <code>crm_create_contact</code>.</td></tr>".to_string()
        } else {
            contacts.iter().take(5).map(|c| {
                format!(
                    "<tr><td><strong>{}</strong></td><td>{}</td><td>{}</td><td><span class='badge'>{}</span></td></tr>",
                    c.name,
                    c.company.as_deref().unwrap_or("-"),
                    c.email.as_deref().unwrap_or("-"),
                    c.status
                )
            }).collect::<Vec<_>>().join("")
        }
    );

    Html(html)
}

async fn mcp_handler(
    State(state): State<AppState>,
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let response = state.mcp.handle_request(payload);
    (StatusCode::OK, Json(response))
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
