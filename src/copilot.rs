use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

use crate::mcp::McpServer;
use crate::models::{CopilotRequest, CopilotResponse};

pub struct CopilotEngine {
    mcp: Arc<McpServer>,
    client: Client,
    ollama_url: String,
    model_name: String,
}

impl CopilotEngine {
    pub fn new(mcp: Arc<McpServer>) -> Self {
        let ollama_url = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
        let model_name = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:1.5b".to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self {
            mcp,
            client,
            ollama_url,
            model_name,
        }
    }

    pub async fn chat(&self, req: CopilotRequest) -> Result<CopilotResponse> {
        let prompt = req.prompt.trim();

        // 1. Try local Ollama Qwen 1.5B instance
        if let Ok(ollama_res) = self.call_ollama(prompt).await {
            return Ok(ollama_res);
        }

        // 2. Intelligent local fallback engine if Ollama is not yet started
        self.heuristic_fallback(prompt)
    }

    async fn call_ollama(&self, user_prompt: &str) -> Result<CopilotResponse> {
        let endpoint = format!("{}/api/generate", self.ollama_url);
        let tools_schema = self.mcp.get_tools_definition();

        let system_instruction = format!(
            "Du bist der operative KI-Assistent 'Conrad Copilot' für das integrierte Rust CRM, DMS & ERP System.\n\
             Du hast Zugriff auf folgende Werkzeuge (MCP Tools):\n{}\n\n\
             Regeln:\n\
             1. Antworte immer auf Deutsch, professionell, präzise und direkt.\n\
             2. Wenn der Benutzer eine Aktion anfordert (z.B. Kontakt erstellen, Rechnung anlegen, Deal anpassen), gib GENAU ein JSON-Codeblock aus im Format:\n\
             ```json\n{{\"action\": \"TOOL_NAME\", \"args\": {{...}}}}\n```\n\
             3. Wenn es eine Informationsfrage ist, beantworte sie direkt.",
            serde_json::to_string_pretty(&tools_schema).unwrap_or_default()
        );

        let full_prompt = format!("System: {}\n\nUser: {}", system_instruction, user_prompt);

        let body = json!({
            "model": self.model_name,
            "prompt": full_prompt,
            "stream": false,
            "options": {
                "temperature": 0.2
            }
        });

        let res = self.client.post(&endpoint).json(&body).send().await?;
        if !res.status().is_success() {
            anyhow::bail!("Ollama error status: {}", res.status());
        }

        let json_resp: Value = res.json().await?;
        let raw_text = json_resp.get("response").and_then(|v| v.as_str()).unwrap_or("");

        // Check for JSON tool call in response
        let mut actions = Vec::new();
        let mut final_reply = raw_text.to_string();

        if let Some(tool_call) = self.extract_tool_call(raw_text) {
            let tool_name = tool_call.get("action").and_then(|v| v.as_str()).unwrap_or("");
            let tool_args = tool_call.get("args").unwrap_or(&Value::Null);

            match self.mcp.execute_tool(tool_name, tool_args) {
                Ok(result) => {
                    actions.push(format!("Tool '{}' erfolgreich ausgeführt: {}", tool_name, result));
                    final_reply = format!("✅ Aktion ausgeführt: {}\n\n{}", tool_name, serde_json::to_string_pretty(&result).unwrap_or_default());
                }
                Err(err) => {
                    actions.push(format!("Tool-Fehler bei '{}': {}", tool_name, err));
                    final_reply = format!("⚠️ Konnte Aktion nicht vollständig ausführen: {}", err);
                }
            }
        }

        Ok(CopilotResponse {
            reply: final_reply,
            actions_performed: actions,
            model_used: format!("Local Ollama ({})", self.model_name),
        })
    }

    fn extract_tool_call(&self, text: &str) -> Option<Value> {
        if let Some(start) = text.find("```json") {
            let rest = &text[start + 7..];
            if let Some(end) = rest.find("```") {
                let json_str = rest[..end].trim();
                return serde_json::from_str(json_str).ok();
            }
        }
        serde_json::from_str(text.trim()).ok()
    }

    fn heuristic_fallback(&self, prompt: &str) -> Result<CopilotResponse> {
        let p_lower = prompt.to_lowercase();
        let mut actions = Vec::new();

        // 1. Pipeline status
        if p_lower.contains("pipeline") || p_lower.contains("status") || p_lower.contains("umsatz") || p_lower.contains("cashflow") {
            let stats = self.mcp.execute_tool("erp_get_cashflow_stats", &json!({}))?;
            let pipeline_eur = stats.get("pipeline_volume_cents").and_then(|v| v.as_i64()).unwrap_or(0) as f64 / 100.0;
            let revenue_eur = stats.get("paid_revenue_cents").and_then(|v| v.as_i64()).unwrap_or(0) as f64 / 100.0;
            let contacts = stats.get("total_contacts").and_then(|v| v.as_i64()).unwrap_or(0);

            actions.push("Abfrage der Pipeline- und Cashflow-Kennzahlen via MCP".to_string());
            return Ok(CopilotResponse {
                reply: format!(
                    "📊 **Aktueller Betriebs-Überblick:**\n\n\
                     • **Gesamtkontakte:** {}\n\
                     • **Pipeline-Volumen:** {:.2} €\n\
                     • **Realisierter Umsatz:** {:.2} €\n\n\
                     *(Hinweis: Qwen 1.5B läuft im Heuristik-Modus, bis Ollama gestartet wird)*",
                    contacts, pipeline_eur, revenue_eur
                ),
                actions_performed: actions,
                model_used: "Embedded Copilot Engine".to_string(),
            });
        }

        // 2. Erstelle Kontakt
        if p_lower.contains("kontakt") && (p_lower.contains("erstell") || p_lower.contains("neu") || p_lower.contains("anleg")) {
            // Simple extraction
            let name = prompt.replace("Erstelle", "")
                .replace("erstelle", "")
                .replace("Kontakt", "")
                .replace("kontakt", "")
                .replace("einen", "")
                .replace("neuen", "")
                .trim()
                .to_string();

            let clean_name = if name.is_empty() { "Neuer Interessent".to_string() } else { name };
            let _res = self.mcp.execute_tool("crm_create_contact", &json!({
                "name": clean_name,
                "status": "lead"
            }))?;

            actions.push(format!("Kontakt '{}' via MCP angelegt", clean_name));
            return Ok(CopilotResponse {
                reply: format!("✅ Kontakt **{}** wurde erfolgreich im CRM angelegt.", clean_name),
                actions_performed: actions,
                model_used: "Embedded Copilot Engine".to_string(),
            });
        }

        // 3. Suche Kontakte
        if p_lower.contains("suche") || p_lower.contains("finde") {
            let q = prompt.replace("Suche", "").replace("suche", "").replace("nach", "").trim().to_string();
            let res = self.mcp.execute_tool("crm_search_contacts", &json!({ "query": q }))?;
            actions.push(format!("Kontaktsuche nach '{}'", q));

            return Ok(CopilotResponse {
                reply: format!("🔍 Suchergebnisse für **{}**:\n\n```json\n{}\n```", q, serde_json::to_string_pretty(&res).unwrap_or_default()),
                actions_performed: actions,
                model_used: "Embedded Copilot Engine".to_string(),
            });
        }

        // Default friendly response
        Ok(CopilotResponse {
            reply: "Hallo! Ich bin dein lokaler CRM-Copilot (Qwen 1.5B bereit).\n\n\
                 Du kannst mir auf Deutsch Anweisungen geben wie:\n\
                 • Wie steht die Pipeline heute?\n\
                 • Erstelle einen neuen Kontakt Thomas Weber von Malerbetrieb Weber\n\
                 • Erstelle ein Angebot über 2.500 EUR für E-Mail-Triage".to_string(),
            actions_performed: vec![],
            model_used: "Embedded Copilot Engine".to_string(),
        })
    }
}
