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
    BookingSlot, CashflowStats, Contact, CustomFieldDef, DatevExportConfig, Deal, DocumentMeta,
    EmailInboundPayload, FollowUpTask, FunnelLeadPayload, FunnelLeadResponse, GobdAuditEntry,
    HarnessPlugin, HubSpotImportResult, InteractionMessage, Invoice, InvoiceItem,
    MarketingCampaign, MarketingRoiSummary, NewBooking, NewContact, NewCustomFieldDef, NewDeal,
    NewDocument, NewFollowUpTask, NewHarnessPlugin, NewInvoice, NewMarketingCampaign,
    NewPluginRecord, NewProductService, NewRetainerSubscription, NewWorkflowRule,
    PluginFieldSchema, PluginRecord, ProductService, RetainerBillingRunResult,
    RetainerSubscription, UpdateContactPayload, UpdatePluginRecord, WhatsAppWebhookPayload,
    WorkflowRule,
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

            CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
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

            CREATE TABLE IF NOT EXISTS marketing_campaigns (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                platform TEXT NOT NULL,
                spend_cents INTEGER NOT NULL DEFAULT 0,
                clicks INTEGER NOT NULL DEFAULT 0,
                leads INTEGER NOT NULL DEFAULT 0,
                customers INTEGER NOT NULL DEFAULT 0,
                revenue_cents INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'active',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS retainer_subscriptions (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                plan_name TEXT NOT NULL,
                amount_cents INTEGER NOT NULL DEFAULT 0,
                vat_rate_percent REAL NOT NULL DEFAULT 19.0,
                interval_months INTEGER NOT NULL DEFAULT 1,
                next_billing_date TEXT NOT NULL,
                active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE RESTRICT
            );

            CREATE TABLE IF NOT EXISTS bookings (
                id TEXT PRIMARY KEY,
                slug TEXT NOT NULL,
                contact_id TEXT,
                guest_name TEXT NOT NULL,
                guest_email TEXT NOT NULL,
                guest_phone TEXT,
                title TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT NOT NULL,
                timezone TEXT NOT NULL DEFAULT 'Europe/Berlin',
                status TEXT NOT NULL DEFAULT 'confirmed',
                notes TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS workflow_rules (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                trigger_type TEXT NOT NULL,
                trigger_filter TEXT NOT NULL,
                action_type TEXT NOT NULL,
                delay_days INTEGER NOT NULL DEFAULT 3,
                template_subject TEXT,
                template_body TEXT NOT NULL,
                active INTEGER NOT NULL DEFAULT 1,
                executions_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS follow_up_tasks (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                deal_id TEXT,
                title TEXT NOT NULL,
                due_date TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open',
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS harness_plugins (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                slug TEXT UNIQUE NOT NULL,
                category TEXT NOT NULL,
                description TEXT NOT NULL,
                version TEXT NOT NULL DEFAULT '1.0.0',
                layer_order INTEGER NOT NULL DEFAULT 100,
                installed INTEGER NOT NULL DEFAULT 1,
                is_core INTEGER NOT NULL DEFAULT 0,
                icon_svg TEXT NOT NULL,
                accent_color TEXT NOT NULL DEFAULT '#2563eb',
                schema_json TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS plugin_records (
                id TEXT PRIMARY KEY,
                plugin_id TEXT NOT NULL,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'Aktiv',
                amount_cents INTEGER NOT NULL DEFAULT 0,
                owner TEXT NOT NULL DEFAULT 'Merlin Conrad',
                data_json TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (plugin_id) REFERENCES harness_plugins(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(name);
            CREATE INDEX IF NOT EXISTS idx_deals_stage ON deals(stage);
            CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
            CREATE INDEX IF NOT EXISTS idx_docs_category ON documents(category);
            CREATE INDEX IF NOT EXISTS idx_gobd_seq ON gobd_audit_log(seq);
            CREATE INDEX IF NOT EXISTS idx_gobd_entity ON gobd_audit_log(entity_type, entity_id);
            CREATE INDEX IF NOT EXISTS idx_campaigns_platform ON marketing_campaigns(platform);
            CREATE INDEX IF NOT EXISTS idx_retainers_next_date ON retainer_subscriptions(next_billing_date);
            CREATE INDEX IF NOT EXISTS idx_bookings_slug ON bookings(slug, start_time);
            CREATE INDEX IF NOT EXISTS idx_plugin_records_pid ON plugin_records(plugin_id);",
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
                "Revisionssicheres GoBD-Audit-Journal initialisiert (Conrad OS Native Rust Kernel)",
                "System",
            );
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO gobd_audit_log (id, seq, timestamp, entity_type, entity_id, action, details, actor, prev_hash, entry_hash)
                 VALUES (?1, 1, ?2, 'system', 'genesis', 'INIT_GOBD_LEDGER', ?3, 'System', ?4, ?5)",
                params![
                    genesis_id,
                    genesis_now,
                    "Revisionssicheres GoBD-Audit-Journal initialisiert (Conrad OS Native Rust Kernel)",
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
                tags: Some("Akustiker, Retainer, KI-Workflows".to_string()),
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
                description: Some("Laufende Betreuung von Conrad OS Automatisierungen, Bugfixes und Modell-Updates".to_string()),
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
                        description: "Betreuung & Hosting Conrad OS Workflow-Engine (November 2026)".to_string(),
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

            // Seed sample retainer subscription
            let _ = self.create_retainer(NewRetainerSubscription {
                contact_id: c1.id.clone(),
                plan_name: "KMU Automatisierungs-Betreuung".to_string(),
                amount_cents: 9900,
                vat_rate_percent: Some(19.0),
                interval_months: Some(1),
                next_billing_date: Some("2026-12-01".to_string()),
            });

            // Seed sample marketing ad campaigns
            let _ = self.record_marketing_campaign(NewMarketingCampaign {
                name: "Meta Ads: Handwerk & KMU Rhein-Neckar".to_string(),
                platform: "meta".to_string(),
                spend_cents: 25000,
                clicks: Some(340),
                leads: Some(18),
                customers: Some(2),
                revenue_cents: Some(198000),
                status: Some("active".to_string()),
            });

            let _ = self.record_marketing_campaign(NewMarketingCampaign {
                name: "Google Search: KI Beratung Mannheim / Heidelberg".to_string(),
                platform: "google".to_string(),
                spend_cents: 18000,
                clicks: Some(145),
                leads: Some(12),
                customers: Some(1),
                revenue_cents: Some(99000),
                status: Some("active".to_string()),
            });

            // Seed system settings
            let _ = self.set_setting("company_name", "Conrad KI");
            let _ = self.set_setting("theme_accent", "#2563EB");
            let _ = self.set_setting("visible_modules", "dashboard,contacts,deals,calendar,invoices,retainers,marketing,workflows,plugins,dms,settings");
            let _ = self.set_setting("active_plugins", "gobd,zugferd,datev,girocode,workflows,scheduler");
        }

        // Ensure Retainer Subscriptions are seeded if empty
        let ret_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM retainer_subscriptions", [], |r| r.get(0)).unwrap_or(0)
        };
        if ret_count == 0 {
            let first_cid: Option<String> = {
                let conn = self.conn.lock().unwrap();
                conn.query_row("SELECT id FROM contacts ORDER BY created_at ASC LIMIT 1", [], |r| r.get(0)).ok()
            };
            if let Some(cid) = first_cid {
                let _ = self.create_retainer(NewRetainerSubscription {
                    contact_id: cid,
                    plan_name: "KMU Automatisierungs-Betreuung (Conrad OS + LLM)".to_string(),
                    amount_cents: 9900,
                    vat_rate_percent: Some(19.0),
                    interval_months: Some(1),
                    next_billing_date: Some("2026-12-01".to_string()),
                });
            }
        }

        // Ensure Marketing Campaigns are seeded if empty
        let mkt_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM marketing_campaigns", [], |r| r.get(0)).unwrap_or(0)
        };
        if mkt_count == 0 {
            let _ = self.record_marketing_campaign(NewMarketingCampaign {
                name: "Meta Ads: Handwerk & KMU Rhein-Neckar".to_string(),
                platform: "meta".to_string(),
                spend_cents: 25000,
                clicks: Some(340),
                leads: Some(18),
                customers: Some(2),
                revenue_cents: Some(198000),
                status: Some("active".to_string()),
            });

            let _ = self.record_marketing_campaign(NewMarketingCampaign {
                name: "Google Search: KI Beratung Mannheim / Heidelberg".to_string(),
                platform: "google".to_string(),
                spend_cents: 18000,
                clicks: Some(145),
                leads: Some(12),
                customers: Some(1),
                revenue_cents: Some(99000),
                status: Some("active".to_string()),
            });
        }

        // Ensure Follow-up Tasks are seeded if empty
        let task_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM follow_up_tasks", [], |r| r.get(0)).unwrap_or(0)
        };
        if task_count == 0 {
            let first_cid: Option<String> = {
                let conn = self.conn.lock().unwrap();
                conn.query_row("SELECT id FROM contacts ORDER BY created_at DESC LIMIT 1", [], |r| r.get(0)).ok()
            };
            if let Some(cid) = first_cid {
                let _ = self.create_follow_up_task(NewFollowUpTask {
                    contact_id: cid,
                    deal_id: None,
                    title: "Follow-up (Angebot): Instagram DM to WhatsApp Lead Engine".to_string(),
                    due_date: Some("2026-11-11".to_string()),
                });
            }
        }

        // Seed default workflow rule if empty
        let wf_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM workflow_rules", [], |r| r.get(0)).unwrap_or(0)
        };
        if wf_count == 0 {
            let _ = self.create_workflow_rule(NewWorkflowRule {
                name: "Angebot Follow-up (3 Tage) + E-Mail-Entwurf".to_string(),
                trigger_type: "deal_stage_changed".to_string(),
                trigger_filter: "proposal".to_string(),
                action_type: "create_task_and_email_draft".to_string(),
                delay_days: Some(3),
                template_subject: Some("Rückfrage zu unserem Angebot — Conrad KI".to_string()),
                template_body: "Hallo {{name}},\n\nich wollte mich kurz erkundigen, ob Sie bereits Gelegenheit hatten, unser Angebot zu prüfen. Lassen Sie uns gerne kurz telefonieren, falls noch Fragen offen sind.\n\nBeste Grüße\nMerlin Conrad".to_string(),
            });
        }

        // Seed Native Conrad OS Documents (DMS Tresor) if empty
        let doc_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM documents", [], |r| r.get(0)).unwrap_or(0)
        };
        if doc_count == 0 {
            let first_cid: Option<String> = {
                let conn = self.conn.lock().unwrap();
                conn.query_row("SELECT id FROM contacts ORDER BY created_at ASC LIMIT 1", [], |r| r.get(0)).ok()
            };
            let _ = self.add_document(NewDocument {
                contact_id: first_cid.clone(),
                invoice_id: None,
                title: "Rahmenvertrag KI-Automatisierung & Retainer (Der Ohrladen)".to_string(),
                filename: "RV_ConradKI_Ohrladen_2026.pdf".to_string(),
                file_path: "/vault/contracts/RV_ConradKI_Ohrladen_2026.pdf".to_string(),
                file_size_bytes: 284_600,
                mime_type: "application/pdf".to_string(),
                category: Some("contract".to_string()),
                tags: Some("GoBD, Rahmenvertrag, Retainer".to_string()),
                extracted_text: Some("Monatlicher Leistungsrahmen: 99,00 EUR netto, Kündigungsfrist 30 Tage zum Monatsende.".to_string()),
            });
            let _ = self.add_document(NewDocument {
                contact_id: None,
                invoice_id: None,
                title: "Kaufvertrag & Teilungserklärung Halmhuberstraße 10, Mannheim".to_string(),
                filename: "Notarurkunde_Halmhuberstr10_Mannheim.pdf".to_string(),
                file_path: "/vault/realestate/Notarurkunde_Halmhuberstr10_Mannheim.pdf".to_string(),
                file_size_bytes: 1_420_000,
                mime_type: "application/pdf".to_string(),
                category: Some("contract".to_string()),
                tags: Some("Immobilie, Mannheim, AfA".to_string()),
                extracted_text: Some("Kaufpreis 86.000 EUR, Eigenkapital 21.000 EUR, Mieter Sebastian Gruia.".to_string()),
            });
        }

        // Seed Native Conrad OS Calendar Bookings if empty
        let book_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM bookings", [], |r| r.get(0)).unwrap_or(0)
        };
        if book_count == 0 {
            let _ = self.book_calendar_slot(NewBooking {
                slug: Some("merlin".to_string()),
                name: "Jens Ohrladen".to_string(),
                email: "info@der-ohrladen.de".to_string(),
                phone: Some("+49 6202 920000".to_string()),
                start_time: "2026-11-10T10:00:00Z".to_string(),
                duration_minutes: Some(30),
                timezone: Some("Europe/Berlin".to_string()),
                notes: Some("Quartals-Review Hörtest-Simulator & Sprach-Assistent".to_string()),
            });
            let _ = self.book_calendar_slot(NewBooking {
                slug: Some("merlin".to_string()),
                name: "Alexandra Imkerin".to_string(),
                email: "alex@schlossgarten-honig.de".to_string(),
                phone: Some("+49 171 4455667".to_string()),
                start_time: "2026-11-12T15:00:00Z".to_string(),
                duration_minutes: Some(30),
                timezone: Some("Europe/Berlin".to_string()),
                notes: Some("Abnahme Angebot: Instagram DM Lead-Pipeline".to_string()),
            });
        }

        // Seed Conrad OS Native Modular Kernel Registry if empty
        let plugin_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM harness_plugins", [], |r| r.get(0)).unwrap_or(0)
        };
        if plugin_count == 0 {
            let now_str = Utc::now().to_rfc3339();
            let default_plugins = vec![
                (
                    "core.crm",
                    "CRM & 360° Kontakte",
                    "contacts",
                    "Sales & CRM",
                    "Zentrales Kontakt- und Lead-Register mit 360°-Timeline, WhatsApp-Webhook und Inline-Editor.",
                    "3.0.0",
                    10,
                    1,
                    1,
                    "users",
                    "#2563eb",
                    r#"[{"key":"company","label":"Unternehmen","field_type":"text"},{"key":"email","label":"E-Mail","field_type":"text"}]"#,
                ),
                (
                    "core.pipeline",
                    "Vertriebs-Pipeline",
                    "deals",
                    "Sales & CRM",
                    "Mehrstufiges Kanban-Board mit gewichteter Abschluss-Prognose und Touch Drag & Drop.",
                    "3.0.0",
                    20,
                    1,
                    1,
                    "kanban",
                    "#4f46e5",
                    r#"[{"key":"stage","label":"Phase","field_type":"badge"},{"key":"value","label":"Volumen","field_type":"currency"}]"#,
                ),
                (
                    "core.invoicing",
                    "GoBD Faktura & ZUGFeRD",
                    "invoices",
                    "Finance & GoBD",
                    "Revisionssichere Rechnungs-Engine mit XRechnung 3.0 XML, DATEV EXTF (SKR03/04) und Storno-Logik.",
                    "3.0.0",
                    30,
                    1,
                    1,
                    "invoice",
                    "#059669",
                    r#"[{"key":"invoice_number","label":"Beleg-Nr.","field_type":"text"},{"key":"gross","label":"Brutto","field_type":"currency"}]"#,
                ),
                (
                    "core.retainers",
                    "Retainer & MRR-Abos",
                    "retainers",
                    "Finance & GoBD",
                    "Wiederkehrende Kundenverträge (39–99 €/Mo.) mit 1-Klick Monatsabrechnungslauf.",
                    "3.0.0",
                    40,
                    1,
                    1,
                    "repeat",
                    "#0d9488",
                    r#"[{"key":"plan_name","label":"Vertrag","field_type":"text"},{"key":"next_billing","label":"Nächster Lauf","field_type":"date"}]"#,
                ),
                (
                    "core.marketing",
                    "Ad Spend & ROAS",
                    "marketing",
                    "Sales & CRM",
                    "Meta- & Google-Ads Kampagnen-Attribution mit Echtzeit-ROAS, Blended CAC und CPC.",
                    "3.0.0",
                    50,
                    1,
                    1,
                    "chart",
                    "#d97706",
                    r#"[{"key":"platform","label":"Kanal","field_type":"badge"},{"key":"roas","label":"ROAS","field_type":"number"}]"#,
                ),
                (
                    "core.workflows",
                    "Workflows & Trigger",
                    "workflows",
                    "AI & Automation",
                    "Ereignisgesteuerte Automatisierung für Aufgaben, Follow-ups und sichere E-Mail-Entwürfe.",
                    "3.0.0",
                    60,
                    1,
                    1,
                    "zap",
                    "#7c3aed",
                    r#"[{"key":"trigger","label":"Trigger","field_type":"text"},{"key":"action","label":"Aktion","field_type":"text"}]"#,
                ),
                (
                    "plugin.projects",
                    "Projekte & Zeiterfassung",
                    "projects",
                    "Operations & ERP",
                    "Integriertes Projekt-, Sprint- und Stunden-Tracking für alle Kundenmandate von Conrad KI.",
                    "3.0.0",
                    110,
                    1,
                    0,
                    "folder",
                    "#0284c7",
                    r#"[{"key":"client","label":"Mandant","field_type":"text"},{"key":"milestone","label":"Meilenstein","field_type":"text"},{"key":"hours","label":"Stunden","field_type":"text"}]"#,
                ),
                (
                    "plugin.realestate",
                    "Immobilien & Mietobjekte",
                    "realestate",
                    "Immobilien & Assets",
                    "Objekt-, Mieter- und Cashflow-Cockpit für Bestandswohnungen und die MFH-Akquise-Pipeline.",
                    "3.0.0",
                    120,
                    1,
                    0,
                    "building",
                    "#16a34a",
                    r#"[{"key":"address","label":"Objektadresse","field_type":"text"},{"key":"tenant","label":"Mieter / Status","field_type":"text"},{"key":"yield_info","label":"Eckdaten","field_type":"text"}]"#,
                ),
                (
                    "plugin.helpdesk",
                    "Service-Desk & SLA-Tickets",
                    "helpdesk",
                    "Operations & ERP",
                    "Kunden-Support, Wartungsanfragen und SLA-Überwachung für betreute KI-Systeme.",
                    "3.0.0",
                    130,
                    1,
                    0,
                    "lifebuoy",
                    "#e11d48",
                    r#"[{"key":"customer","label":"Kunde","field_type":"text"},{"key":"priority","label":"Priorität","field_type":"badge"},{"key":"sla","label":"SLA-Ziel","field_type":"text"}]"#,
                ),
                (
                    "plugin.inventory",
                    "Leistungen & SKR03-Katalog",
                    "inventory",
                    "Finance & GoBD",
                    "Zentrales Artikel-, Lizenz- und Dienstleistungsverzeichnis mit SKR03/04 Erlöskonten-Mapping.",
                    "3.0.0",
                    140,
                    1,
                    0,
                    "box",
                    "#475569",
                    r#"[{"key":"sku","label":"SKU / Code","field_type":"text"},{"key":"unit","label":"Einheit","field_type":"text"},{"key":"skr03","label":"Erlöskonto","field_type":"text"}]"#,
                ),
            ];

            {
                let conn = self.conn.lock().unwrap();
                for p in default_plugins {
                    let _ = conn.execute(
                        "INSERT OR IGNORE INTO harness_plugins (id, name, slug, category, description, version, layer_order, installed, is_core, icon_svg, accent_color, schema_json, created_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                        params![p.0, p.1, p.2, p.3, p.4, p.5, p.6, p.7, p.8, p.9, p.10, p.11, now_str],
                    );
                }
            }

            // Seed realistic records for modular plugins
            let _ = self.create_plugin_record(
                "plugin.projects",
                NewPluginRecord {
                    title: "Der Ohrladen — Hörtest-Simulator v2 & Postfach-Triage".to_string(),
                    status: Some("Aktiv".to_string()),
                    amount_cents: Some(240000),
                    owner: Some("Merlin Conrad".to_string()),
                    data: Some(serde_json::json!({
                        "client": "Jens (Der Ohrladen)",
                        "milestone": "Live-Deployment & Webhook-Sync",
                        "hours": "18.5 Std."
                    })),
                },
            );
            let _ = self.create_plugin_record(
                "plugin.projects",
                NewPluginRecord {
                    title: "Schlossgarten-Imkerei — Instagram DM & Shop-Automation".to_string(),
                    status: Some("In Prüfung".to_string()),
                    amount_cents: Some(95000),
                    owner: Some("Merlin Conrad".to_string()),
                    data: Some(serde_json::json!({
                        "client": "Alex (Schlossgarten-Imkerei)",
                        "milestone": "Freigabe Workflow-Regeln",
                        "hours": "9.0 Std."
                    })),
                },
            );
            let _ = self.create_plugin_record(
                "plugin.projects",
                NewPluginRecord {
                    title: "Leo Club Schwetzingen — Mobile Redesign & Spenden-Portal".to_string(),
                    status: Some("Abgeschlossen".to_string()),
                    amount_cents: Some(0),
                    owner: Some("Merlin Conrad".to_string()),
                    data: Some(serde_json::json!({
                        "client": "Leo Club Schwetzingen",
                        "milestone": "Touch-First Navigation Live",
                        "hours": "6.5 Std."
                    })),
                },
            );

            let _ = self.create_plugin_record(
                "plugin.realestate",
                NewPluginRecord {
                    title: "Studio-Apartment Halmhuberstraße 10, Mannheim".to_string(),
                    status: Some("Vermietet".to_string()),
                    amount_cents: Some(5300), // +53,00 €/Monat positiver Cashflow
                    owner: Some("Merlin & Tamara Conrad".to_string()),
                    data: Some(serde_json::json!({
                        "address": "Halmhuberstraße 10, 68163 Mannheim",
                        "tenant": "Sebastian Gruia (Bestandsmieter)",
                        "yield_info": "Kaufpreis 86.000 € · EK 21.000 € · CF +53 €/Mo."
                    })),
                },
            );
            let _ = self.create_plugin_record(
                "plugin.realestate",
                NewPluginRecord {
                    title: "MFH Akquise-Objekt Rhein-Neckar (6 Wohneinheiten)".to_string(),
                    status: Some("Pipeline 09/2026".to_string()),
                    amount_cents: Some(45000),
                    owner: Some("Merlin & Tamara Conrad".to_string()),
                    data: Some(serde_json::json!({
                        "address": "Schwetzingen / Mannheim Süd",
                        "tenant": "Akquise nach Jobwechsel & Probezeit",
                        "yield_info": "Ziel-Rendite > 6,2 % · 10-Jahres-Plan (3 MFHs)"
                    })),
                },
            );

            let _ = self.create_plugin_record(
                "plugin.helpdesk",
                NewPluginRecord {
                    title: "WhatsApp Business Webhook-Routing für Neukunden".to_string(),
                    status: Some("In Bearbeitung".to_string()),
                    amount_cents: Some(0),
                    owner: Some("Conrad KI Support".to_string()),
                    data: Some(serde_json::json!({
                        "customer": "Der Ohrladen Hörgeräte",
                        "priority": "Hoch",
                        "sla": "< 2 Std. Reaktionszeit"
                    })),
                },
            );
            let _ = self.create_plugin_record(
                "plugin.helpdesk",
                NewPluginRecord {
                    title: "ZUGFeRD 2.2 XML Validierung für Steuerberater-Export".to_string(),
                    status: Some("Gelöst".to_string()),
                    amount_cents: Some(0),
                    owner: Some("Conrad KI Support".to_string()),
                    data: Some(serde_json::json!({
                        "customer": "Schlossgarten Imkerei",
                        "priority": "Normal",
                        "sla": "EN 16931 verifiziert"
                    })),
                },
            );

            let _ = self.create_plugin_record(
                "plugin.inventory",
                NewPluginRecord {
                    title: "Monatlicher KI- & Workflow-Automatisierungs-Retainer".to_string(),
                    status: Some("Aktiv".to_string()),
                    amount_cents: Some(9900),
                    owner: Some("Conrad KI".to_string()),
                    data: Some(serde_json::json!({
                        "sku": "WORKFLOW-RETAINER-M",
                        "unit": "Monatspauschale (zzgl. 19% USt.)",
                        "skr03": "8400 (Erlöse 19% USt.)"
                    })),
                },
            );
            let _ = self.create_plugin_record(
                "plugin.inventory",
                NewPluginRecord {
                    title: "Setup: Lokaler LLM Postfach- & E-Mail-Triage Autopilot".to_string(),
                    status: Some("Aktiv".to_string()),
                    amount_cents: Some(89000),
                    owner: Some("Conrad KI".to_string()),
                    data: Some(serde_json::json!({
                        "sku": "SETUP-POSTFACH-TRIAGE",
                        "unit": "Einmaliges Setup (Pauschal)",
                        "skr03": "8400 (Erlöse 19% USt.)"
                    })),
                },
            );
        }

        // Clean any legacy third-party references in existing DB rows so the OS is 100% unified
        {
            let conn = self.conn.lock().unwrap();
            let _ = conn.execute(
                "UPDATE harness_plugins SET description = 'Integriertes Projekt-, Sprint- und Stunden-Tracking für alle Kundenmandate von Conrad KI.' WHERE id = 'plugin.projects'",
                [],
            );
            let _ = conn.execute(
                "UPDATE harness_plugins SET description = 'Kunden-Support, Wartungsanfragen und SLA-Überwachung für betreute KI-Systeme.' WHERE id = 'plugin.helpdesk'",
                [],
            );
            let _ = conn.execute(
                "UPDATE harness_plugins SET category = 'AI & Automation' WHERE id = 'core.workflows'",
                [],
            );
            let _ = conn.execute(
                "UPDATE plugin_records SET title = REPLACE(title, 'n8n', 'Conrad OS') WHERE title LIKE '%n8n%'",
                [],
            );
            let _ = conn.execute(
                "UPDATE plugin_records SET data_json = REPLACE(data_json, 'Holzhey Consulting GmbH', 'Schlossgarten Imkerei') WHERE data_json LIKE '%Holzhey%'",
                [],
            );
            let _ = conn.execute(
                "UPDATE contacts SET tags = REPLACE(tags, 'n8n', 'KI-Workflows') WHERE tags LIKE '%n8n%'",
                [],
            );
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

        let _ = Self::log_gobd_internal(
            &conn,
            "contact",
            &id,
            "CREATE",
            &format!("Kontakt angelegt: {} ({})", input.name, input.company.as_deref().unwrap_or("Privat")),
            "System",
        );

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

        let _ = Self::log_gobd_internal(
            &conn,
            "deal",
            &id,
            "CREATE",
            &format!("Deal angelegt: {} ({:.2} €, Phase: {})", input.title, input.value_cents as f64 / 100.0, stage),
            "System",
        );

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
        let (contact_id, title): (String, String) = {
            let conn = self.conn.lock().unwrap();
            let info: (String, String) = conn
                .query_row(
                    "SELECT contact_id, title FROM deals WHERE id = ?1",
                    params![deal_id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .unwrap_or_else(|_| ("".to_string(), "Deal".to_string()));

            conn.execute("UPDATE deals SET stage = ?1 WHERE id = ?2", params![new_stage, deal_id])?;

            let _ = Self::log_gobd_internal(
                &conn,
                "deal",
                deal_id,
                "STAGE_CHANGE",
                &format!("Deal '{}' verschoben in Phase '{}'", info.1, new_stage),
                "System",
            );
            info
        };

        // Evaluate automated workflows for this stage transition
        if !contact_id.is_empty() {
            let _ = self.evaluate_deal_stage_workflows(deal_id, &contact_id, &title, new_stage);
        }

        Ok(())
    }

    pub fn add_interaction_with_status(&self, contact_id: &str, channel: &str, direction: &str, subject: Option<&str>, body: &str, status: &str) -> Result<InteractionMessage> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (id, contact_id, channel, direction, subject, body, status, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, contact_id, channel, direction, subject, body, status, now.to_rfc3339()],
        )?;

        Ok(InteractionMessage {
            id,
            contact_id: contact_id.to_string(),
            channel: channel.to_string(),
            direction: direction.to_string(),
            subject: subject.map(String::from),
            body: body.to_string(),
            status: status.to_string(),
            timestamp: now,
        })
    }

    pub fn add_interaction(&self, contact_id: &str, channel: &str, direction: &str, subject: Option<&str>, body: &str) -> Result<InteractionMessage> {
        // Sicherheitsregel: Ausgehende E-Mails werden immer zuerst als 'draft' gespeichert!
        let status = if channel == "email" && direction == "outbound" {
            "draft"
        } else {
            "delivered"
        };
        self.add_interaction_with_status(contact_id, channel, direction, subject, body, status)
    }

    pub fn list_interactions(&self, contact_id: Option<&str>) -> Result<Vec<InteractionMessage>> {
        let conn = self.conn.lock().unwrap();
        let mut msgs = Vec::new();

        if let Some(cid) = contact_id {
            let mut stmt = conn.prepare(
                "SELECT id, contact_id, channel, direction, subject, body, status, timestamp
                 FROM messages WHERE contact_id = ?1 ORDER BY timestamp DESC LIMIT 50",
            )?;
            let rows = stmt.query_map(params![cid], |row| {
                let ts_str: String = row.get(7)?;
                Ok(InteractionMessage {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    channel: row.get(2)?,
                    direction: row.get(3)?,
                    subject: row.get(4)?,
                    body: row.get(5)?,
                    status: row.get(6)?,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&ts_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?;
            msgs.extend(rows.filter_map(Result::ok));
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, contact_id, channel, direction, subject, body, status, timestamp
                 FROM messages ORDER BY timestamp DESC LIMIT 50",
            )?;
            let rows = stmt.query_map([], |row| {
                let ts_str: String = row.get(7)?;
                Ok(InteractionMessage {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    channel: row.get(2)?,
                    direction: row.get(3)?,
                    subject: row.get(4)?,
                    body: row.get(5)?,
                    status: row.get(6)?,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&ts_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?;
            msgs.extend(rows.filter_map(Result::ok));
        }

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
        let mut fields = Vec::new();

        if let Some(et) = entity_type {
            let mut stmt = conn.prepare(
                "SELECT id, entity_type, key, label, field_type, options_json, created_at
                 FROM custom_fields WHERE entity_type = ?1 ORDER BY label ASC",
            )?;
            let rows = stmt.query_map(params![et], |row| {
                let c_str: String = row.get(6)?;
                Ok(CustomFieldDef {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    key: row.get(2)?,
                    label: row.get(3)?,
                    field_type: row.get(4)?,
                    options_json: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?;
            fields.extend(rows.filter_map(Result::ok));
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, entity_type, key, label, field_type, options_json, created_at
                 FROM custom_fields ORDER BY entity_type, label ASC",
            )?;
            let rows = stmt.query_map([], |row| {
                let c_str: String = row.get(6)?;
                Ok(CustomFieldDef {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    key: row.get(2)?,
                    label: row.get(3)?,
                    field_type: row.get(4)?,
                    options_json: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?;
            fields.extend(rows.filter_map(Result::ok));
        }
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

    // =========================================================================
    // Funnel Lead & Autonomous Business Engine
    // =========================================================================

    pub fn ingest_funnel_lead(&self, payload: FunnelLeadPayload) -> Result<FunnelLeadResponse> {
        let email = payload.email.trim().to_lowercase();
        let name = payload.name.unwrap_or_else(|| "Interessent".to_string()).trim().to_string();
        let company = payload.company.map(|c| c.trim().to_string());
        let phone = payload.phone.map(|p| p.trim().to_string());
        let source = payload.source.unwrap_or_else(|| "homepage_rechner_funnel".to_string());
        let notes = payload.notes.unwrap_or_default();

        // 1. Find or create contact
        let contact_id: String = {
            let conn = self.conn.lock().unwrap();
            let existing: Option<String> = conn
                .query_row("SELECT id FROM contacts WHERE LOWER(email) = ?1", [&email], |r| r.get(0))
                .ok();
            existing.unwrap_or_default()
        };

        let contact_id = if contact_id.is_empty() {
            let new_c = self.create_contact(NewContact {
                name: if name.is_empty() { "Interessent".to_string() } else { name.clone() },
                email: Some(email.clone()),
                phone,
                company,
                status: Some("lead".to_string()),
                tags: Some(format!("Funnel, {}", source)),
            })?;
            new_c.id
        } else {
            let conn = self.conn.lock().unwrap();
            let _ = conn.execute(
                "UPDATE contacts SET tags = tags || ', Rechner-Lead', updated_at = ?1 WHERE id = ?2",
                params![Utc::now().to_rfc3339(), contact_id],
            );
            contact_id
        };

        // 2. Automatically create a high-priority Deal in stage "discovery"
        let deal_title = format!("KI-Audit & Hebel-Check: {}", name);
        let deal = self.create_deal(NewDeal {
            contact_id: contact_id.clone(),
            title: deal_title,
            value_cents: 99000,
            currency: Some("EUR".to_string()),
            stage: Some("discovery".to_string()),
            expected_close_date: Some((Utc::now() + Duration::days(14)).format("%Y-%m-%d").to_string()),
        })?;

        // 3. Record interaction note
        let note_body = format!(
            "Neuer Lead über Web-Funnel erfasst.\nQuelle: {}\nDetails: {}",
            source, notes
        );
        let _ = self.add_interaction(&contact_id, "web_funnel", "inbound", Some("Website-Funnel Lead"), &note_body);

        // 4. Log GoBD audit entry
        let _ = self.log_gobd(
            "contact",
            &contact_id,
            "FUNNEL_LEAD_INGEST",
            &format!("Web-Funnel Lead erfasst: {} ({}), Deal: {}", email, name, deal.id),
            "Funnel-API",
        );

        Ok(FunnelLeadResponse {
            success: true,
            contact_id,
            deal_id: deal.id,
            message: "Lead erfolgreich im Conrad OS erfasst und Deal im Pipeline-Board eröffnet.".to_string(),
        })
    }

    // =========================================================================
    // Marketing Ad Campaign Methods
    // =========================================================================

    pub fn list_marketing_campaigns(&self) -> Result<Vec<MarketingCampaign>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, platform, spend_cents, clicks, leads, customers, revenue_cents, status, created_at, updated_at
             FROM marketing_campaigns ORDER BY created_at DESC"
        )?;

        let campaigns = stmt
            .query_map([], |row| {
                let c_str: String = row.get(9)?;
                let u_str: String = row.get(10)?;
                Ok(MarketingCampaign {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    platform: row.get(2)?,
                    spend_cents: row.get(3)?,
                    clicks: row.get(4)?,
                    leads: row.get(5)?,
                    customers: row.get(6)?,
                    revenue_cents: row.get(7)?,
                    status: row.get(8)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&u_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(campaigns)
    }

    pub fn record_marketing_campaign(&self, input: NewMarketingCampaign) -> Result<MarketingCampaign> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let clicks = input.clicks.unwrap_or(0);
        let leads = input.leads.unwrap_or(0);
        let customers = input.customers.unwrap_or(0);
        let revenue_cents = input.revenue_cents.unwrap_or(0);
        let status = input.status.unwrap_or_else(|| "active".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO marketing_campaigns (id, name, platform, spend_cents, clicks, leads, customers, revenue_cents, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                id,
                input.name,
                input.platform,
                input.spend_cents,
                clicks,
                leads,
                customers,
                revenue_cents,
                status,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let _ = Self::log_gobd_internal(
            &conn,
            "marketing",
            &id,
            "CAMPAIGN_RECORDED",
            &format!("Werbekampagne protokolliert: {} ({}, Ausgaben: {:.2} €)", input.name, input.platform, input.spend_cents as f64 / 100.0),
            "System",
        );

        Ok(MarketingCampaign {
            id,
            name: input.name,
            platform: input.platform,
            spend_cents: input.spend_cents,
            clicks,
            leads,
            customers,
            revenue_cents,
            status,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_marketing_roi_summary(&self) -> Result<MarketingRoiSummary> {
        let campaigns = self.list_marketing_campaigns()?;
        let total_spend_cents: i64 = campaigns.iter().map(|c| c.spend_cents).sum();
        let total_clicks: i64 = campaigns.iter().map(|c| c.clicks).sum();
        let total_leads: i64 = campaigns.iter().map(|c| c.leads).sum();
        let total_customers: i64 = campaigns.iter().map(|c| c.customers).sum();
        let total_revenue_cents: i64 = campaigns.iter().map(|c| c.revenue_cents).sum();

        let total_spend_eur = total_spend_cents as f64 / 100.0;
        let total_revenue_eur = total_revenue_cents as f64 / 100.0;

        let blended_cpc_eur = if total_clicks > 0 {
            total_spend_eur / total_clicks as f64
        } else {
            0.0
        };

        let blended_cac_eur = if total_customers > 0 {
            total_spend_eur / total_customers as f64
        } else if total_leads > 0 {
            total_spend_eur / total_leads as f64
        } else {
            0.0
        };

        let roas = if total_spend_eur > 0.0 {
            total_revenue_eur / total_spend_eur
        } else {
            0.0
        };

        Ok(MarketingRoiSummary {
            total_spend_eur,
            total_leads,
            total_customers,
            total_revenue_eur,
            blended_cpc_eur,
            blended_cac_eur,
            roas,
        })
    }

    // =========================================================================
    // Recurring Retainer Billing Methods (ZUGFeRD 2.2 / GoBD)
    // =========================================================================

    pub fn list_retainers(&self) -> Result<Vec<RetainerSubscription>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT r.id, r.contact_id, c.name, r.plan_name, r.amount_cents, r.vat_rate_percent, r.interval_months, r.next_billing_date, r.active, r.created_at
             FROM retainer_subscriptions r
             LEFT JOIN contacts c ON r.contact_id = c.id
             ORDER BY r.next_billing_date ASC"
        )?;

        let retainers = stmt
            .query_map([], |row| {
                let c_str: String = row.get(9)?;
                let active_int: i64 = row.get(8)?;
                Ok(RetainerSubscription {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    contact_name: row.get(2)?,
                    plan_name: row.get(3)?,
                    amount_cents: row.get(4)?,
                    vat_rate_percent: row.get(5)?,
                    interval_months: row.get(6)?,
                    next_billing_date: row.get(7)?,
                    active: active_int != 0,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(retainers)
    }

    pub fn create_retainer(&self, input: NewRetainerSubscription) -> Result<RetainerSubscription> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let vat = input.vat_rate_percent.unwrap_or(19.0);
        let interval = input.interval_months.unwrap_or(1);
        let next_date = input.next_billing_date.unwrap_or_else(|| now.format("%Y-%m-%d").to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO retainer_subscriptions (id, contact_id, plan_name, amount_cents, vat_rate_percent, interval_months, next_billing_date, active, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, ?8)",
            params![
                id,
                input.contact_id,
                input.plan_name,
                input.amount_cents,
                vat,
                interval,
                next_date,
                now.to_rfc3339(),
            ],
        )?;

        let contact_name: Option<String> = conn
            .query_row("SELECT name FROM contacts WHERE id = ?1", [&input.contact_id], |r| r.get(0))
            .ok();

        let _ = Self::log_gobd_internal(
            &conn,
            "retainer",
            &id,
            "CREATE_RETAINER",
            &format!("Monats-Abo/Retainer angelegt: {} für Kontakt {} ({:.2} € / Monat)", input.plan_name, input.contact_id, input.amount_cents as f64 / 100.0),
            "System",
        );

        Ok(RetainerSubscription {
            id,
            contact_id: input.contact_id,
            contact_name,
            plan_name: input.plan_name,
            amount_cents: input.amount_cents,
            vat_rate_percent: vat,
            interval_months: interval,
            next_billing_date: next_date,
            active: true,
            created_at: now,
        })
    }

    pub fn trigger_retainer_billing_run(&self, actor: &str) -> Result<RetainerBillingRunResult> {
        let today_str = Utc::now().format("%Y-%m-%d").to_string();
        let retainers = self.list_retainers()?;
        let mut billed_count = 0;
        let mut total_cents: i64 = 0;
        let mut generated_invoices = Vec::new();

        for r in retainers.into_iter().filter(|r| r.active && r.next_billing_date <= today_str) {
            let inv_input = NewInvoice {
                contact_id: r.contact_id.clone(),
                deal_id: None,
                invoice_number: None,
                items: vec![InvoiceItem {
                    description: format!("Monatspauschale: {} (Abrechnungszeitraum {})", r.plan_name, today_str),
                    quantity: 1.0,
                    unit_price_cents: r.amount_cents,
                    total_cents: r.amount_cents,
                }],
                due_days: Some(14),
            };

            let inv = self.create_invoice(inv_input)?;
            let _ = self.update_invoice_status(&inv.id, "sent");

            billed_count += 1;
            total_cents += inv.gross_cents;
            generated_invoices.push(inv.invoice_number.clone());

            let next_billing = match chrono::NaiveDate::parse_from_str(&r.next_billing_date, "%Y-%m-%d") {
                Ok(d) => {
                    let next = d + Duration::days(r.interval_months * 30);
                    next.format("%Y-%m-%d").to_string()
                }
                Err(_) => (Utc::now() + Duration::days(30)).format("%Y-%m-%d").to_string(),
            };

            let conn = self.conn.lock().unwrap();
            let _ = conn.execute(
                "UPDATE retainer_subscriptions SET next_billing_date = ?1 WHERE id = ?2",
                params![next_billing, r.id],
            );
        }

        let total_amount_eur = total_cents as f64 / 100.0;
        let summary = format!(
            "Abrechnungslauf abgeschlossen: {} Rechnungen generiert. Gesamtsumme: {:.2} €",
            billed_count, total_amount_eur
        );

        let _ = self.log_gobd(
            "retainer",
            "billing_batch",
            "RETAINER_BILLING_RUN",
            &summary,
            actor,
        );

        Ok(RetainerBillingRunResult {
            billed_count,
            generated_invoices,
            total_amount_eur,
            summary,
        })
    }

    // =========================================================================
    // System Settings & Dynamic Customizer (AI-First Admin Control)
    // =========================================================================

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let val: Option<String> = conn
            .query_row("SELECT value FROM system_settings WHERE key = ?1", [key], |r| r.get(0))
            .ok();
        Ok(val)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO system_settings (key, value, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, value, now],
        )?;
        Ok(())
    }

    pub fn get_all_settings(&self) -> Result<std::collections::HashMap<String, String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM system_settings")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut map = std::collections::HashMap::new();
        for row in rows.filter_map(Result::ok) {
            map.insert(row.0, row.1);
        }
        Ok(map)
    }

    // =========================================================================
    // Phase 2: HubSpot Parity — WhatsApp, IMAP/Email Sync, Cal.com & Workflows
    // =========================================================================

    pub fn ingest_whatsapp_webhook(&self, payload: WhatsAppWebhookPayload) -> Result<InteractionMessage> {
        let phone = payload.from.trim().to_string();
        let sender_name = payload
            .sender_name
            .unwrap_or_else(|| format!("WhatsApp Lead ({})", phone));

        // 1. Find existing contact by phone
        let existing_id: Option<String> = {
            let conn = self.conn.lock().unwrap();
            conn.query_row(
                "SELECT id FROM contacts WHERE phone = ?1 OR REPLACE(phone, ' ', '') = REPLACE(?1, ' ', '') LIMIT 1",
                params![phone],
                |r| r.get(0),
            )
            .ok()
        };

        let contact_id = match existing_id {
            Some(id) => id,
            None => {
                let c = self.create_contact(NewContact {
                    name: sender_name.clone(),
                    email: None,
                    phone: Some(phone.clone()),
                    company: None,
                    status: Some("lead".to_string()),
                    tags: Some("WhatsApp, Inbound".to_string()),
                })?;
                c.id
            }
        };

        // 2. Log WhatsApp message into contact timeline
        let msg = self.add_interaction_with_status(
            &contact_id,
            "whatsapp",
            "inbound",
            Some("WhatsApp Business Nachricht"),
            &payload.text,
            "received",
        )?;

        let _ = self.log_gobd(
            "contact",
            &contact_id,
            "WHATSAPP_WEBHOOK_INBOUND",
            &format!("WhatsApp Nachricht von {} empfangen: '{}'", phone, payload.text),
            "WhatsApp-Webhook",
        );

        Ok(msg)
    }

    pub fn ingest_inbound_email(&self, payload: EmailInboundPayload) -> Result<InteractionMessage> {
        let email = payload.from_email.trim().to_lowercase();
        let name = payload
            .from_name
            .unwrap_or_else(|| email.split('@').next().unwrap_or("E-Mail Kontakt").to_string());

        let existing_id: Option<String> = {
            let conn = self.conn.lock().unwrap();
            conn.query_row(
                "SELECT id FROM contacts WHERE LOWER(email) = ?1 LIMIT 1",
                params![email],
                |r| r.get(0),
            )
            .ok()
        };

        let contact_id = match existing_id {
            Some(id) => id,
            None => {
                let c = self.create_contact(NewContact {
                    name,
                    email: Some(email.clone()),
                    phone: None,
                    company: None,
                    status: Some("lead".to_string()),
                    tags: Some("IMAP-Sync, E-Mail".to_string()),
                })?;
                c.id
            }
        };

        let msg = self.add_interaction_with_status(
            &contact_id,
            "email",
            "inbound",
            Some(&payload.subject),
            &payload.body,
            "received",
        )?;

        let _ = self.log_gobd(
            "contact",
            &contact_id,
            "EMAIL_SYNC_INBOUND",
            &format!("E-Mail synchronisiert von {}: Betreff '{}'", email, payload.subject),
            "IMAP-Worker",
        );

        Ok(msg)
    }

    pub fn book_calendar_slot(&self, input: NewBooking) -> Result<BookingSlot> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let slug = input.slug.unwrap_or_else(|| "merlin".to_string());
        let duration = input.duration_minutes.unwrap_or(30);
        let tz = input.timezone.unwrap_or_else(|| "Europe/Berlin".to_string());

        let start_dt = chrono::DateTime::parse_from_rfc3339(&input.start_time)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| now + Duration::hours(24));
        let end_dt = start_dt + Duration::minutes(duration);

        let email_clean = input.email.trim().to_lowercase();

        // Match or create contact
        let existing_id: Option<String> = {
            let conn = self.conn.lock().unwrap();
            conn.query_row(
                "SELECT id FROM contacts WHERE LOWER(email) = ?1 LIMIT 1",
                params![email_clean],
                |r| r.get(0),
            )
            .ok()
        };

        let contact_id = match existing_id {
            Some(cid) => cid,
            None => {
                let c = self.create_contact(NewContact {
                    name: input.name.clone(),
                    email: Some(email_clean.clone()),
                    phone: input.phone.clone(),
                    company: None,
                    status: Some("qualified".to_string()),
                    tags: Some("Terminbuchung, Cal.com-Rust".to_string()),
                })?;
                c.id
            }
        };

        let title = format!("Strategiegespräch: {} ↔ Conrad KI", input.name);

        {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO bookings (id, slug, contact_id, guest_name, guest_email, guest_phone, title, start_time, end_time, timezone, status, notes, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'confirmed', ?11, ?12)",
                params![
                    id,
                    slug,
                    contact_id,
                    input.name,
                    email_clean,
                    input.phone,
                    title,
                    start_dt.to_rfc3339(),
                    end_dt.to_rfc3339(),
                    tz,
                    input.notes,
                    now.to_rfc3339(),
                ],
            )?;

            let _ = Self::log_gobd_internal(
                &conn,
                "booking",
                &id,
                "CALENDAR_SLOT_BOOKED",
                &format!("Termin gebucht ({}): {} am {} ({})", slug, input.name, start_dt.format("%d.%m.%Y %H:%M UTC"), tz),
                "Booking-Engine",
            );
        }

        // Timeline interaction + Email Confirmation Draft (per safety rule: never auto-send emails, save as draft!)
        let _ = self.add_interaction_with_status(
            &contact_id,
            "meeting",
            "inbound",
            Some(&title),
            &format!(
                "Termin gebucht über /book/{}\nStart: {} ({})\nNotizen: {}",
                slug,
                start_dt.format("%d.%m.%Y %H:%M"),
                tz,
                input.notes.as_deref().unwrap_or("—")
            ),
            "confirmed",
        );

        let draft_body = format!(
            "Hallo {},\n\nvielen Dank für Ihre Terminbuchung bei Conrad KI.\n\nTermin-Details:\n• Thema: {}\n• Zeitpunkt: {} ({})\n\nDieser Entwurf wurde zur Freigabe bereitgestellt.\n\nBeste Grüße\nMerlin Conrad",
            input.name,
            title,
            start_dt.format("%d.%m.%Y um %H:%M Uhr"),
            tz
        );
        let _ = self.add_interaction_with_status(
            &contact_id,
            "email",
            "outbound",
            Some(&format!("Terminbestätigung: {}", title)),
            &draft_body,
            "draft",
        );

        Ok(BookingSlot {
            id,
            slug,
            contact_id: Some(contact_id),
            guest_name: input.name,
            guest_email: email_clean,
            guest_phone: input.phone,
            title,
            start_time: start_dt,
            end_time: end_dt,
            timezone: tz,
            status: "confirmed".to_string(),
            notes: input.notes,
            created_at: now,
        })
    }

    pub fn list_bookings(&self, slug: Option<&str>) -> Result<Vec<BookingSlot>> {
        let conn = self.conn.lock().unwrap();
        let mut list = Vec::new();
        let sql = if slug.is_some() {
            "SELECT id, slug, contact_id, guest_name, guest_email, guest_phone, title, start_time, end_time, timezone, status, notes, created_at FROM bookings WHERE slug = ?1 ORDER BY start_time ASC"
        } else {
            "SELECT id, slug, contact_id, guest_name, guest_email, guest_phone, title, start_time, end_time, timezone, status, notes, created_at FROM bookings ORDER BY start_time ASC"
        };

        let mut stmt = conn.prepare(sql)?;
        let mapper = |row: &rusqlite::Row| -> rusqlite::Result<BookingSlot> {
            let s_str: String = row.get(7)?;
            let e_str: String = row.get(8)?;
            let c_str: String = row.get(12)?;
            Ok(BookingSlot {
                id: row.get(0)?,
                slug: row.get(1)?,
                contact_id: row.get(2)?,
                guest_name: row.get(3)?,
                guest_email: row.get(4)?,
                guest_phone: row.get(5)?,
                title: row.get(6)?,
                start_time: chrono::DateTime::parse_from_rfc3339(&s_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                end_time: chrono::DateTime::parse_from_rfc3339(&e_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                timezone: row.get(9)?,
                status: row.get(10)?,
                notes: row.get(11)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            })
        };

        if let Some(s) = slug {
            let rows = stmt.query_map(params![s], mapper)?;
            list.extend(rows.filter_map(Result::ok));
        } else {
            let rows = stmt.query_map([], mapper)?;
            list.extend(rows.filter_map(Result::ok));
        }

        Ok(list)
    }

    pub fn create_workflow_rule(&self, input: NewWorkflowRule) -> Result<WorkflowRule> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let delay = input.delay_days.unwrap_or(3);

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO workflow_rules (id, name, trigger_type, trigger_filter, action_type, delay_days, template_subject, template_body, active, executions_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, 0, ?9)",
            params![
                id,
                input.name,
                input.trigger_type,
                input.trigger_filter,
                input.action_type,
                delay,
                input.template_subject,
                input.template_body,
                now.to_rfc3339(),
            ],
        )?;

        Ok(WorkflowRule {
            id,
            name: input.name,
            trigger_type: input.trigger_type,
            trigger_filter: input.trigger_filter,
            action_type: input.action_type,
            delay_days: delay,
            template_subject: input.template_subject,
            template_body: input.template_body,
            active: true,
            executions_count: 0,
            created_at: now,
        })
    }

    pub fn list_workflow_rules(&self) -> Result<Vec<WorkflowRule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, trigger_type, trigger_filter, action_type, delay_days, template_subject, template_body, active, executions_count, created_at
             FROM workflow_rules ORDER BY created_at DESC",
        )?;

        let rules = stmt
            .query_map([], |row| {
                let c_str: String = row.get(10)?;
                let active_int: i64 = row.get(8)?;
                Ok(WorkflowRule {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    trigger_type: row.get(2)?,
                    trigger_filter: row.get(3)?,
                    action_type: row.get(4)?,
                    delay_days: row.get(5)?,
                    template_subject: row.get(6)?,
                    template_body: row.get(7)?,
                    active: active_int != 0,
                    executions_count: row.get(9)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(rules)
    }

    pub fn list_follow_up_tasks(&self) -> Result<Vec<FollowUpTask>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, contact_id, deal_id, title, due_date, status, created_at FROM follow_up_tasks ORDER BY status DESC, due_date ASC",
        )?;
        let tasks = stmt
            .query_map([], |row| {
                let c_str: String = row.get(6)?;
                Ok(FollowUpTask {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    deal_id: row.get(2)?,
                    title: row.get(3)?,
                    due_date: row.get(4)?,
                    status: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(tasks)
    }

    pub fn create_follow_up_task(&self, input: NewFollowUpTask) -> Result<FollowUpTask> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let due_date = input
            .due_date
            .unwrap_or_else(|| (now + Duration::days(3)).format("%Y-%m-%d").to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO follow_up_tasks (id, contact_id, deal_id, title, due_date, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'open', ?6)",
            params![id, input.contact_id, input.deal_id, input.title, due_date, now.to_rfc3339()],
        )?;

        Ok(FollowUpTask {
            id,
            contact_id: input.contact_id,
            deal_id: input.deal_id,
            title: input.title,
            due_date,
            status: "open".to_string(),
            created_at: now,
        })
    }

    pub fn toggle_task_status(&self, task_id: &str) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let current: String = conn
            .query_row("SELECT status FROM follow_up_tasks WHERE id = ?1", params![task_id], |r| r.get(0))
            .unwrap_or_else(|_| "open".to_string());
        let next = if current == "completed" { "open" } else { "completed" };
        conn.execute(
            "UPDATE follow_up_tasks SET status = ?1 WHERE id = ?2",
            params![next, task_id],
        )?;
        Ok(next.to_string())
    }

    pub fn update_contact(&self, id: &str, payload: UpdateContactPayload) -> Result<Contact> {
        let existing = self.get_contact_by_id(id)?;
        let name = payload.name.unwrap_or(existing.name);
        let email = payload.email.or(existing.email);
        let phone = payload.phone.or(existing.phone);
        let company = payload.company.or(existing.company);
        let status = payload.status.unwrap_or(existing.status);
        let tags = payload.tags.unwrap_or(existing.tags);
        let now = Utc::now();

        {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "UPDATE contacts SET name = ?1, email = ?2, phone = ?3, company = ?4, status = ?5, tags = ?6, updated_at = ?7 WHERE id = ?8",
                params![name, email, phone, company, status, tags, now.to_rfc3339(), id],
            )?;
            let _ = Self::log_gobd_internal(
                &conn,
                "contact",
                id,
                "UPDATE",
                &format!("Kontakt aktualisiert: {} ({})", name, status),
                "Operator",
            );
        }

        self.get_contact_by_id(id)
    }

    fn evaluate_deal_stage_workflows(&self, deal_id: &str, contact_id: &str, deal_title: &str, new_stage: &str) -> Result<()> {
        let rules = self.list_workflow_rules()?;
        let contact = self.get_contact_by_id(contact_id).ok();
        let contact_name = contact.as_ref().map(|c| c.name.as_str()).unwrap_or("Kunde");

        for rule in rules.into_iter().filter(|r| r.active && r.trigger_type == "deal_stage_changed" && r.trigger_filter == new_stage) {
            let now = Utc::now();
            let due_date = (now + Duration::days(rule.delay_days)).format("%Y-%m-%d").to_string();
            let task_id = Uuid::new_v4().to_string();
            let task_title = format!("Follow-up ({}): {}", rule.name, deal_title);

            {
                let conn = self.conn.lock().unwrap();
                let _ = conn.execute(
                    "INSERT INTO follow_up_tasks (id, contact_id, deal_id, title, due_date, status, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, 'open', ?6)",
                    params![task_id, contact_id, deal_id, task_title, due_date, now.to_rfc3339()],
                );
                let _ = conn.execute(
                    "UPDATE workflow_rules SET executions_count = executions_count + 1 WHERE id = ?1",
                    params![rule.id],
                );
            }

            // Create email draft (never send automatically!)
            let body_rendered = rule.template_body.replace("{{name}}", contact_name);
            let subj = rule.template_subject.unwrap_or_else(|| format!("Follow-up: {}", deal_title));
            let _ = self.add_interaction_with_status(
                contact_id,
                "email",
                "outbound",
                Some(&subj),
                &body_rendered,
                "draft",
            );

            let _ = self.log_gobd(
                "workflow",
                &rule.id,
                "WORKFLOW_TRIGGERED",
                &format!("Workflow '{}' ausgelöst für Deal '{}': Folgeaufgabe (Fällig: {}) & E-Mail-Entwurf erstellt", rule.name, deal_title, due_date),
                "Automation-Engine",
            );
        }

        Ok(())
    }

    pub fn toggle_workflow_rule(&self, rule_id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let current: i64 = conn
            .query_row("SELECT active FROM workflow_rules WHERE id = ?1", params![rule_id], |r| r.get(0))
            .unwrap_or(1);
        let next = if current != 0 { 0 } else { 1 };
        conn.execute(
            "UPDATE workflow_rules SET active = ?1 WHERE id = ?2",
            params![next, rule_id],
        )?;
        let _ = Self::log_gobd_internal(
            &conn,
            "workflow",
            rule_id,
            "WORKFLOW_TOGGLED",
            &format!("Workflow-Regel {} auf active={}", rule_id, next),
            "Operator",
        );
        Ok(next != 0)
    }

    // =========================================================================
    // Odoo × DeepSeek Harness Plugin Engine ("Everything is a Plugin")
    // =========================================================================

    pub fn list_plugins(&self) -> Result<Vec<HarnessPlugin>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT p.id, p.name, p.slug, p.category, p.description, p.version, p.layer_order, p.installed, p.is_core, p.icon_svg, p.accent_color, p.schema_json, p.created_at,
                    (SELECT COUNT(*) FROM plugin_records r WHERE r.plugin_id = p.id) AS rec_count
             FROM harness_plugins p
             ORDER BY p.layer_order ASC, p.created_at ASC",
        )?;

        let plugins = stmt
            .query_map([], |row| {
                let c_str: String = row.get(12)?;
                let inst_int: i64 = row.get(7)?;
                let core_int: i64 = row.get(8)?;
                Ok(HarnessPlugin {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    category: row.get(3)?,
                    description: row.get(4)?,
                    version: row.get(5)?,
                    layer_order: row.get(6)?,
                    installed: inst_int != 0,
                    is_core: core_int != 0,
                    icon_svg: row.get(9)?,
                    accent_color: row.get(10)?,
                    schema_json: row.get(11)?,
                    record_count: row.get(13)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(plugins)
    }

    pub fn get_plugin_by_id_or_slug(&self, id_or_slug: &str) -> Result<HarnessPlugin> {
        let plugins = self.list_plugins()?;
        plugins
            .into_iter()
            .find(|p| p.id == id_or_slug || p.slug == id_or_slug)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' nicht gefunden", id_or_slug))
    }

    pub fn create_plugin(&self, input: NewHarnessPlugin) -> Result<HarnessPlugin> {
        let now = Utc::now();
        let raw_slug = input.slug.unwrap_or_else(|| {
            input
                .name
                .to_lowercase()
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        });
        let clean_slug = if raw_slug.is_empty() {
            format!("app_{}", now.timestamp_subsec_millis())
        } else {
            raw_slug
        };
        let id = format!("custom.{}", clean_slug);
        let category = input.category.unwrap_or_else(|| "Custom Studio".to_string());
        let description = input.description.unwrap_or_else(|| {
            format!(
                "Eigenes Harness-Plugin '{}' — dynamisch generiert mit eigenem Daten-Schema & MCP-Anbindung.",
                input.name
            )
        });
        let accent = input.accent_color.unwrap_or_else(|| "#2563eb".to_string());

        let fields = input.fields.unwrap_or_else(|| {
            vec![
                PluginFieldSchema {
                    key: "category".to_string(),
                    label: "Kategorie / Typ".to_string(),
                    field_type: "text".to_string(),
                },
                PluginFieldSchema {
                    key: "notes".to_string(),
                    label: "Notizen / Details".to_string(),
                    field_type: "text".to_string(),
                },
            ]
        });
        let schema_json = serde_json::to_string(&fields).unwrap_or_else(|_| "[]".to_string());

        let layer_order: i64 = {
            let conn = self.conn.lock().unwrap();
            let max_layer: i64 = conn
                .query_row("SELECT COALESCE(MAX(layer_order), 150) FROM harness_plugins", [], |r| r.get(0))
                .unwrap_or(150);
            let next_layer = max_layer + 10;

            conn.execute(
                "INSERT INTO harness_plugins (id, name, slug, category, description, version, layer_order, installed, is_core, icon_svg, accent_color, schema_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, '1.0.0', ?6, 1, 0, 'cube', ?7, ?8, ?9)
                 ON CONFLICT(slug) DO UPDATE SET name = excluded.name, description = excluded.description, installed = 1",
                params![
                    id,
                    input.name,
                    clean_slug,
                    category,
                    description,
                    next_layer,
                    accent,
                    schema_json,
                    now.to_rfc3339(),
                ],
            )?;

            let _ = Self::log_gobd_internal(
                &conn,
                "plugin",
                &id,
                "PLUGIN_CREATED",
                &format!("Neues Harness-Plugin erstellt & gemountet: {} (Layer #{})", input.name, next_layer),
                "Harness-Architect",
            );
            next_layer
        };

        // Create an initial starter record so the new plugin table is immediately interactive
        let _ = self.create_plugin_record(
            &id,
            NewPluginRecord {
                title: format!("Erster Eintrag — {}", input.name),
                status: Some("Aktiv".to_string()),
                amount_cents: Some(0),
                owner: Some("Merlin Conrad".to_string()),
                data: Some(serde_json::json!({
                    "category": "Standard",
                    "notes": "Automatisch beim Plugin-Build initialisiert"
                })),
            },
        );

        Ok(HarnessPlugin {
            id,
            name: input.name,
            slug: clean_slug,
            category,
            description,
            version: "1.0.0".to_string(),
            layer_order,
            installed: true,
            is_core: false,
            icon_svg: "cube".to_string(),
            accent_color: accent,
            schema_json,
            record_count: 1,
            created_at: now,
        })
    }

    pub fn toggle_plugin(&self, id_or_slug: &str) -> Result<HarnessPlugin> {
        let existing = self.get_plugin_by_id_or_slug(id_or_slug)?;
        let next_installed = if existing.installed { 0 } else { 1 };
        {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "UPDATE harness_plugins SET installed = ?1 WHERE id = ?2",
                params![next_installed, existing.id],
            )?;
            let _ = Self::log_gobd_internal(
                &conn,
                "plugin",
                &existing.id,
                if next_installed == 1 { "PLUGIN_MOUNTED" } else { "PLUGIN_UNMOUNTED" },
                &format!(
                    "Harness-Plugin '{}' (Layer #{}) Status geändert auf installed={}",
                    existing.name, existing.layer_order, next_installed == 1
                ),
                "Harness-Architect",
            );
        }
        self.get_plugin_by_id_or_slug(&existing.id)
    }

    pub fn list_plugin_records(&self, id_or_slug: &str) -> Result<Vec<PluginRecord>> {
        let plugin = self.get_plugin_by_id_or_slug(id_or_slug)?;
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, plugin_id, title, status, amount_cents, owner, data_json, created_at, updated_at
             FROM plugin_records
             WHERE plugin_id = ?1
             ORDER BY updated_at DESC",
        )?;

        let records = stmt
            .query_map(params![plugin.id], |row| {
                let c_str: String = row.get(7)?;
                let u_str: String = row.get(8)?;
                Ok(PluginRecord {
                    id: row.get(0)?,
                    plugin_id: row.get(1)?,
                    title: row.get(2)?,
                    status: row.get(3)?,
                    amount_cents: row.get(4)?,
                    owner: row.get(5)?,
                    data_json: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&u_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(records)
    }

    pub fn create_plugin_record(&self, id_or_slug: &str, input: NewPluginRecord) -> Result<PluginRecord> {
        let plugin_id = if let Ok(p) = self.get_plugin_by_id_or_slug(id_or_slug) {
            p.id
        } else {
            id_or_slug.to_string()
        };

        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let status = input.status.unwrap_or_else(|| "Aktiv".to_string());
        let amount_cents = input.amount_cents.unwrap_or(0);
        let owner = input.owner.unwrap_or_else(|| "Merlin Conrad".to_string());
        let data_json = input
            .data
            .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| "{}".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO plugin_records (id, plugin_id, title, status, amount_cents, owner, data_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id,
                plugin_id,
                input.title,
                status,
                amount_cents,
                owner,
                data_json,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        let _ = Self::log_gobd_internal(
            &conn,
            "plugin_record",
            &id,
            "CREATE",
            &format!("Plugin-Datensatz in '{}' angelegt: {}", plugin_id, input.title),
            &owner,
        );

        Ok(PluginRecord {
            id,
            plugin_id,
            title: input.title,
            status,
            amount_cents,
            owner,
            data_json,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_plugin_record(&self, record_id: &str, input: UpdatePluginRecord) -> Result<PluginRecord> {
        let now = Utc::now();
        let conn = self.conn.lock().unwrap();
        let existing: PluginRecord = conn.query_row(
            "SELECT id, plugin_id, title, status, amount_cents, owner, data_json, created_at, updated_at
             FROM plugin_records WHERE id = ?1",
            params![record_id],
            |row| {
                let c_str: String = row.get(7)?;
                let u_str: String = row.get(8)?;
                Ok(PluginRecord {
                    id: row.get(0)?,
                    plugin_id: row.get(1)?,
                    title: row.get(2)?,
                    status: row.get(3)?,
                    amount_cents: row.get(4)?,
                    owner: row.get(5)?,
                    data_json: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&c_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&u_str)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            },
        )?;

        let title = input.title.unwrap_or(existing.title);
        let status = input.status.unwrap_or(existing.status);
        let amount_cents = input.amount_cents.unwrap_or(existing.amount_cents);
        let owner = input.owner.unwrap_or(existing.owner);
        let data_json = match input.data {
            Some(v) => serde_json::to_string(&v).unwrap_or(existing.data_json),
            None => existing.data_json,
        };

        conn.execute(
            "UPDATE plugin_records SET title = ?1, status = ?2, amount_cents = ?3, owner = ?4, data_json = ?5, updated_at = ?6 WHERE id = ?7",
            params![title, status, amount_cents, owner, data_json, now.to_rfc3339(), record_id],
        )?;

        let _ = Self::log_gobd_internal(
            &conn,
            "plugin_record",
            record_id,
            "UPDATE",
            &format!("Plugin-Datensatz aktualisiert: {} (Status: {})", title, status),
            &owner,
        );

        Ok(PluginRecord {
            id: existing.id,
            plugin_id: existing.plugin_id,
            title,
            status,
            amount_cents,
            owner,
            data_json,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    pub fn delete_plugin_record(&self, record_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM plugin_records WHERE id = ?1", params![record_id])?;
        let _ = Self::log_gobd_internal(
            &conn,
            "plugin_record",
            record_id,
            "DELETE",
            &format!("Plugin-Datensatz {} gelöscht", record_id),
            "Operator",
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gobd_hash_chain_and_workflow_engine() -> Result<()> {
        let db = Database::new(":memory:")?;

        // 1. Verify initial seeded GoBD chain
        let (valid, count, _) = db.verify_gobd_integrity()?;
        assert!(valid);
        assert!(count >= 1);

        // 2. Test WhatsApp Business Webhook ingestion
        let wa_msg = db.ingest_whatsapp_webhook(WhatsAppWebhookPayload {
            from: "+49 171 9988776".to_string(),
            sender_name: Some("Bäckerei Schmitt".to_string()),
            text: "Hallo Merlin, wir brauchen Automatisierung für Vorbestellungen.".to_string(),
            message_id: None,
            timestamp: None,
        })?;
        assert_eq!(wa_msg.channel, "whatsapp");
        assert_eq!(wa_msg.direction, "inbound");

        // 3. Test Deal stage transition triggering Workflow (Task + Email Draft)
        let deal = db.create_deal(NewDeal {
            contact_id: wa_msg.contact_id.clone(),
            title: "KI-Bestellannahme Bäckerei".to_string(),
            value_cents: 120000,
            currency: Some("EUR".to_string()),
            stage: Some("discovery".to_string()),
            expected_close_date: None,
        })?;

        db.update_deal_stage(&deal.id, "proposal")?;

        let tasks = db.list_follow_up_tasks()?;
        assert!(!tasks.is_empty());

        let timeline = db.list_interactions(Some(&wa_msg.contact_id))?;
        let has_draft = timeline.iter().any(|m| m.channel == "email" && m.status == "draft");
        assert!(has_draft, "Workflow muss E-Mail als Entwurf ('draft') anlegen!");

        // 4. Test Cal.com Native Booking Slot
        let slot = db.book_calendar_slot(NewBooking {
            slug: Some("merlin".to_string()),
            name: "Dr. Markus Weber".to_string(),
            email: "weber@zahnarzt-mannheim.de".to_string(),
            phone: None,
            start_time: "2026-11-12T10:00:00Z".to_string(),
            duration_minutes: Some(30),
            timezone: Some("Europe/Berlin".to_string()),
            notes: Some("Praxis-Automatisierung".to_string()),
        })?;
        assert_eq!(slot.status, "confirmed");

        // 5. Re-verify GoBD cryptographic ledger after all mutations
        let (valid_after, count_after, _) = db.verify_gobd_integrity()?;
        assert!(valid_after);
        assert!(count_after > count);

        // 6. Test Odoo × DeepSeek Harness Plugin Engine ("Everything is a Plugin")
        let plugins = db.list_plugins()?;
        assert!(plugins.len() >= 10, "Mindestens 10 Harness-Plugins müssen vorinstalliert sein");

        let custom_plugin = db.create_plugin(NewHarnessPlugin {
            name: "Fuhrpark & Leasing".to_string(),
            slug: Some("fuhrpark".to_string()),
            category: Some("Operations & ERP".to_string()),
            description: Some("Tesla & Firmenwagen Verwaltung".to_string()),
            accent_color: Some("#0284c7".to_string()),
            fields: None,
        })?;
        assert_eq!(custom_plugin.slug, "fuhrpark");
        assert!(custom_plugin.installed);

        let rec = db.create_plugin_record(
            &custom_plugin.id,
            NewPluginRecord {
                title: "Tesla Model Y Long Range — MA-MC 2026".to_string(),
                status: Some("Aktiv".to_string()),
                amount_cents: Some(54900),
                owner: Some("Merlin Conrad".to_string()),
                data: Some(serde_json::json!({"category": "Elektrofahrzeug", "notes": "Pendeln Mannheim"})),
            },
        )?;
        assert_eq!(rec.title, "Tesla Model Y Long Range — MA-MC 2026");

        let updated_rec = db.update_plugin_record(
            &rec.id,
            UpdatePluginRecord {
                title: None,
                status: Some("Geprüft".to_string()),
                amount_cents: None,
                owner: None,
                data: None,
            },
        )?;
        assert_eq!(updated_rec.status, "Geprüft");

        let (valid_final, _, _) = db.verify_gobd_integrity()?;
        assert!(valid_final);

        Ok(())
    }
}
