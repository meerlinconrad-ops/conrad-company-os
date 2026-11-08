#![allow(dead_code)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::Database;
use crate::models::{
    DatevExportConfig, FunnelLeadPayload, InvoiceItem, NewBooking, NewContact, NewCustomFieldDef,
    NewDeal, NewHarnessPlugin, NewInvoice, NewMarketingCampaign, NewPluginRecord,
    NewRetainerSubscription,
};

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub struct McpServer {
    db: Database,
}

impl McpServer {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn log_ai_audit(&self, details: &str, actor: &str) -> Result<()> {
        let _ = self.db.log_gobd("copilot", "ai_action", "AI_MUTATION", details, actor)?;
        Ok(())
    }

    pub fn get_tools_definition(&self) -> Value {
        json!([
            {
                "name": "crm_search_contacts",
                "description": "Sucht Kontakte im Conrad CRM nach Name, E-Mail, Telefon, Firma oder Tags",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Suchbegriff" }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "crm_create_contact",
                "description": "Erstellt einen neuen Lead oder Kunden im Conrad CRM",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Vollständiger Name" },
                        "email": { "type": "string", "description": "E-Mail-Adresse" },
                        "phone": { "type": "string", "description": "Telefonnummer" },
                        "company": { "type": "string", "description": "Firmenname" },
                        "status": { "type": "string", "enum": ["lead", "qualified", "customer"], "description": "Status" },
                        "tags": { "type": "string", "description": "Komma-separierte Tags" }
                    },
                    "required": ["name"]
                }
            },
            {
                "name": "crm_create_deal",
                "description": "Erstellt einen Verkaufsvorgang (Deal) in der Vertriebs-Pipeline",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": { "type": "string", "description": "ID des Kontakts" },
                        "title": { "type": "string", "description": "Bezeichnung des Deals" },
                        "value_cents": { "type": "integer", "description": "Deal-Wert in Cent (z.B. 120000 für 1.200 €)" },
                        "currency": { "type": "string", "default": "EUR" },
                        "stage": { "type": "string", "enum": ["discovery", "proposal", "negotiation", "closed_won", "closed_lost"] },
                        "expected_close_date": { "type": "string", "description": "Erwartetes Abschlussdatum (YYYY-MM-DD)" }
                    },
                    "required": ["contact_id", "title", "value_cents"]
                }
            },
            {
                "name": "crm_update_deal_stage",
                "description": "Verschiebt einen Deal in eine andere Pipeline-Phase",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "deal_id": { "type": "string" },
                        "new_stage": { "type": "string", "enum": ["discovery", "proposal", "negotiation", "closed_won", "closed_lost"] }
                    },
                    "required": ["deal_id", "new_stage"]
                }
            },
            {
                "name": "erp_create_invoice",
                "description": "Erstellt eine neue Rechnung im ERP-System mit automatischer MwSt.-Berechnung",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": { "type": "string" },
                        "deal_id": { "type": "string" },
                        "invoice_number": { "type": "string" },
                        "description": { "type": "string" },
                        "amount_cents": { "type": "integer", "description": "Nettobetrag in Cent" },
                        "due_days": { "type": "integer", "default": 14 }
                    },
                    "required": ["contact_id", "description", "amount_cents"]
                }
            },
            {
                "name": "erp_get_cashflow_stats",
                "description": "Ruft zentrale betriebswirtschaftliche Kennzahlen ab (Umsatz, offene Rechnungen, Pipeline-Volumen)",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "crm_log_interaction",
                "description": "Protokolliert eine E-Mail, WhatsApp-Nachricht oder Telefonnotiz in der Kunden-Timeline",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": { "type": "string" },
                        "channel": { "type": "string", "enum": ["email", "whatsapp", "call", "meeting", "note"] },
                        "direction": { "type": "string", "enum": ["inbound", "outbound"] },
                        "subject": { "type": "string" },
                        "body": { "type": "string" }
                    },
                    "required": ["contact_id", "channel", "direction", "body"]
                }
            },
            {
                "name": "crm_add_custom_field",
                "description": "Erweitert das Datenmodell des CRMs dynamisch um ein benutzerdefiniertes Feld",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "entity_type": { "type": "string", "enum": ["contact", "deal", "invoice", "document"] },
                        "key": { "type": "string", "description": "Technischer Feldname (z.B. branche)" },
                        "label": { "type": "string", "description": "Anzeige-Label (z.B. Branche)" },
                        "field_type": { "type": "string", "enum": ["text", "number", "select", "boolean"] }
                    },
                    "required": ["entity_type", "key", "label", "field_type"]
                }
            },
            {
                "name": "payment_generate_epc_qr",
                "description": "Erstellt einen europäischen EPC/SEPA-QR-Code (GiroCode) für gebührenfreie 0%-Sofortüberweisung einer Rechnung",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "invoice_number": { "type": "string" },
                        "recipient_name": { "type": "string" },
                        "iban": { "type": "string" },
                        "amount_cents": { "type": "integer" }
                    },
                    "required": ["invoice_number", "iban", "amount_cents"]
                }
            },
            {
                "name": "calendar_book_slot",
                "description": "Bucht einen Beratungstermin direkt in den integrierten Rust-Terminkalender (Cal.com Ersatz)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "email": { "type": "string" },
                        "start_time": { "type": "string", "description": "ISO-8601 Zeitstempel" },
                        "notes": { "type": "string" }
                    },
                    "required": ["name", "email", "start_time"]
                }
            },
            {
                "name": "gobd_verify_audit_log",
                "description": "Überprüft die mathematische Unveränderbarkeit und Integrität der gesamten Transaktionskette gemäß GoBD",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "erp_storno_invoice",
                "description": "Erstellt eine GoBD-konforme Stornorechnung / Gutschrift und sperrt die Originalrechnung unveränderbar",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "invoice_id": { "type": "string", "description": "ID oder Rechnungsnummer" },
                        "reason": { "type": "string", "description": "Begründung für die Stornierung" }
                    },
                    "required": ["invoice_id", "reason"]
                }
            },
            {
                "name": "erp_get_xrechnung_xml",
                "description": "Generiert die gesetzlich vorgeschriebene elektronische Rechnung (XRechnung 3.0 / ZUGFeRD 2.2 nach EN 16931) für eine Rechnung",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "invoice_id": { "type": "string", "description": "ID oder Rechnungsnummer" }
                    },
                    "required": ["invoice_id"]
                }
            },
            {
                "name": "accounting_export_datev",
                "description": "Exportiert alle festgeschriebenen Rechnungen im offiziellen DATEV EXTF-Format (SKR03 / SKR04) für den Steuerberater",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "year": { "type": "integer", "description": "Wirtschaftsjahr (z.B. 2026)" },
                        "skr": { "type": "string", "enum": ["skr03", "skr04"], "default": "skr03" }
                    }
                }
            },
            {
                "name": "crm_import_hubspot",
                "description": "Importiert Kontakte oder Deals aus einem HubSpot CSV Export in das Company OS (Holzhey Migration)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "data_type": { "type": "string", "enum": ["contacts", "deals"] },
                        "csv_content": { "type": "string", "description": "Inhalt der CSV-Datei" }
                    },
                    "required": ["data_type", "csv_content"]
                }
            },
            {
                "name": "consulting_holzhey_savings_report",
                "description": "Erstellt einen Kostenvergleichs- und Einsparungsbericht für Holzhey Consulting (Conrad Company OS vs. HubSpot Enterprise)",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "funnel_ingest_lead",
                "description": "Erfasst einen neuen Web-Funnel Lead, legt Kontakt und Discovery-Deal an und erzeugt GoBD Audit-Eintrag",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "email": { "type": "string", "description": "E-Mail-Adresse" },
                        "name": { "type": "string", "description": "Name / Ansprechpartner" },
                        "phone": { "type": "string", "description": "Telefon / WhatsApp" },
                        "company": { "type": "string", "description": "Firmenname" },
                        "notes": { "type": "string", "description": "Berechnete Stunden / Einsparpotenzial" },
                        "source": { "type": "string", "description": "Funnel-Quelle" }
                    },
                    "required": ["email"]
                }
            },
            {
                "name": "marketing_get_roi",
                "description": "Berechnet den Marketing-ROI, CPC, CAC und ROAS über alle Meta- und Google Ads Kampagnen",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "marketing_record_ad_spend",
                "description": "Protokolliert Ad Spend und Performance-Kennzahlen für eine Kampagne (Meta / Google)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Kampagnenname" },
                        "platform": { "type": "string", "enum": ["meta", "google", "linkedin", "organic"] },
                        "spend_cents": { "type": "integer", "description": "Ausgaben in Cent (z.B. 25000 für 250,00 €)" },
                        "clicks": { "type": "integer" },
                        "leads": { "type": "integer" },
                        "customers": { "type": "integer" },
                        "revenue_cents": { "type": "integer" }
                    },
                    "required": ["name", "platform", "spend_cents"]
                }
            },
            {
                "name": "retainer_list_subscriptions",
                "description": "Listet alle aktiven monatlichen Kunden-Retainer und Abonnements auf",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "retainer_create_subscription",
                "description": "Erstellt ein neues monatliches Abonnement / Retainer für einen Kunden",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "contact_id": { "type": "string" },
                        "plan_name": { "type": "string" },
                        "amount_cents": { "type": "integer", "description": "Monatlicher Betrag in Cent (z.B. 9900 für 99,00 €)" },
                        "vat_rate_percent": { "type": "number", "default": 19.0 },
                        "interval_months": { "type": "integer", "default": 1 },
                        "next_billing_date": { "type": "string", "description": "Erstes Abrechnungsdatum (YYYY-MM-DD)" }
                    },
                    "required": ["contact_id", "plan_name", "amount_cents"]
                }
            },
            {
                "name": "retainer_trigger_monthly_billing",
                "description": "Führt den automatisierten Abrechnungslauf durch: erzeugt GoBD-festgeschriebene Monatsrechnungen mit ZUGFeRD 2.2 XML für fällige Abos",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "actor": { "type": "string", "description": "Name des aufrufenden Agenten oder Nutzers" }
                    }
                }
            },
            {
                "name": "harness_list_plugins",
                "description": "Listet alle im Conrad OS installierten und verfügbaren Odoo/DeepSeek-Harness Plugins & Patch-Layers auf",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "harness_toggle_plugin",
                "description": "Aktiviert (mountet) oder deaktiviert ein Plugin-Layer im Harness-Stack",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "plugin_id": { "type": "string", "description": "Plugin-ID oder Slug (z.B. plugin.projects, plugin.realestate, plugin.helpdesk)" }
                    },
                    "required": ["plugin_id"]
                }
            },
            {
                "name": "harness_create_plugin",
                "description": "Baut zur Laufzeit ein komplett neues Odoo/Harness Plugin mit eigenem Daten-Schema, Tabelle und UI-Ansicht ('Everything is a Plugin')",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Anzeigename des neuen Plugins (z.B. 'Fuhrpark & Leasing')" },
                        "slug": { "type": "string", "description": "Technischer Kurzname (z.B. 'fuhrpark')" },
                        "category": { "type": "string", "description": "Kategorie (z.B. 'Operations & ERP', 'Immobilien & Assets', 'Custom Studio')" },
                        "description": { "type": "string", "description": "Beschreibung des Plugins" },
                        "accent_color": { "type": "string", "description": "Hex-Farbe (z.B. '#0284c7')" }
                    },
                    "required": ["name"]
                }
            },
            {
                "name": "harness_list_plugin_records",
                "description": "Liest alle Einträge eines bestimmten Plugins (z.B. 'plugin.projects', 'plugin.realestate', 'plugin.helpdesk')",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "plugin_id": { "type": "string", "description": "Plugin-ID oder Slug" }
                    },
                    "required": ["plugin_id"]
                }
            },
            {
                "name": "harness_create_plugin_record",
                "description": "Erstellt einen neuen Datensatz innerhalb eines beliebigen Harness-Plugins",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "plugin_id": { "type": "string", "description": "Plugin-ID oder Slug" },
                        "title": { "type": "string", "description": "Titel / Bezeichnung des Eintrags" },
                        "status": { "type": "string", "description": "Status (z.B. 'Aktiv', 'In Prüfung', 'Abgeschlossen')" },
                        "amount_cents": { "type": "integer", "description": "Wert oder Cashflow in Cent" },
                        "owner": { "type": "string", "description": "Verantwortlicher" },
                        "data": { "type": "object", "description": "Zusätzliche Schema-Felder als JSON-Objekt" }
                    },
                    "required": ["plugin_id", "title"]
                }
            }
        ])
    }

    pub fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        let id = req.id;
        match req.method.as_str() {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "conrad-crm-mcp",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            },

            "tools/list" => {
                let tools = self.get_tools_definition();
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({ "tools": tools })),
                    error: None,
                }
            }

            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = params.get("arguments").unwrap_or(&Value::Null);

                match self.execute_tool(tool_name, arguments) {
                    Ok(result) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                                }
                            ]
                        })),
                        error: None,
                    },
                    Err(err) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32603,
                            message: err.to_string(),
                            data: None,
                        }),
                    },
                }
            }

            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", req.method),
                    data: None,
                }),
            },
        }
    }

    pub fn execute_tool(&self, name: &str, args: &Value) -> Result<Value> {
        match name {
            "crm_search_contacts" => {
                let q = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
                let contacts = self.db.search_contacts(q)?;
                Ok(json!(contacts))
            }
            "crm_create_contact" => {
                let input = NewContact {
                    name: args.get("name").and_then(|v| v.as_str()).unwrap_or("Unnamed").to_string(),
                    email: args.get("email").and_then(|v| v.as_str()).map(String::from),
                    phone: args.get("phone").and_then(|v| v.as_str()).map(String::from),
                    company: args.get("company").and_then(|v| v.as_str()).map(String::from),
                    status: args.get("status").and_then(|v| v.as_str()).map(String::from),
                    tags: args.get("tags").and_then(|v| v.as_str()).map(String::from),
                };
                let contact = self.db.create_contact(input)?;
                Ok(json!(contact))
            }
            "crm_create_deal" => {
                let input = NewDeal {
                    contact_id: args.get("contact_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    title: args.get("title").and_then(|v| v.as_str()).unwrap_or("New Deal").to_string(),
                    value_cents: args.get("value_cents").and_then(|v| v.as_i64()).unwrap_or(0),
                    currency: args.get("currency").and_then(|v| v.as_str()).map(String::from),
                    stage: args.get("stage").and_then(|v| v.as_str()).map(String::from),
                    expected_close_date: args.get("expected_close_date").and_then(|v| v.as_str()).map(String::from),
                };
                let deal = self.db.create_deal(input)?;
                Ok(json!(deal))
            }
            "crm_update_deal_stage" => {
                let deal_id = args.get("deal_id").and_then(|v| v.as_str()).unwrap_or("");
                let new_stage = args.get("new_stage").and_then(|v| v.as_str()).unwrap_or("discovery");
                self.db.update_deal_stage(deal_id, new_stage)?;
                Ok(json!({ "status": "updated", "deal_id": deal_id, "new_stage": new_stage }))
            }
            "erp_create_invoice" => {
                let contact_id = args.get("contact_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let deal_id = args.get("deal_id").and_then(|v| v.as_str()).map(String::from);
                let invoice_number = args.get("invoice_number").and_then(|v| v.as_str()).map(String::from);
                let description = args.get("description").and_then(|v| v.as_str()).unwrap_or("Dienstleistung").to_string();
                let amount_cents = args.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                let due_days = args.get("due_days").and_then(|v| v.as_i64());

                let invoice = self.db.create_invoice(NewInvoice {
                    contact_id,
                    deal_id,
                    invoice_number,
                    items: vec![InvoiceItem {
                        description,
                        quantity: 1.0,
                        unit_price_cents: amount_cents,
                        total_cents: amount_cents,
                    }],
                    due_days,
                })?;
                Ok(json!(invoice))
            }
            "erp_get_cashflow_stats" => {
                let stats = self.db.get_cashflow_stats()?;
                Ok(json!(stats))
            }
            "crm_log_interaction" => {
                let contact_id = args.get("contact_id").and_then(|v| v.as_str()).unwrap_or("");
                let channel = args.get("channel").and_then(|v| v.as_str()).unwrap_or("note");
                let direction = args.get("direction").and_then(|v| v.as_str()).unwrap_or("outbound");
                let subject = args.get("subject").and_then(|v| v.as_str());
                let body = args.get("body").and_then(|v| v.as_str()).unwrap_or("");
                let msg = self.db.add_interaction(contact_id, channel, direction, subject, body)?;
                Ok(json!(msg))
            }
            "crm_add_custom_field" => {
                let entity_type = args.get("entity_type").and_then(|v| v.as_str()).unwrap_or("contact").to_string();
                let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let label = args.get("label").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let field_type = args.get("field_type").and_then(|v| v.as_str()).unwrap_or("text").to_string();
                let fdef = self.db.add_custom_field(NewCustomFieldDef {
                    entity_type,
                    key,
                    label,
                    field_type,
                    options: None,
                })?;
                Ok(json!(fdef))
            }
            "payment_generate_epc_qr" => {
                let inv_num = args.get("invoice_number").and_then(|v| v.as_str()).unwrap_or("RE-2026");
                let recipient = args.get("recipient_name").and_then(|v| v.as_str()).unwrap_or("Conrad KI");
                let iban = args.get("iban").and_then(|v| v.as_str()).unwrap_or("");
                let amount_cents = args.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                let amount_eur = (amount_cents as f64) / 100.0;

                // European EPC QR payload specification (GiroCode standard)
                let epc_payload = format!(
                    "BCD\n002\n1\nSCT\n\n{}\n{}\nEUR{:.2}\n\n\n{}\n",
                    recipient, iban, amount_eur, inv_num
                );

                Ok(json!({
                    "status": "ready",
                    "invoice_number": inv_num,
                    "amount_eur": amount_eur,
                    "epc_qr_string": epc_payload,
                    "fee_percent": 0.0,
                    "instruction": "Kunde kann diesen QR-Code mit jeder Banking-App (Sparkasse, DKB, N26, Revolut) gebührenfrei scannen."
                }))
            }
            "calendar_book_slot" => {
                let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("Gast").to_string();
                let email = args.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let start_time = args.get("start_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let notes = args.get("notes").and_then(|v| v.as_str()).map(String::from);

                let slot = self.db.book_calendar_slot(NewBooking {
                    slug: Some("merlin".to_string()),
                    name,
                    email,
                    phone: None,
                    start_time,
                    duration_minutes: Some(30),
                    timezone: Some("Europe/Berlin".to_string()),
                    notes,
                })?;

                Ok(json!({
                    "status": "confirmed",
                    "booking": slot,
                    "message": "Termin erfolgreich im Rust-Kalender gebucht, Kunden-Timeline aktualisiert und E-Mail-Entwurf gespeichert."
                }))
            }
            "gobd_verify_audit_log" => {
                let (valid, count, report) = self.db.verify_gobd_integrity()?;
                Ok(json!({
                    "gobd_compliant": valid,
                    "total_audited_records": count,
                    "audit_report": report
                }))
            }
            "erp_storno_invoice" => {
                let invoice_id = args.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("");
                let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("GoBD-Korrektur");
                let storno = self.db.storno_invoice(invoice_id, reason, "KI-Copilot")?;
                Ok(json!({
                    "status": "storno_created",
                    "storno_invoice": storno,
                    "gobd_note": "Originalrechnung wurde gesperrt und mit neuer Stornorechnung buchhalterisch ausgeglichen."
                }))
            }
            "erp_get_xrechnung_xml" => {
                let invoice_id = args.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("");
                let xml = self.db.get_xrechnung_xml(
                    invoice_id,
                    "Holzhey Consulting GmbH",
                    "DE312345678",
                    "DE02100100100123456789",
                    "PBNKDEFFXXX",
                )?;
                let preview: String = xml.chars().take(400).collect();
                Ok(json!({
                    "standard": "EN 16931 / CII / XRechnung 3.0",
                    "invoice_id": invoice_id,
                    "xml_total_bytes": xml.len(),
                    "xml_preview": preview
                }))
            }
            "accounting_export_datev" => {
                let year = args.get("year").and_then(|v| v.as_i64()).unwrap_or(2026) as i32;
                let skr = args.get("skr").and_then(|v| v.as_str()).unwrap_or("skr03");
                let csv = self.db.export_datev_csv(
                    DatevExportConfig {
                        year,
                        month: None,
                        skr: skr.to_string(),
                        consultant_number: Some(10001),
                        client_number: Some(20002),
                    },
                    "Holzhey Consulting GmbH",
                )?;
                let line_count = csv.lines().count();
                Ok(json!({
                    "status": "exported",
                    "format": "DATEV EXTF 700 / 21",
                    "year": year,
                    "kontenrahmen": skr,
                    "records_exported": line_count.saturating_sub(2),
                    "csv_head": csv.lines().take(4).collect::<Vec<&str>>()
                }))
            }
            "crm_import_hubspot" => {
                let data_type = args.get("data_type").and_then(|v| v.as_str()).unwrap_or("contacts");
                let content = args.get("csv_content").and_then(|v| v.as_str()).unwrap_or("");
                if data_type == "deals" {
                    let res = self.db.import_hubspot_deals(content, "")?;
                    Ok(json!(res))
                } else {
                    let res = self.db.import_hubspot_contacts(content)?;
                    Ok(json!(res))
                }
            }
            "consulting_holzhey_savings_report" => {
                Ok(json!({
                    "client": "Holzhey Consulting GmbH",
                    "previous_crm": "HubSpot Enterprise / Sales Hub",
                    "hubspot_annual_cost_eur": 18400.0,
                    "conrad_os_annual_cost_eur": 120.0,
                    "annual_cashflow_saved_eur": 18280.0,
                    "roi_percentage": 99.3,
                    "de_compliance": {
                        "gobd": "Revisionssichere SHA-256 Audit-Hash-Chain integriert",
                        "e_rechnung": "ZUGFeRD 2.2 & XRechnung 3.0 (EN 16931) ab Werk",
                        "datev": "EXTF Buchungsstapel Export für SKR03 & SKR04",
                        "girocode": "0% SEPA Instant Payment ohne Zahlungsdienstleister-Abzug"
                    }
                }))
            }
            "funnel_ingest_lead" => {
                let payload = FunnelLeadPayload {
                    email: args.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    name: args.get("name").and_then(|v| v.as_str()).map(String::from),
                    phone: args.get("phone").and_then(|v| v.as_str()).map(String::from),
                    company: args.get("company").and_then(|v| v.as_str()).map(String::from),
                    notes: args.get("notes").and_then(|v| v.as_str()).map(String::from),
                    source: args.get("source").and_then(|v| v.as_str()).map(String::from),
                };
                let res = self.db.ingest_funnel_lead(payload)?;
                Ok(json!(res))
            }
            "marketing_get_roi" => {
                let roi = self.db.get_marketing_roi_summary()?;
                Ok(json!(roi))
            }
            "marketing_record_ad_spend" => {
                let input = NewMarketingCampaign {
                    name: args.get("name").and_then(|v| v.as_str()).unwrap_or("Campaign").to_string(),
                    platform: args.get("platform").and_then(|v| v.as_str()).unwrap_or("meta").to_string(),
                    spend_cents: args.get("spend_cents").and_then(|v| v.as_i64()).unwrap_or(0),
                    clicks: args.get("clicks").and_then(|v| v.as_i64()),
                    leads: args.get("leads").and_then(|v| v.as_i64()),
                    customers: args.get("customers").and_then(|v| v.as_i64()),
                    revenue_cents: args.get("revenue_cents").and_then(|v| v.as_i64()),
                    status: args.get("status").and_then(|v| v.as_str()).map(String::from),
                };
                let c = self.db.record_marketing_campaign(input)?;
                Ok(json!(c))
            }
            "retainer_list_subscriptions" => {
                let list = self.db.list_retainers()?;
                Ok(json!(list))
            }
            "retainer_create_subscription" => {
                let input = NewRetainerSubscription {
                    contact_id: args.get("contact_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    plan_name: args.get("plan_name").and_then(|v| v.as_str()).unwrap_or("Retainer").to_string(),
                    amount_cents: args.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0),
                    vat_rate_percent: args.get("vat_rate_percent").and_then(|v| v.as_f64()),
                    interval_months: args.get("interval_months").and_then(|v| v.as_i64()),
                    next_billing_date: args.get("next_billing_date").and_then(|v| v.as_str()).map(String::from),
                };
                let r = self.db.create_retainer(input)?;
                Ok(json!(r))
            }
            "retainer_trigger_monthly_billing" => {
                let actor = args.get("actor").and_then(|v| v.as_str()).unwrap_or("MCP Copilot Agent");
                let res = self.db.trigger_retainer_billing_run(actor)?;
                Ok(json!(res))
            }
            "harness_list_plugins" => {
                let list = self.db.list_plugins()?;
                Ok(json!(list))
            }
            "harness_toggle_plugin" => {
                let pid = args.get("plugin_id").and_then(|v| v.as_str()).unwrap_or("");
                let plugin = self.db.toggle_plugin(pid)?;
                Ok(json!(plugin))
            }
            "harness_create_plugin" => {
                let input = NewHarnessPlugin {
                    name: args.get("name").and_then(|v| v.as_str()).unwrap_or("Neues Plugin").to_string(),
                    slug: args.get("slug").and_then(|v| v.as_str()).map(String::from),
                    category: args.get("category").and_then(|v| v.as_str()).map(String::from),
                    description: args.get("description").and_then(|v| v.as_str()).map(String::from),
                    accent_color: args.get("accent_color").and_then(|v| v.as_str()).map(String::from),
                    fields: None,
                };
                let plugin = self.db.create_plugin(input)?;
                Ok(json!(plugin))
            }
            "harness_list_plugin_records" => {
                let pid = args.get("plugin_id").and_then(|v| v.as_str()).unwrap_or("plugin.projects");
                let records = self.db.list_plugin_records(pid)?;
                Ok(json!(records))
            }
            "harness_create_plugin_record" => {
                let pid = args.get("plugin_id").and_then(|v| v.as_str()).unwrap_or("plugin.projects");
                let input = NewPluginRecord {
                    title: args.get("title").and_then(|v| v.as_str()).unwrap_or("Neuer Eintrag").to_string(),
                    status: args.get("status").and_then(|v| v.as_str()).map(String::from),
                    amount_cents: args.get("amount_cents").and_then(|v| v.as_i64()),
                    owner: args.get("owner").and_then(|v| v.as_str()).map(String::from),
                    data: args.get("data").cloned(),
                };
                let rec = self.db.create_plugin_record(pid, input)?;
                Ok(json!(rec))
            }
            _ => Err(anyhow::anyhow!("Unbekanntes Tool: {}", name)),
        }
    }
}
