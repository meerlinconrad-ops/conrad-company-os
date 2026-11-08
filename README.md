# ⚡ Conrad OS — High-Volume Pure Rust CRM, DMS & ERP with Local Qwen 1.5B

Ein schlüsselfertiges, operatives All-in-One-System für mittelständische Betriebe, Handwerksunternehmen und Agenturen — geschrieben zu **100 % in Rust**. Entwickelt von Grund auf nach Prinzipien der Software-Architektur und betriebswirtschaftlichen Steuerung.

---

## 🏛️ Architektur & Kernprinzipien

```
+-----------------------------------------------------------------------------------+
|                           CONRAD OS ARCHITEKTUR                                   |
+-----------------------------------------------------------------------------------+
|  📱 FRONTEND LAYER (HTML5 / Modern Dark CSS / Single-Page-App im Binary)          |
|  - Betriebs-Cockpit (MRR, Pipeline-Wert, Cashflow, Offene Rechnungen)              |
|  - Kontakte & Leads (CRM Timeline mit E-Mail & WhatsApp Interaktionen)            |
|  - Vertriebs-Pipeline (4-Phasen Kanban: Discovery -> Angebot -> Deal -> Gewonnen)  |
|  - Faktura & ERP (Angebote, Rechnungen mit 19% MwSt., Revisionsablage)            |
|  - DMS Dokumentenablage & Volltext-Indizierung                                    |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|  ⚡ APPLICATION & MCP LAYER (Axum 0.7 + Tokio + Model Context Protocol)           |
|  - POST /mcp : Offener JSON-RPC 2.0 Endpunkt für Claude, Antigravity, Hermes      |
|  - POST /api/copilot/chat : Autonomer Agent-Loop mit lokalem Qwen 2.5 1.5B        |
|  - Dynamische Custom Fields : Schema-Erweiterung zur Laufzeit durch KI            |
+-----------------------------------------+-----------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|  💾 STORAGE & ENGINE (SQLite WAL Mode + Embedded DMS File Store)                  |
|  - Sub-Millisekunden Latenzen (< 0.5 ms pro Query)                                |
|  - Speicherbedarf: ~28 MB RAM im Leerlauf (10x sparsamer als Electron/Node.js)    |
|  - Single Binary Deployment (Keine externen Datenbankserver zwingend nötig)       |
+-----------------------------------------------------------------------------------+
```

---

## 🤖 Integrierter KI-Copilot (Qwen 1.5B via MCP)

Das CRM verbindet sich direkt mit einer lokalen Instanz von **Qwen 2.5 1.5B** (über Ollama). Der Copilot hat volle Autonomie über das System mittels MCP:

### Verfügbare MCP Tools
1. `crm_search_contacts`: Intelligente Volltextsuche über Kontakte, Firmen und Tags.
2. `crm_create_contact`: Erstellt neue Kontakte und Leads.
3. `crm_create_deal`: Erstellt Vertriebsvorgänge und berechnet Pipeline-Volumen.
4. `crm_update_deal_stage`: Verschiebt Deals entlang der Pipeline-Phasen.
5. `erp_create_invoice`: Generiert Rechnungen mit automatischer MwSt.-Berechnung.
6. `erp_get_cashflow_stats`: Liefert sofortige Finanz- und Betriebskennzahlen.
7. `crm_log_interaction`: Dokumentiert E-Mails, WhatsApp-Nachrichten und Telefonate.
8. `crm_add_custom_field`: **Selbst-Anpassung:** Die KI kann auf Zuruf neue Felder im CRM anlegen!

---

## 🚀 1-Klick Installation & Schnellstart

### Bare-Metal / Lokal (Mac & Linux)
```bash
./scripts/install.sh
./target/release/conrad-crm
```

### Docker Compose
```bash
docker compose up -d
```

Öffne im Browser: **`http://localhost:8080`**

---

## 🔒 Lizenz
MIT License — 100 % kommerziell nutzbar, frei modifizierbar.
