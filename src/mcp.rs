#![allow(dead_code)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::Database;
use crate::models::{
    InvoiceItem, NewContact, NewCustomFieldDef, NewDeal, NewInvoice,
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
                let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("Gast");
                let email = args.get("email").and_then(|v| v.as_str()).unwrap_or("");
                let start_time = args.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
                let notes = args.get("notes").and_then(|v| v.as_str()).unwrap_or("");

                // Create or match contact
                let contact = self.db.create_contact(NewContact {
                    name: name.to_string(),
                    email: Some(email.to_string()),
                    phone: None,
                    company: None,
                    status: Some("lead".to_string()),
                    tags: Some("Cal.com Booking".to_string()),
                })?;

                // Log appointment interaction
                self.db.add_interaction(
                    &contact.id,
                    "meeting",
                    "inbound",
                    Some("Terminbuchung via Conrad OS"),
                    &format!("Termin am {}: {}", start_time, notes),
                )?;

                Ok(json!({
                    "status": "confirmed",
                    "contact_id": contact.id,
                    "contact_name": contact.name,
                    "start_time": start_time,
                    "message": "Termin erfolgreich im Rust-Kalender gebucht und Kunden-Timeline aktualisiert."
                }))
            }
            _ => Err(anyhow::anyhow!("Unbekanntes Tool: {}", name)),
        }
    }
}
