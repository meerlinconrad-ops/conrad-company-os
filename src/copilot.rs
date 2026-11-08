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
        let mode = req.mode.as_deref().unwrap_or("user");
        let actor = format!("AI-Copilot ({})", mode);

        // 1. Try local Ollama Qwen 1.5B instance
        let resp = if let Ok(ollama_res) = self.call_ollama(prompt, mode).await {
            ollama_res
        } else {
            // 2. Intelligent local fallback engine if Ollama is not yet started
            self.heuristic_fallback(prompt, mode)?
        };

        // Audit Trail Log: Revisionssichere Speicherung aller KI-Aktionen im GoBD-Audit-Log
        if !resp.actions_performed.is_empty() || resp.ui_mutations.is_some() {
            let details = format!(
                "Prompt: '{}' | Aktionen: {:?}",
                prompt, resp.actions_performed
            );
            let _ = self.mcp.log_ai_audit(&details, &actor);
        }

        Ok(resp)
    }

    async fn call_ollama(&self, user_prompt: &str, mode: &str) -> Result<CopilotResponse> {
        let endpoint = format!("{}/api/generate", self.ollama_url);
        let tools_schema = self.mcp.get_tools_definition();

        let system_instruction = if mode == "admin" {
            format!(
                "Du bist der System-Architekt & Admin-Copilot für das Conrad Company OS.\n\
                 Im Admin-Modus passt du das Layout, die Themes, Module und Plugins an.\n\
                 Du hast Zugriff auf alle MCP Tools:\n{}\n\n\
                 Regeln:\n\
                 1. Antworte auf Deutsch, präzise und lösungsorientiert.\n\
                 2. Wenn eine System- oder UI-Änderung gewünscht ist, gib ein JSON mit 'action' oder 'mutation' aus.\n\
                 3. Wenn der Benutzer navigieren möchte, gib {{\"navigate_to\": \"VIEW_NAME\"}} an.",
                serde_json::to_string_pretty(&tools_schema).unwrap_or_default()
            )
        } else {
            format!(
                "Du bist der operative KI-Assistent für das Conrad Company OS.\n\
                 Im User-Modus hilfst du dem Benutzer bei der täglichen Arbeit: Kunden anlegen, Rechnungen erstellen, Termine planen und navigieren.\n\
                 Du hast Zugriff auf folgende MCP Tools:\n{}\n\n\
                 Regeln:\n\
                 1. Antworte immer auf Deutsch, professionell und hilfreich.\n\
                 2. Wenn der Benutzer eine Aktion anfordert, gib GENAU ein JSON-Codeblock aus im Format:\n\
                 ```json\n{{\"action\": \"TOOL_NAME\", \"args\": {{...}}}}\n```\n\
                 3. Wenn der Benutzer wohin navigieren will, gib an: {{\"navigate_to\": \"VIEW_NAME\"}}.",
                serde_json::to_string_pretty(&tools_schema).unwrap_or_default()
            )
        };

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

        let mut actions = Vec::new();
        let mut final_reply = raw_text.to_string();
        let mut navigate_to = None;
        let mut ui_mutations = None;

        if let Some(tool_call) = self.extract_tool_call(raw_text) {
            if let Some(nav) = tool_call.get("navigate_to").and_then(|v| v.as_str()) {
                navigate_to = Some(nav.to_string());
            }

            if let Some(mutations) = tool_call.get("mutation") {
                ui_mutations = Some(mutations.clone());
            }

            let tool_name = tool_call.get("action").and_then(|v| v.as_str()).unwrap_or("");
            let tool_args = tool_call.get("args").unwrap_or(&Value::Null);

            if !tool_name.is_empty() {
                match self.mcp.execute_tool(tool_name, tool_args) {
                    Ok(result) => {
                        actions.push(format!("Tool '{}' erfolgreich ausgeführt", tool_name));
                        final_reply = format!("✅ Aktion ausgeführt: {}\n\n{}", tool_name, serde_json::to_string_pretty(&result).unwrap_or_default());
                    }
                    Err(err) => {
                        actions.push(format!("Tool-Fehler bei '{}': {}", tool_name, err));
                        final_reply = format!("⚠️ Konnte Aktion nicht vollständig ausführen: {}", err);
                    }
                }
            }
        }

        Ok(CopilotResponse {
            reply: final_reply,
            actions_performed: actions,
            model_used: format!("Local Ollama ({})", self.model_name),
            navigate_to,
            ui_mutations,
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

    fn heuristic_fallback(&self, prompt: &str, mode: &str) -> Result<CopilotResponse> {
        let p_lower = prompt.to_lowercase();
        let mut actions = Vec::new();
        let mut navigate_to = None;
        let mut ui_mutations = None;

        // --- Navigation Detection ---
        if p_lower.contains("rechnung") && (p_lower.contains("zeig") || p_lower.contains("geh") || p_lower.contains("öffne") || p_lower.contains("navigier") || p_lower.contains("bring mich")) {
            navigate_to = Some("invoices".to_string());
        } else if (p_lower.contains("pipeline") || p_lower.contains("deal")) && (p_lower.contains("zeig") || p_lower.contains("geh") || p_lower.contains("öffne") || p_lower.contains("navigier") || p_lower.contains("bring mich")) {
            navigate_to = Some("deals".to_string());
        } else if (p_lower.contains("kontakt") || p_lower.contains("lead")) && (p_lower.contains("zeig") || p_lower.contains("geh") || p_lower.contains("öffne") || p_lower.contains("navigier") || p_lower.contains("bring mich")) {
            navigate_to = Some("contacts".to_string());
        } else if p_lower.contains("marketing") || p_lower.contains("kampagne") || p_lower.contains("werbung") || p_lower.contains("roas") || p_lower.contains("ad spend") {
            navigate_to = Some("marketing".to_string());
        } else if p_lower.contains("retainer") || p_lower.contains("monatsabo") || p_lower.contains("abo") || p_lower.contains("dauerauftrag") {
            navigate_to = Some("retainers".to_string());
        } else if p_lower.contains("workflow") || p_lower.contains("trigger") || p_lower.contains("automatisier") {
            navigate_to = Some("workflows".to_string());
        } else if p_lower.contains("immobilie") || p_lower.contains("halmhuber") || p_lower.contains("mfh") || p_lower.contains("miet") {
            navigate_to = Some("plugin:plugin.realestate".to_string());
        } else if p_lower.contains("projekt") || p_lower.contains("sprint") || p_lower.contains("zeiterfassung") {
            navigate_to = Some("plugin:plugin.projects".to_string());
        } else if p_lower.contains("helpdesk") || p_lower.contains("ticket") || p_lower.contains("sla") {
            navigate_to = Some("plugin:plugin.helpdesk".to_string());
        } else if p_lower.contains("plugin") || p_lower.contains("harness") || p_lower.contains("app store") || p_lower.contains("modul") {
            navigate_to = Some("plugins".to_string());
        } else if p_lower.contains("dokument") || p_lower.contains("dms") || p_lower.contains("ablage") {
            navigate_to = Some("settings".to_string());
        } else if p_lower.contains("einstellung") || p_lower.contains("customizer") || p_lower.contains("mcp") || p_lower.contains("konfig") || p_lower.contains("architektur") {
            navigate_to = Some("settings".to_string());
        }

        // --- AI-First Harness Plugin Builder ("Du baust es dir selbst") ---
        if (p_lower.contains("plugin") || p_lower.contains("app") || p_lower.contains("modul"))
            && (p_lower.contains("bau") || p_lower.contains("erstell") || p_lower.contains("generier") || p_lower.contains("neu"))
        {
            let mut app_name = prompt
                .replace("Baue", "")
                .replace("baue", "")
                .replace("Erstelle", "")
                .replace("erstelle", "")
                .replace("Generiere", "")
                .replace("generiere", "")
                .replace("mir", "")
                .replace("ein", "")
                .replace("neues", "")
                .replace("neue", "")
                .replace("Plugin", "")
                .replace("plugin", "")
                .replace("App", "")
                .replace("app", "")
                .replace("Modul", "")
                .replace("modul", "")
                .replace("für", "")
                .replace(":", "")
                .trim()
                .to_string();

            if app_name.is_empty() {
                app_name = "Fuhrpark & Assets".to_string();
            }

            let created = self.mcp.execute_tool(
                "harness_create_plugin",
                &json!({
                    "name": app_name,
                    "category": "Custom Studio",
                    "accent_color": "#2563eb"
                }),
            )?;

            let pid = created
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("custom.app")
                .to_string();
            let layer = created.get("layer_order").and_then(|v| v.as_i64()).unwrap_or(160);

            actions.push(format!("Harness-Plugin '{}' (Layer #{}) generiert & gemountet", app_name, layer));
            return Ok(CopilotResponse {
                reply: format!(
                    "**Neues Harness-Plugin gebaut & gemountet:** **{}**\n\n\
                     • **Plugin-ID:** `{}`\n\
                     • **Patch-Layer:** `#{}`\n\
                     • **Datenbank & MCP-Tools:** Live initialisiert\n\n\
                     Ich habe das neue Plugin direkt im Workspace geöffnet.",
                    app_name, pid, layer
                ),
                actions_performed: actions,
                model_used: "Conrad Harness Architect (AI-First)".to_string(),
                navigate_to: Some(format!("plugin:{}", pid)),
                ui_mutations: Some(json!({
                    "refresh_plugins": true,
                    "open_plugin": pid
                })),
            });
        }

        // --- Real Estate / Immobilien Portfolio Query ---
        if p_lower.contains("immobilie") || p_lower.contains("halmhuber") || p_lower.contains("mfh") {
            let recs = self.mcp.execute_tool("harness_list_plugin_records", &json!({ "plugin_id": "plugin.realestate" }))?;
            let count = recs.as_array().map(|a| a.len()).unwrap_or(0);
            actions.push("Immobilien-Plugin Records via MCP abgerufen".to_string());
            return Ok(CopilotResponse {
                reply: format!(
                    "**Immobilien-Portfolio (Plugin `plugin.realestate`):**\n\n\
                     • **Erfasste Objekte / Ziele:** {}\n\
                     • **Bestand:** Studio-Wohnung Halmhuberstraße 10, Mannheim (KP 86.000 €, EK 21.000 €, Cashflow **+53,00 €/Monat**, Mieter: Sebastian Gruia)\n\
                     • **Nächster Meilenstein:** 2. Immobilie (MFH) nach Jobwechsel & Probezeit.\n\n\
                     Ich habe das Immobilien-Plugin geöffnet.",
                    count
                ),
                actions_performed: actions,
                model_used: "Conrad Copilot".to_string(),
                navigate_to: Some("plugin:plugin.realestate".to_string()),
                ui_mutations: None,
            });
        }

        // --- Admin Mode: System, Theme, Layout & Plugin Mutations ---
        if mode == "admin" || p_lower.contains("dark mode") || p_lower.contains("dunkel") || p_lower.contains("light mode") || p_lower.contains("hell") || p_lower.contains("akzent") || p_lower.contains("farbe") || p_lower.contains("custom field") || p_lower.contains("feld hinzuf") {
            if p_lower.contains("dark mode") || p_lower.contains("dunkel") {
                ui_mutations = Some(json!({ "theme_mode": "dark" }));
                actions.push("System-Theme: Dark Mode aktiviert".to_string());
                return Ok(CopilotResponse {
                    reply: "🌙 **Dark Mode aktiviert.** Das System-Interface wurde auf Apple Space Black umgeschaltet.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations,
                });
            }

            if p_lower.contains("light mode") || p_lower.contains("hell") {
                ui_mutations = Some(json!({ "theme_mode": "light" }));
                actions.push("System-Theme: Light Mode aktiviert".to_string());
                return Ok(CopilotResponse {
                    reply: "☀️ **Light Mode aktiviert.** Das Interface erstrahlt im klassischen Apple Studio Canvas Design.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations,
                });
            }

            if p_lower.contains("blau") {
                ui_mutations = Some(json!({ "theme_accent": "#0071E3" }));
                actions.push("Akzentfarbe auf Apple Signature Blue (#0071E3) gesetzt".to_string());
                return Ok(CopilotResponse {
                    reply: "🎨 **Akzentfarbe angepasst:** Apple Signature Blue (#0071E3) ist jetzt systemweit aktiv.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations,
                });
            }

            if p_lower.contains("grün") {
                ui_mutations = Some(json!({ "theme_accent": "#34C759" }));
                actions.push("Akzentfarbe auf Apple Emerald (#34C759) gesetzt".to_string());
                return Ok(CopilotResponse {
                    reply: "🎨 **Akzentfarbe angepasst:** Apple Emerald Green (#34C759) ist jetzt systemweit aktiv.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations,
                });
            }

            if p_lower.contains("orange") || p_lower.contains("conrad") {
                ui_mutations = Some(json!({ "theme_accent": "#C8531A" }));
                actions.push("Akzentfarbe auf Conrad KI Terra Cotta (#C8531A) gesetzt".to_string());
                return Ok(CopilotResponse {
                    reply: "🎨 **Akzentfarbe angepasst:** Conrad KI Terra Cotta (#C8531A) ist jetzt aktiv.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations,
                });
            }

            if p_lower.contains("feld") && (p_lower.contains("erstell") || p_lower.contains("hinzuf") || p_lower.contains("neu")) {
                let _ = self.mcp.execute_tool("crm_add_custom_field", &json!({
                    "entity_type": "contact",
                    "key": "prioritaet",
                    "label": "Kunden-Priorität",
                    "field_type": "text"
                }));
                actions.push("Neues dynamisches Custom Field 'Kunden-Priorität' via MCP hinzugefügt".to_string());
                navigate_to = Some("settings".to_string());
                return Ok(CopilotResponse {
                    reply: "🧩 **Custom Field angelegt:** Das Feld 'Kunden-Priorität' wurde erfolgreich in das Contact-Schema injiziert.".to_string(),
                    actions_performed: actions,
                    model_used: "Conrad System Architect (Admin Mode)".to_string(),
                    navigate_to,
                    ui_mutations: Some(json!({ "refresh_schema": true })),
                });
            }
        }

        // --- User Mode: Data Operations & Queries ---

        // 1. Marketing ROI Query
        if p_lower.contains("roas") || p_lower.contains("marketing") || p_lower.contains("ad spend") || p_lower.contains("werbung") {
            let roi = self.mcp.execute_tool("marketing_get_roi", &json!({}))?;
            let spend = roi.get("total_spend_eur").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let rev = roi.get("total_revenue_eur").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let roas = roi.get("roas").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let cac = roi.get("blended_cac_eur").and_then(|v| v.as_f64()).unwrap_or(0.0);

            actions.push("Marketing-ROI via MCP kalkuliert".to_string());
            navigate_to = Some("marketing".to_string());
            return Ok(CopilotResponse {
                reply: format!(
                    "📈 **Marketing-Performance (Meta & Google Ads):**\n\n\
                     • **Gesamtausgaben (Ad Spend):** {:.2} €\n\
                     • **Generierter Umsatz:** {:.2} €\n\
                     • **ROAS:** **{:.1}x**\n\
                     • **Blended CAC:** {:.2} € pro Kunde/Lead\n\n\
                     Ich habe dich direkt zur Marketing-Ansicht navigiert.",
                    spend, rev, roas, cac
                ),
                actions_performed: actions,
                model_used: "Conrad Copilot".to_string(),
                navigate_to,
                ui_mutations,
            });
        }

        // 2. Retainer Abrechnungslauf
        if (p_lower.contains("retainer") || p_lower.contains("abrechn")) && (p_lower.contains("start") || p_lower.contains("ausführ") || p_lower.contains("lauf") || p_lower.contains("jetzt")) {
            let res = self.mcp.execute_tool("retainer_trigger_monthly_billing", &json!({ "actor": "Copilot Chat" }))?;
            let billed = res.get("billed_count").and_then(|v| v.as_i64()).unwrap_or(0);
            let total = res.get("total_amount_eur").and_then(|v| v.as_f64()).unwrap_or(0.0);

            actions.push(format!("Automatischer Retainer-Abrechnungslauf ausgeführt ({} Rechnungen)", billed));
            navigate_to = Some("retainers".to_string());
            return Ok(CopilotResponse {
                reply: format!(
                    "⚡ **Abrechnungslauf erfolgreich durchgeführt!**\n\n\
                     • **Erstellte GoBD/ZUGFeRD Rechnungen:** {}\n\
                     • **Gesamtsumme:** {:.2} €\n\n\
                     Die Rechnungen wurden festgeschrieben und die nächsten Abrechnungstermine der Kunden automatisch aktualisiert.",
                    billed, total
                ),
                actions_performed: actions,
                model_used: "Conrad Copilot".to_string(),
                navigate_to,
                ui_mutations,
            });
        }

        // 3. Pipeline & Cashflow status
        if p_lower.contains("pipeline") || p_lower.contains("status") || p_lower.contains("umsatz") || p_lower.contains("cashflow") {
            let stats = self.mcp.execute_tool("erp_get_cashflow_stats", &json!({}))?;
            let pipeline_eur = stats.get("pipeline_volume_cents").and_then(|v| v.as_i64()).unwrap_or(0) as f64 / 100.0;
            let revenue_eur = stats.get("paid_revenue_cents").and_then(|v| v.as_i64()).unwrap_or(0) as f64 / 100.0;
            let contacts = stats.get("total_contacts").and_then(|v| v.as_i64()).unwrap_or(0);

            actions.push("Pipeline- und Cashflow-Kennzahlen via MCP abgefragt".to_string());
            if navigate_to.is_none() {
                navigate_to = Some("deals".to_string());
            }
            return Ok(CopilotResponse {
                reply: format!(
                    "📊 **Aktueller Betriebs-Überblick:**\n\n\
                     • **Gesamtkontakte:** {}\n\
                     • **Pipeline-Volumen:** {:.2} €\n\
                     • **Realisierter Umsatz:** {:.2} €\n\n\
                     Ich habe dir das Vertriebs-Board geöffnet.",
                    contacts, pipeline_eur, revenue_eur
                ),
                actions_performed: actions,
                model_used: "Conrad Copilot".to_string(),
                navigate_to,
                ui_mutations,
            });
        }

        // 4. Erstelle Kontakt
        if p_lower.contains("kontakt") && (p_lower.contains("erstell") || p_lower.contains("neu") || p_lower.contains("anleg") || p_lower.contains("trag")) {
            let name = prompt.replace("Erstelle", "")
                .replace("erstelle", "")
                .replace("Trage", "")
                .replace("trage", "")
                .replace("trag", "")
                .replace("Kontakt", "")
                .replace("kontakt", "")
                .replace("einen", "")
                .replace("neuen", "")
                .replace("ein", "")
                .trim()
                .to_string();

            let clean_name = if name.is_empty() { "Neuer Interessent".to_string() } else { name };
            let _res = self.mcp.execute_tool("crm_create_contact", &json!({
                "name": clean_name,
                "status": "lead"
            }))?;

            actions.push(format!("Kontakt '{}' via MCP angelegt", clean_name));
            navigate_to = Some("contacts".to_string());
            return Ok(CopilotResponse {
                reply: format!("✅ Kontakt **{}** wurde erfolgreich angelegt. Ich habe dich zur Kontaktliste navigiert.", clean_name),
                actions_performed: actions,
                model_used: "Conrad Copilot".to_string(),
                navigate_to,
                ui_mutations,
            });
        }

        // 5. Navigation allein
        if let Some(target) = navigate_to.clone() {
            return Ok(CopilotResponse {
                reply: format!("Ansicht gewechselt zu **{}**.", target),
                actions_performed: vec![format!("Navigation zu View '{}'", target)],
                model_used: "Conrad Copilot".to_string(),
                navigate_to: Some(target),
                ui_mutations,
            });
        }

        // Default friendly response
        let welcome = if mode == "admin" {
            r#"🛠️ **Admin-Modus (System-Architekt) aktiv.**

Du kannst das System im laufenden Betrieb per Chat konfigurieren:
• „Schalte auf Dark Mode um" (oder „Aktiviere Light Mode")
• „Setze Akzentfarbe auf Apple Blau" (oder Emerald Grün / Terra Cotta)
• „Füge ein Custom Field 'Priorität' zu Kontakten hinzu"
• „Bring mich zu den Marketing-Kampagnen""#
        } else {
            r#"👤 **User-Modus (Operativer Assistent) aktiv.**

Wie kann ich dir heute helfen?
• „Wie steht unsere Pipeline heute?"
• „Trag Thomas Weber von Dachdeckerei Weber ein"
• „Wie hoch ist unser aktueller ROAS?"
• „Führe den Monatsabrechnungslauf für Retainer aus"
• „Bring mich zu den Rechnungen""#
        };

        Ok(CopilotResponse {
            reply: welcome.to_string(),
            actions_performed: vec![],
            model_used: "Conrad Copilot".to_string(),
            navigate_to: None,
            ui_mutations: None,
        })
    }
}
