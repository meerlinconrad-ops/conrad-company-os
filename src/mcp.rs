#![allow(dead_code)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::Database;
use crate::models::{NewContact, NewDeal};

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

            "tools/list" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "tools": [
                        {
                            "name": "crm_search_contacts",
                            "description": "Search contacts in Conrad CRM by name, email, phone, or company",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "query": { "type": "string", "description": "Search keyword" }
                                },
                                "required": ["query"]
                            }
                        },
                        {
                            "name": "crm_create_contact",
                            "description": "Create a new lead or customer contact in Conrad CRM",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": { "type": "string" },
                                    "email": { "type": "string" },
                                    "phone": { "type": "string" },
                                    "company": { "type": "string" },
                                    "status": { "type": "string", "enum": ["lead", "qualified", "customer"] }
                                },
                                "required": ["name"]
                            }
                        },
                        {
                            "name": "crm_create_deal",
                            "description": "Create a sales deal linked to a contact",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "contact_id": { "type": "string" },
                                    "title": { "type": "string" },
                                    "value_cents": { "type": "integer" },
                                    "currency": { "type": "string", "default": "EUR" },
                                    "stage": { "type": "string", "enum": ["discovery", "proposal", "negotiation", "closed_won", "closed_lost"] }
                                },
                                "required": ["contact_id", "title", "value_cents"]
                            }
                        },
                        {
                            "name": "crm_send_interaction",
                            "description": "Log an email or WhatsApp interaction into the CRM timeline",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "contact_id": { "type": "string" },
                                    "channel": { "type": "string", "enum": ["email", "whatsapp", "call"] },
                                    "direction": { "type": "string", "enum": ["inbound", "outbound"] },
                                    "subject": { "type": "string" },
                                    "body": { "type": "string" }
                                },
                                "required": ["contact_id", "channel", "direction", "body"]
                            }
                        },
                        {
                            "name": "crm_list_pipeline",
                            "description": "Retrieve all active deals and pipeline volume summary",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                })),
                error: None,
            },

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

    fn execute_tool(&self, name: &str, args: &Value) -> Result<Value> {
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
            "crm_send_interaction" => {
                let contact_id = args.get("contact_id").and_then(|v| v.as_str()).unwrap_or("");
                let channel = args.get("channel").and_then(|v| v.as_str()).unwrap_or("email");
                let direction = args.get("direction").and_then(|v| v.as_str()).unwrap_or("outbound");
                let subject = args.get("subject").and_then(|v| v.as_str());
                let body = args.get("body").and_then(|v| v.as_str()).unwrap_or("");
                let msg = self.db.add_message(contact_id, channel, direction, subject, body)?;
                Ok(json!(msg))
            }
            "crm_list_pipeline" => {
                let deals = self.db.list_deals()?;
                let total_volume_cents: i64 = deals.iter().map(|d| d.value_cents).sum();
                Ok(json!({
                    "total_deals": deals.len(),
                    "total_pipeline_eur": (total_volume_cents as f64) / 100.0,
                    "deals": deals
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
}
