# 🦀 Conrad CRM — High-Volume Pure Rust CRM & DMS with Native MCP

A high-performance, single-binary CRM, DMS (Document Management System), and multi-channel communication engine written 100% in Rust. Designed to run smoothly on the smallest VPS (20–40 MB RAM) while scaling to millions of records with sub-millisecond latencies.

## 🚀 Key Features

- **Pure Rust Performance**: Built on Axum, Tokio, and SQLite with WAL (Write-Ahead Logging) mode.
- **Native MCP Server**: Exposes Model Context Protocol (JSON-RPC 2.0) over `POST /mcp` for instant plug-and-play with Claude Code, Hermes Agent, Cursor, and Antigravity.
- **Embedded Document Management (DMS)**: Safe path handling, metadata indexing, attachment associations.
- **Multi-Channel Timelines**: Unified logging and dispatching for WhatsApp and Email interactions.
- **Minimal Footprint**: Single statically linked binary, runs in a 10MB scratch container with zero runtime dependencies.
- **Permissive MIT License**: 100% commercial and self-hosting freedom (unlike restrictive AGPL/SSPL alternatives).

---

## 🛠️ MCP Tools for AI Agents

Any agent can inspect and control Conrad CRM using standard MCP tools:

| Tool Name | Parameters | Purpose |
|---|---|---|
| `crm_search_contacts` | `query` (string) | Fast fuzzy search across contacts and companies |
| `crm_create_contact` | `name`, `email`, `phone`, `company`, `status` | Create new prospect or client |
| `crm_create_deal` | `contact_id`, `title`, `value_cents`, `currency`, `stage` | Pipeline deal creation |
| `crm_send_interaction`| `contact_id`, `channel`, `direction`, `body` | Log email, WhatsApp, or phone note |
| `crm_list_pipeline` | None | Full pipeline overview & deal metrics |

---

## 💻 Quick Start

### Build & Run
```bash
cargo build --release
./target/release/conrad-crm
```

### Environment Variables
- `PORT`: HTTP port (default: `8080`)
- `DATABASE_URL`: Path to SQLite database file (default: `crm.db`)

### Dashboard
Open `http://localhost:8080` in your browser for the real-time embedded status dashboard.
