pub fn render_app_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Conrad OS — Omarchy-Style Pure Rust CRM, DMS & ERP</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg: #090d16;
            --surface: #101726;
            --surface-elevated: #182238;
            --border: #1e293b;
            --border-glow: rgba(59, 130, 246, 0.4);
            --primary: #3b82f6;
            --primary-hover: #2563eb;
            --accent: #10b981;
            --ai-accent: #a855f7;
            --ai-gradient: linear-gradient(135deg, #a855f7, #3b82f6);
            --danger: #ef4444;
            --warning: #f59e0b;
            --text: #f8fafc;
            --text-secondary: #94a3b8;
            --sidebar-width: 250px;
        }

        * { box-sizing: border-box; margin: 0; padding: 0; -webkit-tap-highlight-color: transparent; }
        body {
            background: var(--bg);
            color: var(--text);
            font-family: 'Plus Jakarta Sans', -apple-system, sans-serif;
            display: flex;
            height: 100vh;
            overflow: hidden;
        }

        /* Omarchy Sidebar */
        .sidebar {
            width: var(--sidebar-width);
            background: var(--surface);
            border-right: 1px solid var(--border);
            display: flex;
            flex-direction: column;
            padding: 18px 14px;
            flex-shrink: 0;
            z-index: 20;
        }

        .brand {
            display: flex;
            align-items: center;
            gap: 12px;
            padding-bottom: 20px;
            border-bottom: 1px solid var(--border);
            margin-bottom: 16px;
        }
        .brand-logo {
            width: 36px;
            height: 36px;
            border-radius: 8px;
            background: var(--ai-gradient);
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            color: #fff;
            font-size: 17px;
            box-shadow: 0 4px 14px rgba(168, 85, 247, 0.35);
        }
        .brand-text { font-size: 15px; font-weight: 700; letter-spacing: -0.02em; }
        .brand-sub { font-size: 10px; color: var(--text-secondary); font-family: 'JetBrains Mono', monospace; }

        .kbd-hint {
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid rgba(255, 255, 255, 0.12);
            padding: 2px 5px;
            border-radius: 4px;
            color: var(--text-secondary);
            margin-left: auto;
        }

        .nav-items { display: flex; flex-direction: column; gap: 4px; flex: 1; }
        .nav-btn {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 10px 12px;
            border-radius: 6px;
            background: transparent;
            color: var(--text-secondary);
            border: none;
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.15s ease;
            text-align: left;
            width: 100%;
        }
        .nav-btn:hover { background: var(--surface-elevated); color: var(--text); }
        .nav-btn.active { background: rgba(59, 130, 246, 0.15); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3); }

        .palette-trigger {
            background: #090e17;
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 9px 12px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            color: var(--text-secondary);
            font-size: 12px;
            cursor: pointer;
            margin-bottom: 16px;
            transition: all 0.15s;
        }
        .palette-trigger:hover { border-color: var(--border-glow); color: var(--text); }

        .ai-status-badge {
            margin-top: auto;
            background: rgba(168, 85, 247, 0.08);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 8px;
            padding: 12px;
            display: flex;
            align-items: center;
            gap: 10px;
            cursor: pointer;
            transition: all 0.2s;
        }
        .ai-status-badge:hover { background: rgba(168, 85, 247, 0.15); }
        .ai-pulse {
            width: 8px;
            height: 8px;
            background: var(--accent);
            border-radius: 50%;
            box-shadow: 0 0 8px var(--accent);
            animation: pulse 2s infinite;
        }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }

        /* Main Workspace */
        .main-content {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow-y: auto;
            position: relative;
        }
        .topbar {
            height: 56px;
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 24px;
            background: rgba(16, 23, 38, 0.85);
            backdrop-filter: blur(12px);
            position: sticky;
            top: 0;
            z-index: 10;
        }
        .view-title { font-size: 16px; font-weight: 700; display: flex; align-items: center; gap: 8px; }
        
        .agent-hud {
            font-family: 'JetBrains Mono', monospace;
            font-size: 11px;
            color: #a855f7;
            background: rgba(168, 85, 247, 0.1);
            border: 1px solid rgba(168, 85, 247, 0.25);
            padding: 4px 10px;
            border-radius: 6px;
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .btn {
            background: var(--primary);
            color: #fff;
            border: none;
            border-radius: 6px;
            padding: 8px 14px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            transition: all 0.15s ease;
        }
        .btn:hover { background: var(--primary-hover); }
        .btn-secondary { background: var(--surface-elevated); color: var(--text); border: 1px solid var(--border); }
        .btn-secondary:hover { background: #273147; }
        .btn-ai { background: var(--ai-gradient); }

        .view-container { padding: 24px; flex: 1; }

        /* Bento Grid KPI */
        .kpi-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(210px, 1fr)); gap: 14px; margin-bottom: 24px; }
        .kpi-card {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 18px;
            position: relative;
        }
        .kpi-label { font-size: 11px; color: var(--text-secondary); text-transform: uppercase; font-weight: 600; letter-spacing: 0.05em; margin-bottom: 6px; }
        .kpi-val { font-size: 24px; font-weight: 700; color: #fff; font-family: 'JetBrains Mono', monospace; }
        .kpi-sub { font-size: 11px; color: var(--accent); margin-top: 4px; font-family: 'JetBrains Mono', monospace; }

        /* Tables & Lists */
        .card-table {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 10px;
            overflow: hidden;
            margin-bottom: 24px;
        }
        .card-header {
            padding: 14px 18px;
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .card-title { font-size: 14px; font-weight: 700; }
        table { width: 100%; border-collapse: collapse; font-size: 13px; }
        th { background: #0c121e; color: var(--text-secondary); padding: 10px 16px; text-align: left; font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em; border-bottom: 1px solid var(--border); font-family: 'JetBrains Mono', monospace; }
        td { padding: 12px 16px; border-bottom: 1px solid var(--border); color: var(--text); }
        tr:last-child td { border-bottom: none; }
        tr:hover td { background: rgba(255, 255, 255, 0.02); }

        .badge {
            display: inline-block;
            padding: 2px 7px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            font-family: 'JetBrains Mono', monospace;
        }
        .badge-lead { background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }
        .badge-qualified { background: rgba(59, 130, 246, 0.15); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3); }
        .badge-customer { background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
        .badge-paid { background: rgba(16, 185, 129, 0.15); color: #34d399; }
        .badge-draft { background: rgba(156, 163, 175, 0.15); color: #9ca3af; }

        /* Kanban Board */
        .kanban-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 14px;
            align-items: start;
        }
        .kanban-col {
            background: #0d1322;
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 14px;
            min-height: 420px;
        }
        .kanban-head {
            font-size: 12px;
            font-weight: 700;
            margin-bottom: 12px;
            display: flex;
            justify-content: space-between;
            color: var(--text-secondary);
            font-family: 'JetBrains Mono', monospace;
        }
        .kanban-card {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 12px;
            margin-bottom: 8px;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .kanban-card:hover { border-color: var(--primary); transform: translateY(-1px); }
        .kanban-title { font-weight: 600; font-size: 13px; margin-bottom: 4px; }
        .kanban-company { font-size: 11px; color: var(--text-secondary); margin-bottom: 8px; }
        .kanban-val { font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 700; color: var(--accent); }

        /* Omarchy Command Palette (Cmd+K) */
        .palette-overlay {
            position: fixed;
            top: 0; left: 0; width: 100%; height: 100%;
            background: rgba(0, 0, 0, 0.65);
            backdrop-filter: blur(6px);
            display: none;
            align-items: flex-start;
            justify-content: center;
            padding-top: 15vh;
            z-index: 300;
        }
        .palette-box {
            background: var(--surface);
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 12px;
            width: 90%;
            max-width: 580px;
            box-shadow: 0 25px 50px rgba(0,0,0,0.7), 0 0 20px rgba(168, 85, 247, 0.2);
            overflow: hidden;
            animation: slideDown 0.15s ease-out;
        }
        @keyframes slideDown { from { transform: translateY(-12px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
        .palette-input-wrap {
            display: flex;
            align-items: center;
            padding: 14px 18px;
            border-bottom: 1px solid var(--border);
            gap: 12px;
            background: #0c121e;
        }
        .palette-input {
            flex: 1;
            background: transparent;
            border: none;
            color: #fff;
            font-size: 14px;
            outline: none;
            font-family: inherit;
        }
        .palette-list {
            max-height: 340px;
            overflow-y: auto;
            padding: 8px;
        }
        .palette-item {
            padding: 10px 14px;
            border-radius: 6px;
            font-size: 13px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            color: var(--text);
            transition: all 0.1s;
        }
        .palette-item:hover, .palette-item.active { background: var(--surface-elevated); color: #60a5fa; }
        .palette-meta { font-family: 'JetBrains Mono', monospace; font-size: 11px; color: var(--text-secondary); }

        /* Floating / Drawer AI Copilot */
        .copilot-drawer {
            position: fixed;
            top: 0;
            right: -420px;
            width: 400px;
            height: 100vh;
            background: var(--surface);
            border-left: 1px solid var(--border);
            box-shadow: -10px 0 30px rgba(0,0,0,0.5);
            transition: right 0.25s cubic-bezier(0.16, 1, 0.3, 1);
            display: flex;
            flex-direction: column;
            z-index: 100;
        }
        .copilot-drawer.open { right: 0; }
        .copilot-header {
            padding: 16px 18px;
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            background: #0d1322;
        }
        .copilot-messages {
            flex: 1;
            padding: 18px;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 12px;
        }
        .chat-bubble {
            padding: 11px 15px;
            border-radius: 10px;
            font-size: 13px;
            line-height: 1.5;
            max-width: 90%;
        }
        .chat-user {
            background: var(--primary);
            color: #fff;
            align-self: flex-end;
            border-bottom-right-radius: 2px;
        }
        .chat-ai {
            background: var(--surface-elevated);
            color: var(--text);
            align-self: flex-start;
            border-bottom-left-radius: 2px;
            border: 1px solid var(--border);
        }
        .copilot-input-bar {
            padding: 14px;
            border-top: 1px solid var(--border);
            display: flex;
            gap: 8px;
            background: #0d1322;
        }
        .copilot-input {
            flex: 1;
            background: var(--bg);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 9px 12px;
            color: #fff;
            font-size: 13px;
            outline: none;
        }
        .copilot-input:focus { border-color: var(--ai-accent); }

        /* Modal */
        .modal-overlay {
            position: fixed;
            top: 0; left: 0; width: 100%; height: 100%;
            background: rgba(0,0,0,0.7);
            backdrop-filter: blur(4px);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 200;
        }
        .modal {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 12px;
            width: 90%;
            max-width: 500px;
            padding: 22px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.6);
        }
        .modal-head { display: flex; justify-content: space-between; align-items: center; margin-bottom: 18px; }
        .form-group { margin-bottom: 12px; }
        .form-label { display: block; font-size: 11px; font-weight: 600; color: var(--text-secondary); margin-bottom: 5px; }
        .form-input { width: 100%; background: var(--bg); border: 1px solid var(--border); border-radius: 6px; padding: 9px 11px; color: #fff; font-size: 13px; outline: none; }
        .form-input:focus { border-color: var(--primary); }

        /* Mobile Bottom Nav */
        .mobile-nav { display: none; }

        @media (max-width: 768px) {
            .sidebar { display: none; }
            .mobile-nav {
                display: flex;
                position: fixed;
                bottom: 0; left: 0; width: 100%;
                height: 56px;
                background: var(--surface);
                border-top: 1px solid var(--border);
                z-index: 50;
                justify-content: space-around;
                align-items: center;
            }
            .mobile-nav-btn {
                background: transparent;
                border: none;
                color: var(--text-secondary);
                font-size: 10px;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 4px;
                font-weight: 600;
                cursor: pointer;
            }
            .mobile-nav-btn.active { color: var(--primary); }
            .kanban-grid { grid-template-columns: 1fr; }
            .topbar { padding: 0 16px; }
            .view-container { padding: 16px; padding-bottom: 74px; }
            .copilot-drawer { width: 100%; right: -100%; }
            .agent-hud { display: none; }
        }
    </style>
</head>
<body>

    <!-- Desktop Sidebar (Omarchy Style) -->
    <aside class="sidebar">
        <div class="brand">
            <div class="brand-logo">⚡</div>
            <div>
                <div class="brand-text">Conrad OS</div>
                <div class="brand-sub">OMARCHY ARCHITECTURE</div>
            </div>
        </div>

        <div class="palette-trigger" onclick="openPalette()">
            <span>⌘ Befehle suchen...</span>
            <span class="kbd-hint">⌘K</span>
        </div>

        <nav class="nav-items">
            <button class="nav-btn active" onclick="switchTab('dashboard')">
                <span>📊 Dashboard</span>
                <span class="kbd-hint">1</span>
            </button>
            <button class="nav-btn" onclick="switchTab('contacts')">
                <span>👥 Kontakte & Leads</span>
                <span class="kbd-hint">2</span>
            </button>
            <button class="nav-btn" onclick="switchTab('deals')">
                <span>💼 Vertriebs-Pipeline</span>
                <span class="kbd-hint">3</span>
            </button>
            <button class="nav-btn" onclick="switchTab('invoices')">
                <span>🧾 ERP & Faktura</span>
                <span class="kbd-hint">4</span>
            </button>
            <button class="nav-btn" onclick="switchTab('dms')">
                <span>📁 DMS Dokumente</span>
                <span class="kbd-hint">5</span>
            </button>
            <button class="nav-btn" onclick="switchTab('settings')">
                <span>⚙️ Customizer & MCP</span>
                <span class="kbd-hint">6</span>
            </button>
        </nav>

        <div class="ai-status-badge" onclick="toggleCopilot()">
            <div class="ai-pulse"></div>
            <div style="flex: 1;">
                <div style="font-size: 12px; font-weight: 700; color: #c4b5fd;">Qwen 1.5B Copilot</div>
                <div style="font-size: 10px; color: var(--text-secondary); font-family: 'JetBrains Mono';">MCP: ACTIVE [C]</div>
            </div>
            <span class="kbd-hint">C</span>
        </div>
    </aside>

    <!-- Main Workspace -->
    <main class="main-content">
        <header class="topbar">
            <h1 class="view-title" id="current-view-title">Betriebs-Cockpit</h1>
            <div class="agent-hud">
                <span style="color:var(--accent);">●</span> LOCAL ENGINE // QWEN-1.5B // WAL: 0.2ms
            </div>
            <div style="display: flex; gap: 8px; align-items: center;">
                <button class="btn btn-secondary" onclick="openPalette()">⌘K</button>
                <button class="btn btn-ai" onclick="toggleCopilot()">🤖 Copilot</button>
                <button class="btn" onclick="openNewContactModal()">+ Neuer Kontakt</button>
            </div>
        </header>

        <!-- View 1: Dashboard -->
        <section id="view-dashboard" class="view-container">
            <div class="kpi-grid">
                <div class="kpi-card">
                    <div class="kpi-label">Pipeline-Volumen</div>
                    <div class="kpi-val" id="kpi-pipeline">0,00 €</div>
                    <div class="kpi-sub">↑ Aktive Angebote</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-label">Realisierter Umsatz</div>
                    <div class="kpi-val" id="kpi-revenue" style="color: var(--accent);">0,00 €</div>
                    <div class="kpi-sub">Bezahlte Rechnungen</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-label">Offene Forderungen</div>
                    <div class="kpi-val" id="kpi-pending" style="color: var(--warning);">0,00 €</div>
                    <div class="kpi-sub">Ausstehend</div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-label">Kontakte & Leads</div>
                    <div class="kpi-val" id="kpi-contacts">0</div>
                    <div class="kpi-sub">Im CRM erfasst</div>
                </div>
            </div>

            <div class="card-table">
                <div class="card-header">
                    <div class="card-title">Neueste Kontakte & Leads</div>
                    <button class="btn btn-secondary" onclick="switchTab('contacts')">Alle anzeigen →</button>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Unternehmen</th>
                            <th>E-Mail</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody id="dashboard-recent-contacts">
                        <tr><td colspan="4" style="text-align:center; color:var(--text-secondary);">Lade Kontakte...</td></tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- View 2: Kontakte -->
        <section id="view-contacts" class="view-container" style="display: none;">
            <div style="display: flex; gap: 12px; margin-bottom: 20px;">
                <input type="text" id="contact-search" class="form-input" placeholder="🔍 Suche nach Name, Firma, E-Mail oder Tags..." oninput="filterContacts()">
                <button class="btn" onclick="openNewContactModal()">+ Neuer Kontakt</button>
            </div>
            <div class="card-table">
                <table>
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Firma</th>
                            <th>Kontakt</th>
                            <th>Status</th>
                            <th>Tags</th>
                        </tr>
                    </thead>
                    <tbody id="contacts-full-table"></tbody>
                </table>
            </div>
        </section>

        <!-- View 3: Pipeline / Deals -->
        <section id="view-deals" class="view-container" style="display: none;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <p style="color: var(--text-secondary); font-size: 13px;">Klick auf eine Karte schiebt den Deal in die nächste Phase</p>
                <button class="btn" onclick="openNewDealModal()">+ Neuer Deal</button>
            </div>
            <div class="kanban-grid">
                <div class="kanban-col" id="col-discovery">
                    <div class="kanban-head"><span>1. ERSTKONTAKT</span><span id="sum-discovery">0 €</span></div>
                    <div class="kanban-items" id="items-discovery"></div>
                </div>
                <div class="kanban-col" id="col-proposal">
                    <div class="kanban-head"><span>2. ANGEBOT</span><span id="sum-proposal">0 €</span></div>
                    <div class="kanban-items" id="items-proposal"></div>
                </div>
                <div class="kanban-col" id="col-negotiation">
                    <div class="kanban-head"><span>3. VERHANDLUNG</span><span id="sum-negotiation">0 €</span></div>
                    <div class="kanban-items" id="items-negotiation"></div>
                </div>
                <div class="kanban-col" id="col-closed_won">
                    <div class="kanban-head"><span style="color: var(--accent);">4. GEWONNEN</span><span id="sum-closed_won">0 €</span></div>
                    <div class="kanban-items" id="items-closed_won"></div>
                </div>
            </div>
        </section>

        <!-- View 4: ERP & Invoices -->
        <section id="view-invoices" class="view-container" style="display: none;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <p style="color: var(--text-secondary); font-size: 13px;">Rechnungserstellung mit automatischer 19% MwSt.</p>
                <button class="btn" onclick="openNewInvoiceModal()">+ Neue Rechnung</button>
            </div>
            <div class="card-table">
                <table>
                    <thead>
                        <tr>
                            <th>Rechnungs-Nr.</th>
                            <th>Kunde</th>
                            <th>Fälligkeit</th>
                            <th>Netto</th>
                            <th>Brutto (19%)</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody id="invoices-full-table"></tbody>
                </table>
            </div>
        </section>

        <!-- View 5: DMS -->
        <section id="view-dms" class="view-container" style="display: none;">
            <div class="card-table">
                <div class="card-header">
                    <div class="card-title">Dokumenten-Management (DMS)</div>
                    <span style="font-size: 12px; color: var(--text-secondary);">Revisionssichere Dateiablage</span>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>Titel / Datei</th>
                            <th>Kategorie</th>
                            <th>Größe</th>
                            <th>Datum</th>
                        </tr>
                    </thead>
                    <tbody id="dms-full-table">
                        <tr><td colspan="4" style="text-align:center; color:var(--text-secondary);">Keine Dokumente abgelegt.</td></tr>
                    </tbody>
                </table>
            </div>
        </section>

        <!-- View 6: Customizer & MCP -->
        <section id="view-settings" class="view-container" style="display: none;">
            <div class="card-table" style="padding: 24px;">
                <h3 style="margin-bottom: 12px;">🛠️ Model Context Protocol (MCP) Integration</h3>
                <p style="color: var(--text-secondary); font-size: 13px; line-height: 1.6; margin-bottom: 20px;">
                    Dieses CRM stellt einen nativen MCP-Server unter <code>POST /mcp</code> bereit.<br>
                    Jedes Tool (Kontaktsuche, Deal-Erstellung, Rechnungen, Custom Fields) kann von Claude Code, Antigravity oder einem lokalen Qwen aufgerufen werden.
                </p>
                <div style="background: #000; padding: 14px; border-radius: 8px; font-family: monospace; font-size: 12px; color: #34d399; margin-bottom: 24px;">
                    curl -X POST http://localhost:8080/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
                </div>

                <h3 style="margin-bottom: 12px;">🧩 Dynamische Custom Fields</h3>
                <div style="display: flex; gap: 10px; margin-bottom: 16px;">
                    <input type="text" id="new-cf-key" class="form-input" placeholder="Technischer Key (z.B. branche)">
                    <input type="text" id="new-cf-label" class="form-input" placeholder="Label (z.B. Branche)">
                    <button class="btn" onclick="saveCustomField()">Feld hinzufügen</button>
                </div>
                <div id="custom-fields-list" style="font-size: 13px; color: var(--text-secondary);"></div>
            </div>
        </section>
    </main>

    <!-- Omarchy Command Palette (Cmd+K) -->
    <div class="palette-overlay" id="palette-overlay" onclick="if(event.target===this) closePalette()">
        <div class="palette-box">
            <div class="palette-input-wrap">
                <span style="font-size: 16px;">⚡</span>
                <input type="text" id="palette-input" class="palette-input" placeholder="Omarchy Quick Command oder Ansicht suchen..." oninput="filterPalette()">
                <span class="kbd-hint">ESC</span>
            </div>
            <div class="palette-list" id="palette-list">
                <div class="palette-item" onclick="switchTab('dashboard'); closePalette();">
                    <span>📊 Gehe zu Dashboard</span>
                    <span class="palette-meta">1</span>
                </div>
                <div class="palette-item" onclick="switchTab('contacts'); closePalette();">
                    <span>👥 Gehe zu Kontakte & Leads</span>
                    <span class="palette-meta">2</span>
                </div>
                <div class="palette-item" onclick="switchTab('deals'); closePalette();">
                    <span>💼 Gehe zu Vertriebs-Pipeline</span>
                    <span class="palette-meta">3</span>
                </div>
                <div class="palette-item" onclick="switchTab('invoices'); closePalette();">
                    <span>🧾 Gehe zu ERP & Faktura</span>
                    <span class="palette-meta">4</span>
                </div>
                <div class="palette-item" onclick="switchTab('dms'); closePalette();">
                    <span>📁 Gehe zu DMS Dokumente</span>
                    <span class="palette-meta">5</span>
                </div>
                <div class="palette-item" onclick="openNewContactModal(); closePalette();">
                    <span>➕ Neuer Kontakt anlegen</span>
                    <span class="palette-meta">N C</span>
                </div>
                <div class="palette-item" onclick="openNewDealModal(); closePalette();">
                    <span>➕ Neuer Deal anlegen</span>
                    <span class="palette-meta">N D</span>
                </div>
                <div class="palette-item" onclick="openNewInvoiceModal(); closePalette();">
                    <span>➕ Neue Rechnung buchen</span>
                    <span class="palette-meta">N I</span>
                </div>
                <div class="palette-item" onclick="toggleCopilot(); closePalette();">
                    <span>🤖 Qwen 1.5B Copilot Drawer öffnen</span>
                    <span class="palette-meta">C</span>
                </div>
            </div>
        </div>
    </div>

    <!-- Mobile Bottom Navigation -->
    <nav class="mobile-nav">
        <button class="mobile-nav-btn active" onclick="switchTab('dashboard')"><span>📊</span>Cockpit</button>
        <button class="mobile-nav-btn" onclick="switchTab('contacts')"><span>👥</span>Kontakte</button>
        <button class="mobile-nav-btn" onclick="switchTab('deals')"><span>💼</span>Deals</button>
        <button class="mobile-nav-btn" onclick="switchTab('invoices')"><span>🧾</span>ERP</button>
        <button class="mobile-nav-btn" onclick="toggleCopilot()"><span>🤖</span>KI</button>
    </nav>

    <!-- AI Copilot Drawer -->
    <aside class="copilot-drawer" id="copilot-drawer">
        <div class="copilot-header">
            <div style="display: flex; align-items: center; gap: 10px;">
                <span style="font-size: 18px;">🤖</span>
                <div>
                    <div style="font-size: 14px; font-weight: 700;">Conrad Copilot</div>
                    <div style="font-size: 10px; color: #a78bfa; font-family:'JetBrains Mono';">QWEN 1.5B // MCP ENGINE</div>
                </div>
            </div>
            <button onclick="toggleCopilot()" style="background:none; border:none; color:var(--text-secondary); font-size:18px; cursor:pointer;">✕</button>
        </div>

        <div class="copilot-messages" id="copilot-messages">
            <div class="chat-bubble chat-ai">
                Hallo Merlin! Ich bin dein integrierter Qwen-Copilot. Sag mir einfach, was du tun möchtest:
                <br><br>
                💡 <em>„Wie steht unsere Pipeline heute?"</em><br>
                💡 <em>„Erstelle Kontakt Stefan Koch von Koch Sanitär"</em><br>
                💡 <em>„Erstelle Rechnung für Jens Ohrladen über 99 €"</em>
            </div>
        </div>

        <div class="copilot-input-bar">
            <input type="text" id="copilot-input" class="copilot-input" placeholder="Befehl oder Frage eingeben..." onkeydown="if(event.key==='Enter') sendCopilotMsg()">
            <button class="btn btn-ai" onclick="sendCopilotMsg()">Senden</button>
        </div>
    </aside>

    <!-- Modal: Neuer Kontakt -->
    <div class="modal-overlay" id="contact-modal">
        <div class="modal">
            <div class="modal-head">
                <h3 style="font-size: 15px; font-weight: 700;">Neuen Kontakt anlegen</h3>
                <button onclick="closeModal('contact-modal')" style="background:none; border:none; color:var(--text-secondary); font-size:18px; cursor:pointer;">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Vollständiger Name *</label>
                <input type="text" id="c-name" class="form-input" placeholder="z.B. Maximilian Mustermann">
            </div>
            <div class="form-group">
                <label class="form-label">Unternehmen / Firma</label>
                <input type="text" id="c-company" class="form-input" placeholder="z.B. Musterbau GmbH">
            </div>
            <div class="form-group">
                <label class="form-label">E-Mail-Adresse</label>
                <input type="email" id="c-email" class="form-input" placeholder="name@firma.de">
            </div>
            <div class="form-group">
                <label class="form-label">Telefonnummer</label>
                <input type="text" id="c-phone" class="form-input" placeholder="+49 170 1234567">
            </div>
            <div class="form-group">
                <label class="form-label">Status</label>
                <select id="c-status" class="form-input">
                    <option value="lead">Lead (Interessent)</option>
                    <option value="qualified">Qualifiziert</option>
                    <option value="customer">Kunde (Aktiv)</option>
                </select>
            </div>
            <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 18px;">
                <button class="btn btn-secondary" onclick="closeModal('contact-modal')">Abbrechen</button>
                <button class="btn" onclick="submitCreateContact()">Speichern</button>
            </div>
        </div>
    </div>

    <!-- Modal: Neuer Deal -->
    <div class="modal-overlay" id="deal-modal">
        <div class="modal">
            <div class="modal-head">
                <h3 style="font-size: 15px; font-weight: 700;">Neuen Deal / Angebot anlegen</h3>
                <button onclick="closeModal('deal-modal')" style="background:none; border:none; color:var(--text-secondary); font-size:18px; cursor:pointer;">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Deal Titel *</label>
                <input type="text" id="d-title" class="form-input" placeholder="z.B. KI-Postfach Autopilot Setup">
            </div>
            <div class="form-group">
                <label class="form-label">Kontakt / Kunde *</label>
                <select id="d-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Betrag in Euro (€) *</label>
                <input type="number" id="d-val" class="form-input" placeholder="z.B. 1500">
            </div>
            <div class="form-group">
                <label class="form-label">Phase (Stage)</label>
                <select id="d-stage" class="form-input">
                    <option value="discovery">1. Erstkontakt / Discovery</option>
                    <option value="proposal">2. Angebot versendet</option>
                    <option value="negotiation">3. Verhandlung</option>
                    <option value="closed_won">4. Gewonnen (Closed Won)</option>
                </select>
            </div>
            <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 18px;">
                <button class="btn btn-secondary" onclick="closeModal('deal-modal')">Abbrechen</button>
                <button class="btn" onclick="submitCreateDeal()">Deal erstellen</button>
            </div>
        </div>
    </div>

    <!-- Modal: Neue Rechnung -->
    <div class="modal-overlay" id="invoice-modal">
        <div class="modal">
            <div class="modal-head">
                <h3 style="font-size: 15px; font-weight: 700;">Rechnung erstellen (ERP)</h3>
                <button onclick="closeModal('invoice-modal')" style="background:none; border:none; color:var(--text-secondary); font-size:18px; cursor:pointer;">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Kunde *</label>
                <select id="inv-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Position / Beschreibung *</label>
                <input type="text" id="inv-desc" class="form-input" placeholder="z.B. Monatliche Betreuung n8n (November 2026)">
            </div>
            <div class="form-group">
                <label class="form-label">Nettobetrag (€) *</label>
                <input type="number" id="inv-amount" class="form-input" placeholder="z.B. 99">
            </div>
            <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 18px;">
                <button class="btn btn-secondary" onclick="closeModal('invoice-modal')">Abbrechen</button>
                <button class="btn" onclick="submitCreateInvoice()">Rechnung buchen</button>
            </div>
        </div>
    </div>

    <script>
        let contactsData = [];
        let dealsData = [];
        let invoicesData = [];

        async function init() {
            await refreshAllData();
            setupKeyboardShortcuts();
        }

        // Global Omarchy Keyboard Shortcuts
        function setupKeyboardShortcuts() {
            window.addEventListener('keydown', (e) => {
                // Ignore if focus is in an input or textarea
                const inInput = ['INPUT', 'TEXTAREA', 'SELECT'].includes(document.activeElement.tagName);

                // Cmd+K or Ctrl+K -> Command Palette
                if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                    e.preventDefault();
                    togglePalette();
                    return;
                }

                if (e.key === 'Escape') {
                    closePalette();
                    closeModal('contact-modal');
                    closeModal('deal-modal');
                    closeModal('invoice-modal');
                    document.getElementById('copilot-drawer').classList.remove('open');
                    return;
                }

                if (inInput) return;

                if (e.key === '1') switchTab('dashboard');
                if (e.key === '2') switchTab('contacts');
                if (e.key === '3') switchTab('deals');
                if (e.key === '4') switchTab('invoices');
                if (e.key === '5') switchTab('dms');
                if (e.key === '6') switchTab('settings');
                if (e.key.toLowerCase() === 'c') toggleCopilot();
                if (e.key === '/') {
                    e.preventDefault();
                    switchTab('contacts');
                    document.getElementById('contact-search')?.focus();
                }
            });
        }

        function openPalette() {
            document.getElementById('palette-overlay').style.display = 'flex';
            setTimeout(() => document.getElementById('palette-input').focus(), 50);
        }
        function closePalette() {
            document.getElementById('palette-overlay').style.display = 'none';
        }
        function togglePalette() {
            const el = document.getElementById('palette-overlay');
            if (el.style.display === 'flex') closePalette(); else openPalette();
        }

        function filterPalette() {
            const q = document.getElementById('palette-input').value.toLowerCase();
            const items = document.querySelectorAll('.palette-item');
            items.forEach(it => {
                const text = it.innerText.toLowerCase();
                it.style.display = text.includes(q) ? 'flex' : 'none';
            });
        }

        async function refreshAllData() {
            try {
                const [statsRes, contactsRes, dealsRes, invRes] = await Promise.all([
                    fetch('/mcp', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'tools/call', params: { name: 'erp_get_cashflow_stats' } })
                    }).then(r => r.json()),
                    fetch('/api/contacts').then(r => r.json()),
                    fetch('/api/deals').then(r => r.json()),
                    fetch('/api/invoices').then(r => r.json())
                ]);

                contactsData = contactsRes || [];
                dealsData = dealsRes || [];
                invoicesData = invRes || [];

                // Render KPIs
                if (statsRes?.result?.content?.[0]?.text) {
                    const stats = JSON.parse(statsRes.result.content[0].text);
                    document.getElementById('kpi-pipeline').innerText = (stats.pipeline_volume_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-revenue').innerText = (stats.paid_revenue_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-pending').innerText = (stats.pending_invoices_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-contacts').innerText = stats.total_contacts;
                }

                renderContactsTables();
                renderDealsKanban();
                renderInvoicesTable();
            } catch (e) {
                console.error("Data refresh error:", e);
            }
        }

        function renderContactsTables() {
            const tbodyDash = document.getElementById('dashboard-recent-contacts');
            const tbodyFull = document.getElementById('contacts-full-table');

            if (!contactsData.length) {
                tbodyDash.innerHTML = `<tr><td colspan="4" style="text-align:center; color:var(--text-secondary);">Keine Kontakte vorhanden.</td></tr>`;
                tbodyFull.innerHTML = `<tr><td colspan="5" style="text-align:center; color:var(--text-secondary);">Keine Kontakte vorhanden.</td></tr>`;
                return;
            }

            tbodyDash.innerHTML = contactsData.slice(0, 5).map(c => `
                <tr>
                    <td><strong>${escapeHtml(c.name)}</strong></td>
                    <td>${escapeHtml(c.company || '—')}</td>
                    <td>${escapeHtml(c.email || '—')}</td>
                    <td><span class="badge badge-${c.status}">${c.status}</span></td>
                </tr>
            `).join('');

            tbodyFull.innerHTML = contactsData.map(c => `
                <tr>
                    <td><strong>${escapeHtml(c.name)}</strong></td>
                    <td>${escapeHtml(c.company || '—')}</td>
                    <td>${escapeHtml(c.email || c.phone || '—')}</td>
                    <td><span class="badge badge-${c.status}">${c.status}</span></td>
                    <td><span style="font-size:11px; color:var(--text-secondary); font-family:'JetBrains Mono';">${escapeHtml(c.tags || '—')}</span></td>
                </tr>
            `).join('');
        }

        function renderDealsKanban() {
            const stages = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            stages.forEach(st => {
                const colItems = document.getElementById(`items-${st}`);
                const sumEl = document.getElementById(`sum-${st}`);
                const filtered = dealsData.filter(d => d.stage === st);
                const sum = filtered.reduce((acc, d) => acc + d.value_cents, 0) / 100;

                sumEl.innerText = sum.toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                colItems.innerHTML = filtered.map(d => `
                    <div class="kanban-card" onclick="advanceDeal('${d.id}', '${d.stage}')" title="Klick schiebt den Deal vorwärts">
                        <div class="kanban-title">${escapeHtml(d.title)}</div>
                        <div class="kanban-company">${escapeHtml(d.contact_name || 'Kontakt')}</div>
                        <div class="kanban-val">${(d.value_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' })}</div>
                    </div>
                `).join('');
            });
        }

        function renderInvoicesTable() {
            const tbody = document.getElementById('invoices-full-table');
            if (!invoicesData.length) {
                tbody.innerHTML = `<tr><td colspan="6" style="text-align:center; color:var(--text-secondary);">Noch keine Rechnungen erfasst.</td></tr>`;
                return;
            }

            tbody.innerHTML = invoicesData.map(inv => `
                <tr>
                    <td><strong style="font-family:'JetBrains Mono';">${inv.invoice_number}</strong></td>
                    <td>${escapeHtml(inv.contact_name || 'Kunde')}</td>
                    <td style="font-family:'JetBrains Mono';">${inv.due_date}</td>
                    <td style="font-family:'JetBrains Mono';">${(inv.net_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' })}</td>
                    <td style="font-family:'JetBrains Mono';"><strong>${(inv.gross_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' })}</strong></td>
                    <td><span class="badge badge-${inv.status}" onclick="toggleInvoicePaid('${inv.id}', '${inv.status}')" style="cursor:pointer;" title="Klick ändert Status">${inv.status}</span></td>
                </tr>
            `).join('');
        }

        async function advanceDeal(id, currentStage) {
            const stages = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            const idx = stages.indexOf(currentStage);
            if (idx === -1 || idx === stages.length - 1) return;
            const nextStage = stages[idx + 1];

            await fetch('/mcp', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    jsonrpc: '2.0',
                    id: 1,
                    method: 'tools/call',
                    params: { name: 'crm_update_deal_stage', arguments: { deal_id: id, new_stage: nextStage } }
                })
            });
            await refreshAllData();
        }

        async function toggleInvoicePaid(id, currentStatus) {
            const newStatus = currentStatus === 'paid' ? 'sent' : 'paid';
            await fetch(`/api/invoices/${id}/status`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ status: newStatus })
            });
            await refreshAllData();
        }

        function switchTab(viewId) {
            const views = ['dashboard', 'contacts', 'deals', 'invoices', 'dms', 'settings'];
            views.forEach(v => {
                const el = document.getElementById(`view-${v}`);
                if (el) el.style.display = v === viewId ? 'block' : 'none';
            });
            document.querySelectorAll('.nav-btn').forEach(btn => btn.classList.remove('active'));
            document.querySelectorAll('.mobile-nav-btn').forEach(btn => btn.classList.remove('active'));

            const titles = {
                dashboard: 'Betriebs-Cockpit',
                contacts: 'Kontakte & Leads',
                deals: 'Vertriebs-Pipeline',
                invoices: 'ERP & Rechnungen',
                dms: 'DMS Dokumente',
                settings: 'Customizer & MCP'
            };
            document.getElementById('current-view-title').innerText = titles[viewId] || 'Workspace';
        }

        function toggleCopilot() {
            document.getElementById('copilot-drawer').classList.toggle('open');
            if (document.getElementById('copilot-drawer').classList.contains('open')) {
                setTimeout(() => document.getElementById('copilot-input').focus(), 150);
            }
        }

        async function sendCopilotMsg() {
            const input = document.getElementById('copilot-input');
            const prompt = input.value.trim();
            if (!prompt) return;

            const box = document.getElementById('copilot-messages');
            box.innerHTML += `<div class="chat-bubble chat-user">${escapeHtml(prompt)}</div>`;
            input.value = '';
            box.scrollTop = box.scrollHeight;

            try {
                const res = await fetch('/api/copilot/chat', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ prompt })
                }).then(r => r.json());

                let replyHtml = escapeHtml(res.reply).replace(/\n/g, '<br>');
                if (res.actions_performed && res.actions_performed.length) {
                    replyHtml += `<div style="margin-top:8px; font-size:11px; color:#34d399; font-weight:600; font-family:'JetBrains Mono';">⚡ MCP TOOL AUSGEFÜHRT</div>`;
                }
                box.innerHTML += `<div class="chat-bubble chat-ai">${replyHtml}</div>`;
                box.scrollTop = box.scrollHeight;

                await refreshAllData();
            } catch (e) {
                box.innerHTML += `<div class="chat-bubble chat-ai" style="color:var(--danger);">Fehler beim Aufruf des Copilots.</div>`;
            }
        }

        function openNewContactModal() {
            document.getElementById('contact-modal').style.display = 'flex';
            setTimeout(() => document.getElementById('c-name').focus(), 50);
        }
        function openNewDealModal() {
            const sel = document.getElementById('d-contact-select');
            sel.innerHTML = contactsData.map(c => `<option value="${c.id}">${escapeHtml(c.name)} (${escapeHtml(c.company || 'Privat')})</option>`).join('');
            document.getElementById('deal-modal').style.display = 'flex';
        }
        function openNewInvoiceModal() {
            const sel = document.getElementById('inv-contact-select');
            sel.innerHTML = contactsData.map(c => `<option value="${c.id}">${escapeHtml(c.name)} (${escapeHtml(c.company || 'Privat')})</option>`).join('');
            document.getElementById('invoice-modal').style.display = 'flex';
        }
        function closeModal(id) {
            document.getElementById(id).style.display = 'none';
        }

        async function submitCreateContact() {
            const name = document.getElementById('c-name').value.trim();
            if (!name) return alert('Bitte Name eingeben');

            await fetch('/api/contacts', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name,
                    company: document.getElementById('c-company').value.trim(),
                    email: document.getElementById('c-email').value.trim(),
                    phone: document.getElementById('c-phone').value.trim(),
                    status: document.getElementById('c-status').value
                })
            });
            closeModal('contact-modal');
            await refreshAllData();
        }

        async function submitCreateDeal() {
            const title = document.getElementById('d-title').value.trim();
            const contact_id = document.getElementById('d-contact-select').value;
            const val = parseFloat(document.getElementById('d-val').value) || 0;
            const stage = document.getElementById('d-stage').value;

            await fetch('/api/deals', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    title,
                    contact_id,
                    value_cents: Math.round(val * 100),
                    stage
                })
            });
            closeModal('deal-modal');
            await refreshAllData();
        }

        async function submitCreateInvoice() {
            const contact_id = document.getElementById('inv-contact-select').value;
            const desc = document.getElementById('inv-desc').value.trim();
            const amount = parseFloat(document.getElementById('inv-amount').value) || 0;

            await fetch('/api/invoices', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    contact_id,
                    items: [{ description: desc, quantity: 1, unit_price_cents: Math.round(amount * 100), total_cents: Math.round(amount * 100) }]
                })
            });
            closeModal('invoice-modal');
            await refreshAllData();
        }

        function filterContacts() {
            const q = document.getElementById('contact-search').value.toLowerCase();
            const rows = document.querySelectorAll('#contacts-full-table tr');
            rows.forEach(r => {
                r.style.display = r.innerText.toLowerCase().includes(q) ? '' : 'none';
            });
        }

        function escapeHtml(text) {
            if (!text) return '';
            return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
        }

        window.onload = init;
    </script>
</body>
</html>"#
}
