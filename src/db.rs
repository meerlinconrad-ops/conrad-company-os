#![allow(dead_code)]
use anyhow::Result;
use chrono::{Duration, Utc};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::{
    CashflowStats, Contact, CustomFieldDef, Deal, DocumentMeta, InteractionMessage, Invoice,
    InvoiceItem, NewContact, NewCustomFieldDef, NewDeal, NewDocument, NewInvoice, NewProductService,
    ProductService,
};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // High-performance Pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA cache_size = -64000;
             PRAGMA temp_store = MEMORY;",
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        db.seed_defaults_if_empty()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                company TEXT,
                status TEXT NOT NULL DEFAULT 'lead',
                tags TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS deals (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                title TEXT NOT NULL,
                value_cents INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'EUR',
                stage TEXT NOT NULL DEFAULT 'discovery',
                expected_close_date TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                channel TEXT NOT NULL,
                direction TEXT NOT NULL,
                subject TEXT,
                body TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'sent',
                timestamp TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                sku TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                unit_price_cents INTEGER NOT NULL DEFAULT 0,
                vat_rate_percent REAL NOT NULL DEFAULT 19.0,
                unit TEXT NOT NULL DEFAULT 'Pauschal',
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS invoices (
                id TEXT PRIMARY KEY,
                invoice_number TEXT UNIQUE NOT NULL,
                contact_id TEXT NOT NULL,
                deal_id TEXT,
                status TEXT NOT NULL DEFAULT 'draft',
                issued_date TEXT NOT NULL,
                due_date TEXT NOT NULL,
                net_cents INTEGER NOT NULL DEFAULT 0,
                vat_cents INTEGER NOT NULL DEFAULT 0,
                gross_cents INTEGER NOT NULL DEFAULT 0,
                items_json TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE RESTRICT,
                FOREIGN KEY (deal_id) REFERENCES deals(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                contact_id TEXT,
                invoice_id TEXT,
                title TEXT NOT NULL,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size_bytes INTEGER NOT NULL DEFAULT 0,
                mime_type TEXT NOT NULL DEFAULT 'application/octet-stream',
                category TEXT NOT NULL DEFAULT 'general',
                tags TEXT NOT NULL DEFAULT '',
                extracted_text TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE SET NULL,
                FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS custom_fields (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                key TEXT NOT NULL,
                label TEXT NOT NULL,
                field_type TEXT NOT NULL,
                options_json TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(entity_type, key)
            );

            CREATE TABLE IF NOT EXISTS custom_values (
                entity_id TEXT NOT NULL,
                field_id TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY (entity_id, field_id)
            );

            CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(name);
            CREATE INDEX IF NOT EXISTS idx_deals_stage ON deals(stage);
            CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
            CREATE INDEX IF NOT EXISTS idx_docs_category ON documents(category);",
        )?;
        Ok(())
    }

    fn seed_defaults_if_empty(&self) -> Result<()> {
        let count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM contacts", [], |r| r.get(0))?
        };

        if count == 0 {
            // Seed sample clients
            let c1 = self.create_contact(NewContact {
                name: "Jens Ohrladen".to_string(),
                email: Some("info@der-ohrladen.de".to_string()),
                phone: Some("+49 6202 920000".to_string()),
                company: Some("Der Ohrladen Hörgeräte".to_string()),
                status: Some("customer".to_string()),
                tags: Some("Akustiker, Retainer, n8n".to_string()),
            })?;

            let c2 = self.create_contact(NewContact {
                name: "Alexandra Imkerin".to_string(),
                email: Some("alex@schlossgarten-honig.de".to_string()),
                phone: Some("+49 171 4455667".to_string()),
                company: Some("Schlossgarten Imkerei".to_string()),
                status: Some("qualified".to_string()),
                tags: Some("E-Commerce, Social-Media".to_string()),
            })?;

            // Seed sample products
            self.create_product(NewProductService {
                sku: "WORKFLOW-RETAINER-M".to_string(),
                name: "Monatlicher Workflow & KI-Betrieb".to_string(),
                description: Some("Laufende Betreuung von n8n-Workflows, Bugfixes und Modell-Updates".to_string()),
                unit_price_cents: 9900,
                vat_rate_percent: Some(19.0),
                unit: Some("Monat".to_string()),
            })?;

            self.create_product(NewProductService {
                sku: "SETUP-POSTFACH-TRIAGE".to_string(),
                name: "Setup: E-Mail Triage & Postfach Autopilot".to_string(),
                description: Some("Einrichtung lokales LLM + automatische Priorisierung und Antwortentwürfe".to_string()),
                unit_price_cents: 89000,
                vat_rate_percent: Some(19.0),
                unit: Some("Pauschal".to_string()),
            })?;

            // Seed sample deals
            let d1 = self.create_deal(NewDeal {
                contact_id: c1.id.clone(),
                title: "Jahres-Retainer KI-Workflows 2026/27".to_string(),
                value_cents: 118800, // 99€ * 12
                currency: Some("EUR".to_string()),
                stage: Some("closed_won".to_string()),
                expected_close_date: Some("2026-11-01".to_string()),
            })?;

            self.create_deal(NewDeal {
                contact_id: c2.id.clone(),
                title: "Instagram DM to WhatsApp Lead Engine".to_string(),
                value_cents: 95000,
                currency: Some("EUR".to_string()),
                stage: Some("proposal".to_string()),
                expected_close_date: Some("2026-11-20".to_string()),
            })?;

            // Seed sample invoice
            self.create_invoice(NewInvoice {
                contact_id: c1.id.clone(),
                deal_id: Some(d1.id),
                invoice_number: Some("RE-2026-001".to_string()),
                items: vec![
                    InvoiceItem {
                        description: "Betreuung & Hosting lokales n8n (November 2026)".to_string(),
                        quantity: 1.0,
                        unit_price_cents: 9900,
                        total_cents: 9900,
                    }
                ],
                due_days: Some(14),
            })?;

            // Seed custom fields
            self.add_custom_field(NewCustomFieldDef {
                entity_type: "contact".to_string(),
                key: "branche".to_string(),
                label: "Branche".to_string(),
                field_type: "text".to_string(),
                options: None,
            })?;
        }
        Ok(())
    }

    // =========================================================================
    // CRM Methods
    // =========================================================================

    pub fn create_contact(&self, input: NewContact) -> Result<Contact> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let status = input.status.unwrap_or_else(|| "lead".to_string());
        let tags = input.tags.unwrap_or_default();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO contacts (id, name, email, phone, company, status, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id,
                input.name,
                input.email,
                input.phone,
                input.company,
                status,
                tags,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        Ok(Contact {
            id,
            name: input.name,
            email: input.email,
            phone: input.phone,
            company: input.company,
            status,
            tags,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn search_contacts(&self, query: &str) -> Result<Vec<Contact>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, name, email, phone, company, status, tags, created_at, updated_at
             FROM contacts
             WHERE name LIKE ?1 OR email LIKE ?1 OR company LIKE ?1 OR phone LIKE ?1 OR tags LIKE ?1
             ORDER BY updated_at DESC
             LIMIT 50",
        )?;

        let contacts = stmt
            .query_map([pattern], |row| {
                let c_str: String = row.get(7)?;
                let u_str: String = row.get(8)?;
                Ok(Contact {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    phone: row.get(3)?,
                    company: row.get(4)?,
                    status: row.get(5)?,
                    tags: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&u_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(contacts)
    }

    pub fn list_contacts(&self) -> Result<Vec<Contact>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, email, phone, company, status, tags, created_at, updated_at
             FROM contacts
             ORDER BY updated_at DESC
             LIMIT 100",
        )?;

        let contacts = stmt
            .query_map([], |row| {
                let c_str: String = row.get(7)?;
                let u_str: String = row.get(8)?;
                Ok(Contact {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    phone: row.get(3)?,
                    company: row.get(4)?,
                    status: row.get(5)?,
                    tags: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&u_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(contacts)
    }

    pub fn create_deal(&self, input: NewDeal) -> Result<Deal> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let currency = input.currency.unwrap_or_else(|| "EUR".to_string());
        let stage = input.stage.unwrap_or_else(|| "discovery".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO deals (id, contact_id, title, value_cents, currency, stage, expected_close_date, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                input.contact_id,
                input.title,
                input.value_cents,
                currency,
                stage,
                input.expected_close_date,
                now.to_rfc3339(),
            ],
        )?;

        // Fetch contact name for convenience
        let contact_name: Option<String> = conn
            .query_row("SELECT name FROM contacts WHERE id = ?1", [&input.contact_id], |r| r.get(0))
            .ok();

        Ok(Deal {
            id,
            contact_id: input.contact_id,
            contact_name,
            title: input.title,
            value_cents: input.value_cents,
            currency,
            stage,
            expected_close_date: input.expected_close_date,
            created_at: now,
        })
    }

    pub fn list_deals(&self) -> Result<Vec<Deal>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT d.id, d.contact_id, c.name, d.title, d.value_cents, d.currency, d.stage, d.expected_close_date, d.created_at
             FROM deals d
             LEFT JOIN contacts c ON d.contact_id = c.id
             ORDER BY d.created_at DESC",
        )?;

        let deals = stmt
            .query_map([], |row| {
                let c_str: String = row.get(8)?;
                Ok(Deal {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    contact_name: row.get(2)?,
                    title: row.get(3)?,
                    value_cents: row.get(4)?,
                    currency: row.get(5)?,
                    stage: row.get(6)?,
                    expected_close_date: row.get(7)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(deals)
    }

    pub fn update_deal_stage(&self, deal_id: &str, new_stage: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE deals SET stage = ?1 WHERE id = ?2", params![new_stage, deal_id])?;
        Ok(())
    }

    pub fn add_interaction(&self, contact_id: &str, channel: &str, direction: &str, subject: Option<&str>, body: &str) -> Result<InteractionMessage> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (id, contact_id, channel, direction, subject, body, status, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'delivered', ?7)",
            params![id, contact_id, channel, direction, subject, body, now.to_rfc3339()],
        )?;

        Ok(InteractionMessage {
            id,
            contact_id: contact_id.to_string(),
            channel: channel.to_string(),
            direction: direction.to_string(),
            subject: subject.map(String::from),
            body: body.to_string(),
            status: "delivered".to_string(),
            timestamp: now,
        })
    }

    pub fn list_interactions(&self, contact_id: Option<&str>) -> Result<Vec<InteractionMessage>> {
        let conn = self.conn.lock().unwrap();
        let sql = if let Some(cid) = contact_id {
            format!("SELECT id, contact_id, channel, direction, subject, body, status, timestamp FROM messages WHERE contact_id = '{}' ORDER BY timestamp DESC LIMIT 50", cid)
        } else {
            "SELECT id, contact_id, channel, direction, subject, body, status, timestamp FROM messages ORDER BY timestamp DESC LIMIT 50".to_string()
        };

        let mut stmt = conn.prepare(&sql)?;
        let msgs = stmt
            .query_map([], |row| {
                let ts_str: String = row.get(7)?;
                Ok(InteractionMessage {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    channel: row.get(2)?,
                    direction: row.get(3)?,
                    subject: row.get(4)?,
                    body: row.get(5)?,
                    status: row.get(6)?,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&ts_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(msgs)
    }

    // =========================================================================
    // ERP Methods (Products & Invoicing)
    // =========================================================================

    pub fn create_product(&self, input: NewProductService) -> Result<ProductService> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let vat = input.vat_rate_percent.unwrap_or(19.0);
        let unit = input.unit.unwrap_or_else(|| "Pauschal".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO products (id, sku, name, description, unit_price_cents, vat_rate_percent, unit, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, input.sku, input.name, input.description, input.unit_price_cents, vat, unit, now.to_rfc3339()],
        )?;

        Ok(ProductService {
            id,
            sku: input.sku,
            name: input.name,
            description: input.description,
            unit_price_cents: input.unit_price_cents,
            vat_rate_percent: vat,
            unit,
            created_at: now,
        })
    }

    pub fn list_products(&self) -> Result<Vec<ProductService>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, sku, name, description, unit_price_cents, vat_rate_percent, unit, created_at FROM products ORDER BY name ASC")?;
        let items = stmt
            .query_map([], |row| {
                let c_str: String = row.get(7)?;
                Ok(ProductService {
                    id: row.get(0)?,
                    sku: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    unit_price_cents: row.get(4)?,
                    vat_rate_percent: row.get(5)?,
                    unit: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(items)
    }

    pub fn create_invoice(&self, input: NewInvoice) -> Result<Invoice> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let due_date = now + Duration::days(input.due_days.unwrap_or(14));

        let net_cents: i64 = input.items.iter().map(|i| i.total_cents).sum();
        let vat_cents: i64 = (net_cents as f64 * 0.19).round() as i64;
        let gross_cents = net_cents + vat_cents;

        let invoice_num = input.invoice_number.unwrap_or_else(|| {
            format!("RE-{}-{}", now.format("%Y"), Utc::now().timestamp_subsec_millis())
        });
        let items_json = serde_json::to_string(&input.items).unwrap_or_else(|_| "[]".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO invoices (id, invoice_number, contact_id, deal_id, status, issued_date, due_date, net_cents, vat_cents, gross_cents, items_json, created_at)
             VALUES (?1, ?2, ?3, ?4, 'draft', ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                id,
                invoice_num,
                input.contact_id,
                input.deal_id,
                now.format("%Y-%m-%d").to_string(),
                due_date.format("%Y-%m-%d").to_string(),
                net_cents,
                vat_cents,
                gross_cents,
                items_json,
                now.to_rfc3339(),
            ],
        )?;

        let contact_name: Option<String> = conn
            .query_row("SELECT name FROM contacts WHERE id = ?1", [&input.contact_id], |r| r.get(0))
            .ok();

        Ok(Invoice {
            id,
            invoice_number: invoice_num,
            contact_id: input.contact_id,
            contact_name,
            deal_id: input.deal_id,
            status: "draft".to_string(),
            issued_date: now.format("%Y-%m-%d").to_string(),
            due_date: due_date.format("%Y-%m-%d").to_string(),
            net_cents,
            vat_cents,
            gross_cents,
            items_json,
            created_at: now,
        })
    }

    pub fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT i.id, i.invoice_number, i.contact_id, c.name, i.deal_id, i.status, i.issued_date, i.due_date, i.net_cents, i.vat_cents, i.gross_cents, i.items_json, i.created_at
             FROM invoices i
             LEFT JOIN contacts c ON i.contact_id = c.id
             ORDER BY i.created_at DESC",
        )?;

        let invs = stmt
            .query_map([], |row| {
                let c_str: String = row.get(12)?;
                Ok(Invoice {
                    id: row.get(0)?,
                    invoice_number: row.get(1)?,
                    contact_id: row.get(2)?,
                    contact_name: row.get(3)?,
                    deal_id: row.get(4)?,
                    status: row.get(5)?,
                    issued_date: row.get(6)?,
                    due_date: row.get(7)?,
                    net_cents: row.get(8)?,
                    vat_cents: row.get(9)?,
                    gross_cents: row.get(10)?,
                    items_json: row.get(11)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(invs)
    }

    pub fn update_invoice_status(&self, invoice_id: &str, new_status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE invoices SET status = ?1 WHERE id = ?2", params![new_status, invoice_id])?;
        Ok(())
    }

    pub fn get_cashflow_stats(&self) -> Result<CashflowStats> {
        let conn = self.conn.lock().unwrap();

        let total_contacts: i64 = conn.query_row("SELECT COUNT(*) FROM contacts", [], |r| r.get(0)).unwrap_or(0);
        let active_leads: i64 = conn.query_row("SELECT COUNT(*) FROM contacts WHERE status IN ('lead', 'qualified')", [], |r| r.get(0)).unwrap_or(0);
        let pipeline_volume_cents: i64 = conn.query_row("SELECT COALESCE(SUM(value_cents), 0) FROM deals WHERE stage != 'closed_lost'", [], |r| r.get(0)).unwrap_or(0);
        let paid_revenue_cents: i64 = conn.query_row("SELECT COALESCE(SUM(gross_cents), 0) FROM invoices WHERE status = 'paid'", [], |r| r.get(0)).unwrap_or(0);
        let pending_invoices_cents: i64 = conn.query_row("SELECT COALESCE(SUM(gross_cents), 0) FROM invoices WHERE status IN ('sent', 'draft', 'overdue')", [], |r| r.get(0)).unwrap_or(0);
        let deals_won_count: i64 = conn.query_row("SELECT COUNT(*) FROM deals WHERE stage = 'closed_won'", [], |r| r.get(0)).unwrap_or(0);

        Ok(CashflowStats {
            total_contacts,
            active_leads,
            pipeline_volume_cents,
            paid_revenue_cents,
            pending_invoices_cents,
            deals_won_count,
        })
    }

    // =========================================================================
    // DMS Methods (Document Management)
    // =========================================================================

    pub fn add_document(&self, input: NewDocument) -> Result<DocumentMeta> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let category = input.category.unwrap_or_else(|| "general".to_string());
        let tags = input.tags.unwrap_or_default();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO documents (id, contact_id, invoice_id, title, filename, file_path, file_size_bytes, mime_type, category, tags, extracted_text, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                id,
                input.contact_id,
                input.invoice_id,
                input.title,
                input.filename,
                input.file_path,
                input.file_size_bytes,
                input.mime_type,
                category,
                tags,
                input.extracted_text,
                now.to_rfc3339(),
            ],
        )?;

        Ok(DocumentMeta {
            id,
            contact_id: input.contact_id,
            invoice_id: input.invoice_id,
            title: input.title,
            filename: input.filename,
            file_path: input.file_path,
            file_size_bytes: input.file_size_bytes,
            mime_type: input.mime_type,
            category,
            tags,
            extracted_text: input.extracted_text,
            created_at: now,
        })
    }

    pub fn list_documents(&self) -> Result<Vec<DocumentMeta>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, contact_id, invoice_id, title, filename, file_path, file_size_bytes, mime_type, category, tags, extracted_text, created_at
             FROM documents
             ORDER BY created_at DESC",
        )?;

        let docs = stmt
            .query_map([], |row| {
                let c_str: String = row.get(11)?;
                Ok(DocumentMeta {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    invoice_id: row.get(2)?,
                    title: row.get(3)?,
                    filename: row.get(4)?,
                    file_path: row.get(5)?,
                    file_size_bytes: row.get(6)?,
                    mime_type: row.get(7)?,
                    category: row.get(8)?,
                    tags: row.get(9)?,
                    extracted_text: row.get(10)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(docs)
    }

    // =========================================================================
    // Customization Engine (Dynamic Custom Fields)
    // =========================================================================

    pub fn add_custom_field(&self, input: NewCustomFieldDef) -> Result<CustomFieldDef> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let options_json = input.options.and_then(|opts| serde_json::to_string(&opts).ok());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO custom_fields (id, entity_type, key, label, field_type, options_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, input.entity_type, input.key, input.label, input.field_type, options_json, now.to_rfc3339()],
        )?;

        Ok(CustomFieldDef {
            id,
            entity_type: input.entity_type,
            key: input.key,
            label: input.label,
            field_type: input.field_type,
            options_json,
            created_at: now,
        })
    }

    pub fn list_custom_fields(&self, entity_type: Option<&str>) -> Result<Vec<CustomFieldDef>> {
        let conn = self.conn.lock().unwrap();
        let sql = if let Some(et) = entity_type {
            format!("SELECT id, entity_type, key, label, field_type, options_json, created_at FROM custom_fields WHERE entity_type = '{}' ORDER BY label ASC", et)
        } else {
            "SELECT id, entity_type, key, label, field_type, options_json, created_at FROM custom_fields ORDER BY entity_type, label ASC".to_string()
        };

        let mut stmt = conn.prepare(&sql)?;
        let fields = stmt
            .query_map([], |row| {
                let c_str: String = row.get(6)?;
                Ok(CustomFieldDef {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    key: row.get(2)?,
                    label: row.get(3)?,
                    field_type: row.get(4)?,
                    options_json: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(fields)
    }
}
