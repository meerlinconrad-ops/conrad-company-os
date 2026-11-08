#![allow(dead_code)]
use anyhow::Result;
use chrono::{Duration, Utc};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::datev::generate_datev_csv;
use crate::gobd::{calculate_entry_hash, GENESIS_HASH};
use crate::hubspot::{parse_hubspot_contacts_csv, parse_hubspot_deals_csv};
use crate::models::{
    CashflowStats, Contact, CustomFieldDef, DatevExportConfig, Deal, DocumentMeta, GobdAuditEntry,
    HubSpotImportResult, InteractionMessage, Invoice, InvoiceItem, NewContact, NewCustomFieldDef,
    NewDeal, NewDocument, NewInvoice, NewProductService, ProductService,
};
use crate::zugferd::generate_xrechnung_xml;

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

            CREATE TABLE IF NOT EXISTS gobd_audit_log (
                id TEXT PRIMARY KEY,
                seq INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                action TEXT NOT NULL,
                details TEXT NOT NULL,
                actor TEXT NOT NULL,
                prev_hash TEXT NOT NULL,
                entry_hash TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(name);
            CREATE INDEX IF NOT EXISTS idx_deals_stage ON deals(stage);
            CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
            CREATE INDEX IF NOT EXISTS idx_docs_category ON documents(category);
            CREATE INDEX IF NOT EXISTS idx_gobd_seq ON gobd_audit_log(seq);
            CREATE INDEX IF NOT EXISTS idx_gobd_entity ON gobd_audit_log(entity_type, entity_id);",
        )?;
        Ok(())
    }

    fn seed_defaults_if_empty(&self) -> Result<()> {
        // Initialize Genesis block for GoBD audit trail if empty
        let gobd_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM gobd_audit_log", [], |r| r.get(0)).unwrap_or(0)
        };
        if gobd_count == 0 {
            let genesis_id = Uuid::new_v4().to_string();
            let genesis_now = Utc::now().to_rfc3339();
            let genesis_hash = calculate_entry_hash(
                GENESIS_HASH,
                1,
                &genesis_now,
                "system",
                "genesis",
                "INIT_GOBD_LEDGER",
                "Revisionssicheres GoBD-Audit-Journal initialisiert (Holzhey Consulting Company OS)",
                "System",
            );
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO gobd_audit_log (id, seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash)
                 VALUES (?1, 1, ?2, 'system', 'genesis', 'INIT_GOBD_LEDGER', ?3, 'System', ?4, ?5)",
                params![
                    genesis_id,
                    genesis_now,
                    "Revisionssicheres GoBD-Audit-Journal initialisiert (Holzhey Consulting Company OS)",
                    GENESIS_HASH,
                    genesis_hash
                ],
            )?;
        }
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

        // GoBD Audit Log: Record creation
        let _ = Self::log_gobd_internal(
            &conn,
            "invoice",
            &id,
            "CREATE",
            &format!("Rechnung angelegt: {} (Netto: {:.2} €)", invoice_num, net_cents as f64 / 100.0),
            "System",
        );

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

        // Check GoBD immutability constraints
        let (current_status, invoice_num): (String, String) = conn
            .query_row(
                "SELECT status, invoice_number FROM invoices WHERE id = ?1",
                params![invoice_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|e| anyhow::anyhow!("Rechnung {} nicht gefunden: {}", invoice_id, e))?;

        if current_status == "cancelled" {
            return Err(anyhow::anyhow!(
                "GoBD-Sperre: Rechnung {} ist storniert und kann nicht mehr geändert werden.",
                invoice_num
            ));
        }

        if (current_status == "sent" || current_status == "paid") && new_status == "draft" {
            return Err(anyhow::anyhow!(
                "GoBD-Sperre: Festgeschriebene Rechnung {} (Status: '{}') darf nicht auf Entwurf zurückgesetzt werden. Korrekturen erfordern eine Stornorechnung.",
                invoice_num,
                current_status
            ));
        }

        conn.execute("UPDATE invoices SET status = ?1 WHERE id = ?2", params![new_status, invoice_id])?;

        let _ = Self::log_gobd_internal(
            &conn,
            "invoice",
            invoice_id,
            "STATUS_CHANGE",
            &format!("Status von Rechnung {} geändert von '{}' auf '{}'", invoice_num, current_status, new_status),
            "System",
        );

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

    // =========================================================================
    // GoBD Revisionssicheres Audit-Log & Storno-Logik
    // =========================================================================

    fn log_gobd_internal(
        conn: &Connection,
        entity_type: &str,
        entity_id: &str,
        action: &str,
        details: &str,
        actor: &str,
    ) -> Result<GobdAuditEntry> {
        let last: Option<(i64, String)> = conn
            .query_row(
                "SELECT seq, entry_hash FROM gobd_audit_log ORDER BY seq DESC LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();

        let (last_seq, prev_hash) = match last {
            Some((s, h)) => (s, h),
            None => (0, GENESIS_HASH.to_string()),
        };

        let new_seq = last_seq + 1;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let timestamp = now.to_rfc3339();

        let entry_hash = calculate_entry_hash(
            &prev_hash,
            new_seq,
            &timestamp,
            entity_type,
            entity_id,
            action,
            details,
            actor,
        );

        conn.execute(
            "INSERT INTO gobd_audit_log (id, seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![id, new_seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash],
        )?;

        Ok(GobdAuditEntry {
            id,
            seq: new_seq,
            timestamp: now,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            action: action.to_string(),
            details: details.to_string(),
            actor: actor.to_string(),
            prev_hash,
            entry_hash,
        })
    }

    pub fn log_gobd(
        &self,
        entity_type: &str,
        entity_id: &str,
        action: &str,
        details: &str,
        actor: &str,
    ) -> Result<GobdAuditEntry> {
        let conn = self.conn.lock().unwrap();
        Self::log_gobd_internal(&conn, entity_type, entity_id, action, details, actor)
    }

    pub fn verify_gobd_integrity(&self) -> Result<(bool, usize, String)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash
             FROM gobd_audit_log
             ORDER BY seq ASC",
        )?;

        let rows: Vec<GobdAuditEntry> = stmt
            .query_map([], |row| {
                let ts_str: String = row.get(2)?;
                Ok(GobdAuditEntry {
                    id: row.get(0)?,
                    seq: row.get(1)?,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&ts_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    entity_type: row.get(3)?,
                    entity_id: row.get(4)?,
                    action: row.get(5)?,
                    details: row.get(6)?,
                    actor: row.get(7)?,
                    prev_hash: row.get(8)?,
                    entry_hash: row.get(9)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        if rows.is_empty() {
            return Ok((true, 0, "GoBD-Audit-Journal ist leer.".to_string()));
        }

        let mut expected_prev = GENESIS_HASH.to_string();

        for (idx, entry) in rows.iter().enumerate() {
            if idx > 0 && entry.prev_hash != expected_prev {
                return Ok((
                    false,
                    idx,
                    format!(
                        "GoBD-Integritätsfehler bei Seq {}: Vorheriger Hash stimmt nicht überein (Erwartet: {}, Gefunden: {})",
                        entry.seq, expected_prev, entry.prev_hash
                    ),
                ));
            }

            let recomputed = calculate_entry_hash(
                &entry.prev_hash,
                entry.seq,
                &entry.timestamp.to_rfc3339(),
                &entry.entity_type,
                &entry.entity_id,
                &entry.action,
                &entry.details,
                &entry.actor,
            );

            if recomputed != entry.entry_hash {
                return Ok((
                    false,
                    idx,
                    format!(
                        "GoBD-Manipulationsalarm bei Seq {}: Hash stimmt nicht mit Datensatz überein!",
                        entry.seq
                    ),
                ));
            }

            expected_prev = entry.entry_hash.clone();
        }

        Ok((
            true,
            rows.len(),
            format!(
                "GoBD-Konformität bestätigt: Alle {} Journal-Einträge sind kryptografisch intakt und revisionssicher unverändert.",
                rows.len()
            ),
        ))
    }

    pub fn list_gobd_audit_log(&self, limit: usize) -> Result<Vec<GobdAuditEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash
             FROM gobd_audit_log
             ORDER BY seq DESC
             LIMIT ?1",
        )?;

        let entries = stmt
            .query_map(params![limit as i64], |row| {
                let ts_str: String = row.get(2)?;
                Ok(GobdAuditEntry {
                    id: row.get(0)?,
                    seq: row.get(1)?,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&ts_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    entity_type: row.get(3)?,
                    entity_id: row.get(4)?,
                    action: row.get(5)?,
                    details: row.get(6)?,
                    actor: row.get(7)?,
                    prev_hash: row.get(8)?,
                    entry_hash: row.get(9)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(entries)
    }

    pub fn get_invoice_by_id(&self, invoice_id: &str) -> Result<Invoice> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT i.id, i.invoice_number, i.contact_id, c.name, i.deal_id, i.status, i.issued_date, i.due_date, i.net_cents, i.vat_cents, i.gross_cents, i.items_json, i.created_at
             FROM invoices i
             LEFT JOIN contacts c ON i.contact_id = c.id
             WHERE i.id = ?1 OR i.invoice_number = ?1",
            params![invoice_id],
            |row| {
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
            },
        ).map_err(|e| anyhow::anyhow!("Rechnung '{}' nicht gefunden: {}", invoice_id, e))
    }

    pub fn get_contact_by_id(&self, contact_id: &str) -> Result<Contact> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, email, phone, company, status, tags, created_at, updated_at FROM contacts WHERE id = ?1",
            params![contact_id],
            |row| {
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
            },
        ).map_err(|e| anyhow::anyhow!("Kontakt '{}' nicht gefunden: {}", contact_id, e))
    }

    pub fn storno_invoice(&self, invoice_id: &str, reason: &str, actor: &str) -> Result<Invoice> {
        let orig = self.get_invoice_by_id(invoice_id)?;
        if orig.status == "cancelled" {
            return Err(anyhow::anyhow!("Rechnung {} ist bereits storniert.", orig.invoice_number));
        }

        let now = Utc::now();
        let storno_id = Uuid::new_v4().to_string();
        let storno_number = format!("ST-{}", orig.invoice_number);

        let items: Vec<InvoiceItem> = serde_json::from_str(&orig.items_json).unwrap_or_default();
        let storno_items: Vec<InvoiceItem> = items
            .into_iter()
            .map(|mut item| {
                item.description = format!("Stornierung: {}", item.description);
                item.total_cents = -item.total_cents;
                item
            })
            .collect();
        let storno_items_json = serde_json::to_string(&storno_items).unwrap_or_else(|_| "[]".to_string());

        let storno_net = -orig.net_cents;
        let storno_vat = -orig.vat_cents;
        let storno_gross = -orig.gross_cents;

        {
            let conn = self.conn.lock().unwrap();
            conn.execute("UPDATE invoices SET status = 'cancelled' WHERE id = ?1", params![orig.id])?;

            conn.execute(
                "INSERT INTO invoices (id, invoice_number, contact_id, deal_id, status, issued_date, due_date, net_cents, vat_cents, gross_cents, items_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, 'sent', ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    storno_id,
                    storno_number,
                    orig.contact_id,
                    orig.deal_id,
                    now.format("%Y-%m-%d").to_string(),
                    now.format("%Y-%m-%d").to_string(),
                    storno_net,
                    storno_vat,
                    storno_gross,
                    storno_items_json,
                    now.to_rfc3339(),
                ],
            )?;

            Self::log_gobd_internal(
                &conn,
                "invoice",
                &orig.id,
                "STORNO",
                &format!(
                    "Storno durchgeführt: Beleg {} erstellt für {} (Begründung: '{}')",
                    storno_number, orig.invoice_number, reason
                ),
                actor,
            )?;
        }

        Ok(Invoice {
            id: storno_id,
            invoice_number: storno_number,
            contact_id: orig.contact_id,
            contact_name: orig.contact_name,
            deal_id: orig.deal_id,
            status: "sent".to_string(),
            issued_date: now.format("%Y-%m-%d").to_string(),
            due_date: now.format("%Y-%m-%d").to_string(),
            net_cents: storno_net,
            vat_cents: storno_vat,
            gross_cents: storno_gross,
            items_json: storno_items_json,
            created_at: now,
        })
    }

    pub fn get_xrechnung_xml(
        &self,
        invoice_id: &str,
        seller_name: &str,
        vat_id: &str,
        iban: &str,
        bic: &str,
    ) -> Result<String> {
        let inv = self.get_invoice_by_id(invoice_id)?;
        let items: Vec<InvoiceItem> = serde_json::from_str(&inv.items_json).unwrap_or_default();
        let buyer = self.get_contact_by_id(&inv.contact_id).ok();

        let xml = generate_xrechnung_xml(
            &inv,
            &items,
            seller_name,
            vat_id,
            iban,
            bic,
            buyer.as_ref(),
        );

        let _ = self.log_gobd(
            "invoice",
            &inv.id,
            "XRECHNUNG_GENERATED",
            &format!("XRechnung / ZUGFeRD 2.2 XML für {} generiert", inv.invoice_number),
            "System",
        );

        Ok(xml)
    }

    pub fn export_datev_csv(&self, config: DatevExportConfig, company_name: &str) -> Result<String> {
        let invs = self.list_invoices()?;
        let csv = generate_datev_csv(&invs, &config, company_name);

        let _ = self.log_gobd(
            "accounting",
            "datev_export",
            "DATEV_EXPORT",
            &format!("DATEV-Buchungsstapel für Jahr {} (SKR: {}) exportiert", config.year, config.skr),
            "System",
        );

        Ok(csv)
    }

    pub fn import_hubspot_contacts(&self, csv_content: &str) -> Result<HubSpotImportResult> {
        let (parsed_contacts, mut res) = parse_hubspot_contacts_csv(csv_content);
        let mut actual_imported = 0;

        for c in parsed_contacts {
            match self.create_contact(c) {
                Ok(_) => actual_imported += 1,
                Err(e) => {
                    res.errors.push(format!("Fehler beim Anlegen: {}", e));
                    res.skipped_count += 1;
                }
            }
        }

        res.imported_count = actual_imported;
        res.summary = format!(
            "HubSpot-Kontakte erfolgreich importiert: {} angelegt, {} übersprungen.",
            res.imported_count, res.skipped_count
        );

        let _ = self.log_gobd(
            "crm",
            "hubspot_import",
            "HUBSPOT_IMPORT",
            &format!("HubSpot Kontakt-Import: {} Kontakte importiert", actual_imported),
            "System",
        );

        Ok(res)
    }

    pub fn import_hubspot_deals(&self, csv_content: &str, default_contact_id: &str) -> Result<HubSpotImportResult> {
        let (parsed_deals, mut res) = parse_hubspot_deals_csv(csv_content, default_contact_id);
        let mut actual_imported = 0;

        for d in parsed_deals {
            match self.create_deal(d) {
                Ok(_) => actual_imported += 1,
                Err(e) => {
                    res.errors.push(format!("Fehler beim Anlegen Deal: {}", e));
                    res.skipped_count += 1;
                }
            }
        }

        res.imported_count = actual_imported;
        res.summary = format!(
            "HubSpot-Deals erfolgreich importiert: {} Deals angelegt, {} übersprungen.",
            res.imported_count, res.skipped_count
        );

        let _ = self.log_gobd(
            "crm",
            "hubspot_import",
            "HUBSPOT_DEALS_IMPORT",
            &format!("HubSpot Deal-Import: {} Deals importiert", actual_imported),
            "System",
        );

        Ok(res)
    }
}
