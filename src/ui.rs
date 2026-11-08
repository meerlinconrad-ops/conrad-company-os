pub fn render_app_html() -> &'static str {
    r##"<!DOCTYPE html>
<html lang="de" data-theme="light">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <meta name="apple-mobile-web-app-capable" content="yes">
    <meta name="apple-mobile-web-app-status-bar-style" content="default">
    <meta name="apple-mobile-web-app-title" content="Conrad OS">
    <link rel="manifest" href="/manifest.json">
    <title>Conrad OS — Enterprise CRM, ERP &amp; DMS</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>
        /* ── Twenty CRM & HubSpot Enterprise Design System ── */
        :root {
            --font-sans: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            --font-mono: "JetBrains Mono", SFMono-Regular, Menlo, monospace;

            /* Light Theme (Twenty CRM Clean Slate) */
            --bg-app: #f8fafc;
            --bg-sidebar: #f1f5f9;
            --bg-surface: #ffffff;
            --bg-subtle: #f8fafc;
            --bg-hover: #f1f5f9;
            --bg-active: #e2e8f0;

            --border-color: #e2e8f0;
            --border-strong: #cbd5e1;

            --text-primary: #0f172a;
            --text-secondary: #475569;
            --text-muted: #64748b;

            --accent: #2563eb;
            --accent-hover: #1d4ed8;
            --accent-soft: #eff6ff;
            --accent-border: #bfdbfe;

            /* Semantic Badges (HubSpot Style) */
            --status-green-bg: #dcfce7;
            --status-green-fg: #15803d;
            --status-amber-bg: #fef3c7;
            --status-amber-fg: #b45309;
            --status-blue-bg: #dbeafe;
            --status-blue-fg: #1d4ed8;
            --status-purple-bg: #f3e8ff;
            --status-purple-fg: #7e22ce;
            --status-red-bg: #fee2e2;
            --status-red-fg: #b91c1c;
            --status-neutral-bg: #f1f5f9;
            --status-neutral-fg: #475569;

            --shadow-xs: 0 1px 2px rgba(15, 23, 42, 0.04);
            --shadow-sm: 0 1px 3px rgba(15, 23, 42, 0.06), 0 1px 2px rgba(15, 23, 42, 0.04);
            --shadow-lg: 0 10px 25px -5px rgba(15, 23, 42, 0.1), 0 8px 10px -6px rgba(15, 23, 42, 0.06);
            --shadow-drawer: -8px 0 30px rgba(15, 23, 42, 0.12);

            --sidebar-width: 240px;
        }

        [data-theme="dark"] {
            --bg-app: #090d16;
            --bg-sidebar: #0f172a;
            --bg-surface: #111827;
            --bg-subtle: #1e293b;
            --bg-hover: #1e293b;
            --bg-active: #334155;

            --border-color: #1e293b;
            --border-strong: #334155;

            --text-primary: #f8fafc;
            --text-secondary: #94a3b8;
            --text-muted: #64748b;

            --accent: #3b82f6;
            --accent-hover: #60a5fa;
            --accent-soft: rgba(59, 130, 246, 0.15);
            --accent-border: rgba(59, 130, 246, 0.35);

            --status-green-bg: rgba(21, 128, 61, 0.25);
            --status-green-fg: #4ade80;
            --status-amber-bg: rgba(180, 83, 9, 0.25);
            --status-amber-fg: #fbbf24;
            --status-blue-bg: rgba(29, 78, 216, 0.25);
            --status-blue-fg: #60a5fa;
            --status-purple-bg: rgba(126, 34, 206, 0.25);
            --status-purple-fg: #c084fc;
            --status-red-bg: rgba(185, 28, 28, 0.25);
            --status-red-fg: #f87171;
            --status-neutral-bg: #1e293b;
            --status-neutral-fg: #94a3b8;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            background-color: var(--bg-app);
            color: var(--text-primary);
            font-family: var(--font-sans);
            font-size: 13px;
            line-height: 1.45;
            display: flex;
            height: 100vh;
            overflow: hidden;
            -webkit-font-smoothing: antialiased;
        }

        svg.icon {
            width: 15px;
            height: 15px;
            stroke: currentColor;
            stroke-width: 1.8;
            fill: none;
            stroke-linecap: round;
            stroke-linejoin: round;
            flex-shrink: 0;
        }

        /* ── Twenty CRM Sidebar ── */
        .sidebar {
            width: var(--sidebar-width);
            background: var(--bg-sidebar);
            border-right: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
            padding: 12px 10px;
            flex-shrink: 0;
            z-index: 25;
            user-select: none;
        }

        .workspace-switcher {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 8px 10px;
            border-radius: 6px;
            margin-bottom: 10px;
            cursor: pointer;
        }

        .workspace-switcher:hover {
            background: var(--bg-hover);
        }

        .workspace-logo {
            width: 24px;
            height: 24px;
            border-radius: 5px;
            background: var(--text-primary);
            color: var(--bg-surface);
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 12px;
        }

        .workspace-name {
            font-weight: 600;
            font-size: 13px;
            color: var(--text-primary);
            flex: 1;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .cmdk-trigger-btn {
            display: flex;
            align-items: center;
            gap: 8px;
            width: 100%;
            padding: 6px 10px;
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 6px;
            color: var(--text-muted);
            font-size: 12px;
            cursor: pointer;
            margin-bottom: 12px;
            box-shadow: var(--shadow-xs);
        }

        .cmdk-trigger-btn:hover {
            border-color: var(--border-strong);
            color: var(--text-primary);
        }

        .kbd-tag {
            margin-left: auto;
            font-family: var(--font-mono);
            font-size: 10px;
            padding: 1px 5px;
            border-radius: 4px;
            background: var(--bg-subtle);
            border: 1px solid var(--border-color);
            color: var(--text-muted);
        }

        /* Segmented Role Switcher */
        .mode-segmented-control {
            display: grid;
            grid-template-columns: 1fr 1fr;
            background: var(--bg-active);
            padding: 2px;
            border-radius: 6px;
            margin-bottom: 14px;
        }

        .mode-segmented-btn {
            border: none;
            background: transparent;
            color: var(--text-secondary);
            font-size: 11px;
            font-weight: 600;
            padding: 5px 8px;
            border-radius: 4px;
            cursor: pointer;
            font-family: inherit;
        }

        .mode-segmented-btn.active {
            background: var(--bg-surface);
            color: var(--text-primary);
            box-shadow: var(--shadow-xs);
        }

        .nav-group-title {
            font-size: 11px;
            font-weight: 600;
            color: var(--text-muted);
            padding: 10px 10px 4px;
            text-transform: uppercase;
            letter-spacing: 0.04em;
        }

        .nav-list {
            list-style: none;
            display: flex;
            flex-direction: column;
            gap: 1px;
        }

        .nav-item-btn {
            display: flex;
            align-items: center;
            gap: 9px;
            width: 100%;
            padding: 7px 10px;
            border: none;
            background: transparent;
            color: var(--text-secondary);
            font-size: 13px;
            font-weight: 500;
            border-radius: 6px;
            cursor: pointer;
            text-align: left;
            font-family: inherit;
        }

        .nav-item-btn:hover {
            background: var(--bg-hover);
            color: var(--text-primary);
        }

        .nav-item-btn.active {
            background: var(--bg-active);
            color: var(--text-primary);
            font-weight: 600;
        }

        .nav-count {
            margin-left: auto;
            font-family: var(--font-mono);
            font-size: 11px;
            color: var(--text-muted);
        }

        .sidebar-footer {
            margin-top: auto;
            padding: 8px 10px;
            border-top: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            gap: 9px;
            cursor: pointer;
            border-radius: 6px;
        }

        .sidebar-footer:hover {
            background: var(--bg-hover);
        }

        .avatar-sm {
            width: 26px;
            height: 26px;
            border-radius: 6px;
            background: var(--accent);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 600;
            font-size: 11px;
            flex-shrink: 0;
        }

        /* ── Main Content Shell ── */
        .main-wrapper {
            flex: 1;
            display: flex;
            flex-direction: column;
            height: 100vh;
            overflow: hidden;
            background: var(--bg-surface);
        }

        /* Breadcrumb & Action Topbar */
        .topbar {
            height: 48px;
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 20px;
            background: var(--bg-surface);
            flex-shrink: 0;
        }

        .topbar-left {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 13px;
        }

        .breadcrumb-root {
            color: var(--text-muted);
            font-weight: 500;
        }

        .breadcrumb-sep {
            color: var(--border-strong);
        }

        .topbar-title {
            font-weight: 600;
            color: var(--text-primary);
        }

        .topbar-right {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        /* View Filter Bar (Twenty Style Subheader) */
        .sub-toolbar {
            height: 42px;
            border-bottom: 1px solid var(--border-color);
            background: var(--bg-subtle);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 20px;
            flex-shrink: 0;
        }

        .filter-pills {
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .filter-pill-btn {
            padding: 4px 10px;
            border-radius: 5px;
            border: 1px solid transparent;
            background: transparent;
            color: var(--text-secondary);
            font-size: 12px;
            font-weight: 500;
            cursor: pointer;
            font-family: inherit;
        }

        .filter-pill-btn:hover {
            background: var(--bg-hover);
            color: var(--text-primary);
        }

        .filter-pill-btn.active {
            background: var(--bg-surface);
            border-color: var(--border-color);
            color: var(--text-primary);
            box-shadow: var(--shadow-xs);
            font-weight: 600;
        }

        /* Buttons */
        .btn {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            height: 30px;
            padding: 0 11px;
            border-radius: 6px;
            font-size: 12px;
            font-weight: 500;
            font-family: inherit;
            cursor: pointer;
            border: 1px solid var(--border-color);
            background: var(--bg-surface);
            color: var(--text-primary);
            box-shadow: var(--shadow-xs);
            transition: all 0.12s ease;
            white-space: nowrap;
        }

        .btn:hover {
            background: var(--bg-hover);
            border-color: var(--border-strong);
        }

        .btn-primary {
            background: var(--accent);
            border-color: var(--accent);
            color: #ffffff;
        }

        .btn-primary:hover {
            background: var(--accent-hover);
            border-color: var(--accent-hover);
        }

        /* Content Scroll Area */
        .view-content-area {
            flex: 1;
            overflow-y: auto;
            background: var(--bg-app);
        }

        .view-padded {
            padding: 20px 24px 60px;
            max-width: 1400px;
        }

        /* KPI Strip (Enterprise Metric Row) */
        .kpi-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 12px;
            margin-bottom: 20px;
        }

        @media (max-width: 1024px) {
            .kpi-grid { grid-template-columns: repeat(2, 1fr); }
        }
        @media (max-width: 640px) {
            .kpi-grid { grid-template-columns: 1fr; }
        }

        .kpi-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 14px 16px;
            box-shadow: var(--shadow-xs);
        }

        .kpi-label {
            font-size: 11px;
            font-weight: 600;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.03em;
            margin-bottom: 6px;
            display: flex;
            justify-content: space-between;
        }

        .kpi-value {
            font-size: 20px;
            font-weight: 700;
            font-family: var(--font-mono);
            color: var(--text-primary);
            letter-spacing: -0.02em;
        }

        .kpi-meta {
            font-size: 11px;
            color: var(--text-muted);
            margin-top: 4px;
        }

        /* ── Twenty CRM Spreadsheet Data Grid ── */
        .sheet-wrapper {
            background: var(--bg-surface);
            border-bottom: 1px solid var(--border-color);
            width: 100%;
            overflow-x: auto;
        }

        .card-panel {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            box-shadow: var(--shadow-xs);
            overflow: hidden;
            margin-bottom: 16px;
        }

        .card-panel-header {
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            background: var(--bg-surface);
        }

        .card-panel-title {
            font-size: 13px;
            font-weight: 600;
            color: var(--text-primary);
        }

        table.data-grid {
            width: 100%;
            border-collapse: collapse;
            text-align: left;
            font-size: 13px;
        }

        table.data-grid th {
            height: 34px;
            padding: 0 14px;
            font-size: 11px;
            font-weight: 600;
            color: var(--text-muted);
            background: var(--bg-subtle);
            border-bottom: 1px solid var(--border-color);
            border-right: 1px solid var(--border-color);
            white-space: nowrap;
            user-select: none;
        }

        table.data-grid th:last-child {
            border-right: none;
        }

        table.data-grid td {
            height: 38px;
            padding: 0 14px;
            border-bottom: 1px solid var(--border-color);
            border-right: 1px solid var(--border-color);
            color: var(--text-primary);
            vertical-align: middle;
            white-space: nowrap;
        }

        table.data-grid td:last-child {
            border-right: none;
        }

        table.data-grid tbody tr {
            cursor: pointer;
            transition: background-color 0.08s ease;
        }

        table.data-grid tbody tr:hover td {
            background: var(--bg-hover);
        }

        .cell-entity {
            display: flex;
            align-items: center;
            gap: 9px;
            font-weight: 500;
        }

        .entity-avatar {
            width: 22px;
            height: 22px;
            border-radius: 5px;
            background: var(--accent-soft);
            color: var(--accent);
            border: 1px solid var(--accent-border);
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 10px;
            font-weight: 700;
            flex-shrink: 0;
        }

        .mono-val {
            font-family: var(--font-mono);
            font-size: 12px;
        }

        /* Status Pills */
        .badge {
            display: inline-flex;
            align-items: center;
            gap: 5px;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
        }

        .badge-dot {
            width: 6px;
            height: 6px;
            border-radius: 50%;
            background: currentColor;
        }

        .badge.success { background: var(--status-green-bg); color: var(--status-green-fg); }
        .badge.warning { background: var(--status-amber-bg); color: var(--status-amber-fg); }
        .badge.info    { background: var(--status-blue-bg); color: var(--status-blue-fg); }
        .badge.purple  { background: var(--status-purple-bg); color: var(--status-purple-fg); }
        .badge.danger  { background: var(--status-red-bg); color: var(--status-red-fg); }
        .badge.neutral { background: var(--status-neutral-bg); color: var(--status-neutral-fg); }

        .tag-chip {
            display: inline-block;
            padding: 1px 6px;
            border-radius: 4px;
            background: var(--bg-subtle);
            border: 1px solid var(--border-color);
            color: var(--text-secondary);
            font-size: 11px;
            margin-right: 4px;
        }

        /* ── HubSpot / Twenty Kanban Pipeline Board ── */
        .kanban-board {
            display: grid;
            grid-template-columns: repeat(4, minmax(260px, 1fr));
            gap: 12px;
            padding: 16px 20px;
            overflow-x: auto;
            height: calc(100vh - 92px);
            align-items: start;
        }

        .kanban-column {
            background: var(--bg-sidebar);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            display: flex;
            flex-direction: column;
            max-height: 100%;
            padding: 10px;
        }

        .kanban-col-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 4px 4px 10px;
            border-bottom: 1px solid var(--border-color);
            margin-bottom: 10px;
        }

        .kanban-stage-name {
            font-size: 12px;
            font-weight: 600;
            color: var(--text-primary);
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .kanban-stage-sum {
            font-family: var(--font-mono);
            font-size: 11px;
            font-weight: 600;
            color: var(--text-muted);
        }

        .kanban-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 6px;
            padding: 11px 12px;
            margin-bottom: 8px;
            cursor: grab;
            box-shadow: var(--shadow-xs);
            transition: border-color 0.12s ease, box-shadow 0.12s ease;
        }

        .kanban-card:hover {
            border-color: var(--border-strong);
            box-shadow: var(--shadow-sm);
        }

        .kanban-card-title {
            font-size: 13px;
            font-weight: 600;
            color: var(--text-primary);
            margin-bottom: 6px;
        }

        .kanban-card-company {
            font-size: 12px;
            color: var(--text-secondary);
            display: flex;
            align-items: center;
            gap: 6px;
            margin-bottom: 10px;
        }

        .kanban-card-footer {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding-top: 8px;
            border-top: 1px solid var(--border-color);
            font-size: 11px;
        }

        /* ── HubSpot / Twenty Record 360° Slide-Over Inspector ── */
        .record-drawer-backdrop {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(15, 23, 42, 0.25);
            z-index: 60;
            justify-content: flex-end;
        }

        .record-drawer-backdrop.open {
            display: flex;
        }

        .record-drawer {
            width: min(94vw, 680px);
            height: 100vh;
            background: var(--bg-surface);
            border-left: 1px solid var(--border-color);
            box-shadow: var(--shadow-drawer);
            display: flex;
            flex-direction: column;
            animation: slideLeft 0.18s ease-out;
        }

        @keyframes slideLeft {
            from { transform: translateX(100%); }
            to { transform: translateX(0); }
        }

        .drawer-header {
            padding: 16px 20px;
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            background: var(--bg-surface);
        }

        .drawer-entity-head {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .drawer-avatar-lg {
            width: 38px;
            height: 38px;
            border-radius: 8px;
            background: var(--accent-soft);
            color: var(--accent);
            border: 1px solid var(--accent-border);
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 15px;
        }

        .drawer-body {
            flex: 1;
            display: grid;
            grid-template-columns: 230px 1fr;
            overflow: hidden;
        }

        @media (max-width: 680px) {
            .drawer-body { grid-template-columns: 1fr; overflow-y: auto; }
        }

        .drawer-props-pane {
            border-right: 1px solid var(--border-color);
            background: var(--bg-subtle);
            padding: 16px;
            overflow-y: auto;
        }

        .prop-row {
            margin-bottom: 12px;
        }

        .prop-label {
            font-size: 11px;
            font-weight: 600;
            color: var(--text-muted);
            margin-bottom: 3px;
            display: block;
        }

        .prop-input {
            width: 100%;
            padding: 5px 8px;
            font-size: 12px;
            font-family: inherit;
            color: var(--text-primary);
            background: transparent;
            border: 1px solid transparent;
            border-radius: 4px;
        }

        .prop-input:hover, .prop-input:focus {
            background: var(--bg-surface);
            border-color: var(--border-strong);
            outline: none;
        }

        .drawer-activity-pane {
            display: flex;
            flex-direction: column;
            overflow: hidden;
            background: var(--bg-surface);
        }

        .composer-tabs {
            display: flex;
            border-bottom: 1px solid var(--border-color);
            background: var(--bg-subtle);
            padding: 0 16px;
            gap: 16px;
        }

        .composer-tab {
            padding: 10px 4px;
            font-size: 12px;
            font-weight: 600;
            color: var(--text-muted);
            background: none;
            border: none;
            border-bottom: 2px solid transparent;
            cursor: pointer;
            font-family: inherit;
        }

        .composer-tab.active {
            color: var(--accent);
            border-bottom-color: var(--accent);
        }

        .composer-box {
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            background: var(--bg-surface);
        }

        .timeline-feed {
            flex: 1;
            overflow-y: auto;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 12px;
            background: var(--bg-app);
        }

        .timeline-item {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 6px;
            padding: 10px 12px;
            box-shadow: var(--shadow-xs);
        }

        .timeline-item-head {
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 11px;
            color: var(--text-muted);
            margin-bottom: 6px;
        }

        /* ── AI Copilot Side-Drawer ── */
        .copilot-panel {
            position: fixed;
            top: 0;
            right: -400px;
            width: 380px;
            height: 100vh;
            background: var(--bg-surface);
            border-left: 1px solid var(--border-color);
            box-shadow: var(--shadow-drawer);
            transition: right 0.2s ease;
            display: flex;
            flex-direction: column;
            z-index: 70;
        }

        .copilot-panel.open {
            right: 0;
        }

        .copilot-messages {
            flex: 1;
            overflow-y: auto;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 10px;
            background: var(--bg-app);
        }

        .chat-bubble {
            max-width: 88%;
            padding: 10px 12px;
            border-radius: 8px;
            font-size: 12.5px;
            line-height: 1.45;
        }

        .chat-bubble.user {
            background: var(--accent);
            color: #fff;
            align-self: flex-end;
        }

        .chat-bubble.ai {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            color: var(--text-primary);
            align-self: flex-start;
        }

        /* ── Cmd+K Command Palette Modal (Linear / Twenty Style) ── */
        .cmdk-backdrop {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(15, 23, 42, 0.4);
            backdrop-filter: blur(2px);
            z-index: 100;
            align-items: flex-start;
            justify-content: center;
            padding-top: 12vh;
        }

        .cmdk-modal {
            width: min(92vw, 560px);
            background: var(--bg-surface);
            border: 1px solid var(--border-strong);
            border-radius: 10px;
            box-shadow: var(--shadow-lg);
            overflow: hidden;
        }

        .cmdk-input-row {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
        }

        .cmdk-input {
            flex: 1;
            border: none;
            background: transparent;
            font-size: 14px;
            color: var(--text-primary);
            font-family: inherit;
            outline: none;
        }

        /* Standard Modals */
        .modal-backdrop {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(15, 23, 42, 0.4);
            z-index: 90;
            align-items: center;
            justify-content: center;
        }

        .modal-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            box-shadow: var(--shadow-lg);
            width: min(92vw, 440px);
            padding: 20px;
        }

        .modal-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 16px;
            padding-bottom: 10px;
            border-bottom: 1px solid var(--border-color);
        }

        .modal-title {
            font-size: 14px;
            font-weight: 600;
        }

        .modal-close-btn {
            background: none;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            font-size: 16px;
        }

        .form-group {
            margin-bottom: 12px;
        }

        .form-label {
            display: block;
            font-size: 11px;
            font-weight: 600;
            color: var(--text-secondary);
            margin-bottom: 4px;
        }

        .form-input {
            width: 100%;
            height: 32px;
            padding: 0 10px;
            border: 1px solid var(--border-color);
            border-radius: 6px;
            background: var(--bg-surface);
            color: var(--text-primary);
            font-size: 13px;
            font-family: inherit;
            outline: none;
        }

        .form-input:focus {
            border-color: var(--accent);
            box-shadow: 0 0 0 2px var(--accent-soft);
        }

        .dashboard-split-grid {
            display: grid;
            grid-template-columns: 1.5fr 1fr;
            gap: 16px;
        }

        @media (max-width: 900px) {
            .dashboard-split-grid {
                grid-template-columns: 1fr;
            }
        }

        @media (max-width: 768px) {
            #mobileMenuBtn { display: inline-flex !important; }
            .breadcrumb-root, .breadcrumb-sep, #topbarModePill, #btnBookingTopbar, #topbarActionBtn {
                display: none !important;
            }
            .topbar {
                padding: 0 12px;
            }
            .sub-toolbar {
                flex-direction: column;
                height: auto;
                padding: 10px 12px;
                gap: 8px;
                align-items: stretch;
            }
            .view-padded {
                padding: 14px 12px 50px;
            }
            .sidebar {
                position: fixed;
                left: -100%;
                top: 0;
                bottom: 0;
                transition: left 0.2s ease;
                box-shadow: var(--shadow-lg);
            }
            .sidebar.mobile-open { left: 0; }
            .copilot-panel { width: 100% !important; right: -100% !important; }
            .copilot-panel.open { right: 0 !important; }
        }
    </style>
</head>
<body>

    <!-- ── Twenty CRM Left Sidebar ── -->
    <aside class="sidebar" id="appSidebar">
        <div class="workspace-switcher" onclick="openLoginModal()" title="Workspace &amp; Rolle wechseln">
            <div class="workspace-logo">C</div>
            <span class="workspace-name" id="ui-brand-name">Conrad KI</span>
            <span style="font-size:10px; color:var(--text-muted);">▾</span>
        </div>

        <button class="cmdk-trigger-btn" onclick="openCmdKModal()">
            <svg class="icon" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
            <span>Suche &amp; Befehle</span>
            <span class="kbd-tag">⌘K</span>
        </button>

        <div class="mode-segmented-control" role="tablist">
            <button class="mode-segmented-btn active" id="btnModeUser" onclick="switchAppMode('user')">Operativ</button>
            <button class="mode-segmented-btn" id="btnModeAdmin" onclick="switchAppMode('admin')">Architekt</button>
        </div>

        <div class="nav-group-title">CRM &amp; Vertrieb</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn active" id="nav-btn-dashboard" onclick="switchView('dashboard')">
                    <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="7" height="9" rx="1"/><rect x="14" y="3" width="7" height="5" rx="1"/><rect x="14" y="12" width="7" height="9" rx="1"/><rect x="3" y="16" width="7" height="5" rx="1"/></svg>
                    <span>Cockpit &amp; Aufgaben</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-contacts" onclick="switchView('contacts')">
                    <svg class="icon" viewBox="0 0 24 24"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                    <span>Personen &amp; Firmen</span>
                    <span class="nav-count" id="badge-contacts-count">0</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-deals" onclick="switchView('deals')">
                    <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
                    <span>Deals Pipeline</span>
                    <span class="nav-count" id="badge-deals-count">0</span>
                </button>
            </li>
        </ul>

        <div class="nav-group-title">ERP &amp; Finanzen</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn" id="nav-btn-invoices" onclick="switchView('invoices')">
                    <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>
                    <span>Faktura (ZUGFeRD)</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-retainers" onclick="switchView('retainers')">
                    <svg class="icon" viewBox="0 0 24 24"><polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>
                    <span>Retainer &amp; Abos</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-marketing" onclick="switchView('marketing')">
                    <svg class="icon" viewBox="0 0 24 24"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
                    <span>Marketing Attribution</span>
                </button>
            </li>
        </ul>

        <div class="nav-group-title">Compliance &amp; System</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn" id="nav-btn-dms" onclick="switchView('dms')">
                    <svg class="icon" viewBox="0 0 24 24"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                    <span>GoBD Journal</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-settings" onclick="switchView('settings')">
                    <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
                    <span>Datenmodell &amp; System</span>
                </button>
            </li>
        </ul>

        <div class="sidebar-footer" onclick="openLoginModal()">
            <div class="avatar-sm" id="ui-user-avatar">MC</div>
            <div style="overflow:hidden; flex:1;">
                <div style="font-weight:600; font-size:12px;" id="ui-user-name">Merlin Conrad</div>
                <div style="font-size:11px; color:var(--text-muted);" id="ui-user-role">Administrator</div>
            </div>
        </div>
    </aside>

    <!-- ── Main Workspace ── -->
    <div class="main-wrapper">
        <header class="topbar">
            <div class="topbar-left">
                <button class="btn" style="display:none; padding:0 8px;" id="mobileMenuBtn" onclick="toggleMobileSidebar()">☰</button>
                <span class="breadcrumb-root">Workspace</span>
                <span class="breadcrumb-sep">/</span>
                <span class="topbar-title" id="currentViewTitle">Cockpit &amp; Aufgaben</span>
                <span class="badge info" id="topbarModePill" style="margin-left:8px;">
                    <span class="badge-dot"></span>
                    <span id="topbarModeLabel">Operativ</span>
                </span>
            </div>

            <div class="topbar-right">
                <a href="/book/merlin" id="btnBookingTopbar" target="_blank" class="btn" style="text-decoration:none;" title="Öffentliche Terminbuchungsseite">
                    <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="18" rx="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
                    <span>Buchungs-Link</span>
                </a>
                <button class="btn" onclick="toggleTheme()" title="Erscheinungsbild umschalten">
                    <span id="themeToggleIcon">◐</span>
                </button>
                <button class="btn btn-apple-ai" onclick="toggleCopilot()">
                    <svg class="icon" viewBox="0 0 24 24"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
                    <span id="copilotBtnLabel">KI-Assistent</span>
                </button>
                <button class="btn btn-primary" id="topbarActionBtn" onclick="openCreateModal()">
                    <span>+ Datensatz</span>
                </button>
            </div>
        </header>

        <div class="view-content-area">

            <!-- VIEW 1: COCKPIT & TASKS -->
            <section id="view-dashboard" class="view-padded">
                <div class="kpi-grid">
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Pipeline-Volumen</span><span>EUR</span></div>
                        <div class="kpi-value" id="kpi-pipeline">0,00 €</div>
                        <div class="kpi-meta">Gewichtete Verkaufschancen</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Realisierter Umsatz</span><span>GoBD</span></div>
                        <div class="kpi-value" id="kpi-revenue" style="color:var(--status-green-fg);">0,00 €</div>
                        <div class="kpi-meta">Festgeschrieben &amp; bezahlt</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Offene Forderungen</span><span>Faktura</span></div>
                        <div class="kpi-value" id="kpi-pending" style="color:var(--status-amber-fg);">0,00 €</div>
                        <div class="kpi-meta">Wartend auf Zahlungseingang</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Aktive Datensätze</span><span>CRM</span></div>
                        <div class="kpi-value" id="kpi-contacts">0</div>
                        <div class="kpi-meta">Kontakte &amp; Unternehmen</div>
                    </div>
                </div>

                <div class="dashboard-split-grid">
                    <div class="card-panel">
                        <div class="card-panel-header">
                            <span class="card-panel-title">Neueste Personen &amp; Unternehmen</span>
                            <button class="btn" onclick="switchView('contacts')">Alle öffnen →</button>
                        </div>
                        <table class="data-grid">
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Unternehmen</th>
                                    <th>Status</th>
                                    <th>Erstellt</th>
                                </tr>
                            </thead>
                            <tbody id="dashboardRecentContacts"></tbody>
                        </table>
                    </div>

                    <div class="card-panel">
                        <div class="card-panel-header">
                            <span class="card-panel-title">Fällige Follow-up Aufgaben (Workflows)</span>
                            <span class="badge neutral" id="tasksCountBadge">0 offen</span>
                        </div>
                        <div id="dashboardTasksList" style="padding:12px 16px; display:flex; flex-direction:column; gap:8px;"></div>
                    </div>
                </div>
            </section>

            <!-- VIEW 2: KONTAKTE & UNTERNEHMEN (Twenty CRM Spreadsheet View) -->
            <section id="view-contacts" style="display:none;">
                <div class="sub-toolbar">
                    <div class="filter-pills">
                        <button class="filter-pill-btn active" id="flt-all" onclick="setContactStatusFilter('all')">Alle Datensätze</button>
                        <button class="filter-pill-btn" id="flt-lead" onclick="setContactStatusFilter('lead')">Leads</button>
                        <button class="filter-pill-btn" id="flt-qualified" onclick="setContactStatusFilter('qualified')">Qualifiziert</button>
                        <button class="filter-pill-btn" id="flt-customer" onclick="setContactStatusFilter('customer')">Bestandskunden</button>
                    </div>
                    <div style="display:flex; gap:8px; align-items:center;">
                        <input type="text" id="contactSearchInput" class="form-input" style="height:28px; width:240px; font-size:12px;" placeholder="Tabelle filtern (Name, Firma, E-Mail)..." oninput="filterContactsList()">
                        <button class="btn btn-primary" onclick="openNewContactModal()">+ Neuer Kontakt</button>
                    </div>
                </div>
                <div class="sheet-wrapper">
                    <table class="data-grid">
                        <thead>
                            <tr>
                                <th style="width:240px;">Name</th>
                                <th style="width:200px;">Unternehmen</th>
                                <th style="width:220px;">E-Mail</th>
                                <th style="width:150px;">Telefon</th>
                                <th style="width:130px;">Lifecycle Phase</th>
                                <th>Segment / Tags</th>
                                <th style="width:130px;">Aktionen</th>
                            </tr>
                        </thead>
                        <tbody id="contactsFullTable"></tbody>
                    </table>
                </div>
            </section>

            <!-- VIEW 3: DEALS PIPELINE (HubSpot / Twenty Kanban) -->
            <section id="view-deals" style="display:none;">
                <div class="sub-toolbar">
                    <div style="font-size:12px; color:var(--text-secondary);">
                        Drag &amp; Drop zwischen Phasen · Wechsel auf <strong>2. Angebot</strong> erstellt automatisch Follow-up Aufgabe (+3 Tage) &amp; E-Mail-Entwurf.
                    </div>
                    <button class="btn btn-primary" onclick="openNewDealModal()">+ Deal anlegen</button>
                </div>
                <div class="kanban-board">
                    <div class="kanban-column" id="k-col-discovery">
                        <div class="kanban-col-header">
                            <span class="kanban-stage-name"><span class="badge info">1</span> Erstkontakt</span>
                            <span class="kanban-stage-sum" id="k-sum-discovery">0 €</span>
                        </div>
                        <div id="k-items-discovery"></div>
                    </div>
                    <div class="kanban-column" id="k-col-proposal">
                        <div class="kanban-col-header">
                            <span class="kanban-stage-name"><span class="badge warning">2</span> Angebot</span>
                            <span class="kanban-stage-sum" id="k-sum-proposal">0 €</span>
                        </div>
                        <div id="k-items-proposal"></div>
                    </div>
                    <div class="kanban-column" id="k-col-negotiation">
                        <div class="kanban-col-header">
                            <span class="kanban-stage-name"><span class="badge purple">3</span> Verhandlung</span>
                            <span class="kanban-stage-sum" id="k-sum-negotiation">0 €</span>
                        </div>
                        <div id="k-items-negotiation"></div>
                    </div>
                    <div class="kanban-column" id="k-col-closed_won">
                        <div class="kanban-col-header">
                            <span class="kanban-stage-name"><span class="badge success">4</span> Gewonnen</span>
                            <span class="kanban-stage-sum" id="k-sum-closed_won">0 €</span>
                        </div>
                        <div id="k-items-closed_won"></div>
                    </div>
                </div>
            </section>

            <!-- VIEW 4: FAKTURA & E-RECHNUNG -->
            <section id="view-invoices" style="display:none;">
                <div class="sub-toolbar">
                    <div style="font-size:12px; color:var(--text-secondary);">
                        EN 16931 XRechnung 3.0 / ZUGFeRD 2.2 · GoBD-Unveränderbarkeit &amp; DATEV Buchungsstapel
                    </div>
                    <div style="display:flex; gap:8px;">
                        <a href="/api/export/datev?year=2026&skr=skr03" class="btn" style="text-decoration:none;">↓ DATEV EXTF (SKR03)</a>
                        <button class="btn btn-primary" onclick="openNewInvoiceModal()">+ Neue Rechnung</button>
                    </div>
                </div>
                <div class="sheet-wrapper">
                    <table class="data-grid">
                        <thead>
                            <tr>
                                <th>Beleg-Nr.</th>
                                <th>Kunde</th>
                                <th>Fälligkeit</th>
                                <th>Netto</th>
                                <th>Brutto (inkl. USt)</th>
                                <th>GoBD Status</th>
                                <th>E-Rechnung / Zahlung</th>
                            </tr>
                        </thead>
                        <tbody id="invoicesFullTable"></tbody>
                    </table>
                </div>
            </section>

            <!-- VIEW 5: RETAINER & ABOS -->
            <section id="view-retainers" style="display:none;">
                <div class="sub-toolbar">
                    <div style="font-size:12px; color:var(--text-secondary);">
                        Wiederkehrende Service-Verträge (MRR) &amp; automatischer Monatsabrechnungslauf
                    </div>
                    <button class="btn btn-primary" onclick="triggerRetainerRun()">Fällige Retainer jetzt abrechnen</button>
                </div>
                <div class="sheet-wrapper">
                    <table class="data-grid">
                        <thead>
                            <tr>
                                <th>Kunde</th>
                                <th>Vertragsleistung</th>
                                <th>Monatspauschale (Netto)</th>
                                <th>Abrechnungsintervall</th>
                                <th>Nächster Lauf</th>
                                <th>Status</th>
                            </tr>
                        </thead>
                        <tbody id="retainersFullTable"></tbody>
                    </table>
                </div>
            </section>

            <!-- VIEW 6: MARKETING ATTRIBUTION -->
            <section id="view-marketing" class="view-padded" style="display:none;">
                <div class="kpi-grid">
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Ad Spend Gesamt</span><span>EUR</span></div>
                        <div class="kpi-value" id="m-kpi-spend">0,00 €</div>
                        <div class="kpi-meta">Meta &amp; Google Kampagnen</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Akquirierte Leads</span><span>CRM</span></div>
                        <div class="kpi-value" id="m-kpi-leads">0</div>
                        <div class="kpi-meta">Direkte Funnel-Zuordnung</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Blended CAC</span><span>Effizienz</span></div>
                        <div class="kpi-value" id="m-kpi-cac">0,00 €</div>
                        <div class="kpi-meta">Akquisekosten pro Kunde</div>
                    </div>
                    <div class="kpi-card">
                        <div class="kpi-label"><span>Return on Ad Spend</span><span>ROAS</span></div>
                        <div class="kpi-value" id="m-kpi-roas" style="color:var(--accent);">0.0x</div>
                        <div class="kpi-meta">Umsatz-Multiplikator</div>
                    </div>
                </div>

                <div class="card-panel">
                    <div class="card-panel-header">
                        <span class="card-panel-title">Kampagnen-Attribution</span>
                    </div>
                    <table class="data-grid">
                        <thead>
                            <tr>
                                <th>Kampagne</th>
                                <th>Kanal</th>
                                <th>Spend</th>
                                <th>Klicks</th>
                                <th>Leads</th>
                                <th>Abschlüsse</th>
                                <th>Umsatz</th>
                                <th>Status</th>
                            </tr>
                        </thead>
                        <tbody id="marketingCampaignsTable"></tbody>
                    </table>
                </div>
            </section>

            <!-- VIEW 7: GOBD AUDIT JOURNAL -->
            <section id="view-dms" style="display:none;">
                <div class="sub-toolbar">
                    <div style="font-size:12px; color:var(--text-secondary);">
                        Kryptografisches SHA-256 Transaktionsregister gemäß GoBD
                    </div>
                    <button class="btn" onclick="verifyGobdChain()">Kryptografische Integrität prüfen</button>
                </div>
                <div id="gobdVerifyResult" style="padding:10px 20px; font-size:12px; display:none; border-bottom:1px solid var(--border-color);"></div>
                <div class="sheet-wrapper">
                    <table class="data-grid">
                        <thead>
                            <tr>
                                <th style="width:70px;">Seq</th>
                                <th style="width:160px;">Zeitstempel</th>
                                <th style="width:180px;">Aktion</th>
                                <th style="width:140px;">Objekt-ID</th>
                                <th>Revisions-Details</th>
                                <th style="width:140px;">Akteur</th>
                            </tr>
                        </thead>
                        <tbody id="gobdAuditTable"></tbody>
                    </table>
                </div>
            </section>

            <!-- VIEW 8: DATENMODELL & SYSTEM (Twenty Schema Builder) -->
            <section id="view-settings" class="view-padded" style="display:none;">
                <div style="display:grid; grid-template-columns: 1fr 1fr; gap:16px;">
                    <div class="card-panel" style="padding:16px;">
                        <h3 style="font-size:13px; font-weight:600; margin-bottom:12px;">Workspace &amp; Erscheinungsbild</h3>
                        <div class="form-group">
                            <label class="form-label">Mandantenname</label>
                            <div style="display:flex; gap:8px;">
                                <input type="text" id="settingCompanyName" class="form-input" value="Conrad KI">
                                <button class="btn" onclick="saveCompanyNameSetting()">Speichern</button>
                            </div>
                        </div>
                        <div class="form-group" style="margin-top:14px;">
                            <label class="form-label">Markenakzent</label>
                            <div style="display:flex; gap:6px;">
                                <button class="btn" onclick="setThemeAccent('#2563eb')">Twenty Blue</button>
                                <button class="btn" onclick="setThemeAccent('#0f766e')">Teal</button>
                                <button class="btn" onclick="setThemeAccent('#c8531a')">HubSpot Orange</button>
                                <button class="btn" onclick="setThemeAccent('#4f46e5')">Indigo</button>
                            </div>
                        </div>
                    </div>

                    <div class="card-panel" style="padding:16px;">
                        <h3 style="font-size:13px; font-weight:600; margin-bottom:12px;">Custom Fields (Datenmodell-Erweiterung)</h3>
                        <div style="display:flex; gap:8px; margin-bottom:12px;">
                            <input type="text" id="newCfKey" class="form-input" placeholder="Key (z.B. branche)">
                            <input type="text" id="newCfLabel" class="form-input" placeholder="Label (z.B. Branche)">
                            <button class="btn btn-primary" onclick="submitCustomField()">+ Feld</button>
                        </div>
                        <div id="customFieldsRenderList" style="font-size:12px; color:var(--text-secondary);"></div>
                    </div>
                </div>

                <div class="card-panel" style="padding:16px;">
                    <h3 style="font-size:13px; font-weight:600; margin-bottom:10px;">Aktive Schnittstellen, Webhooks &amp; Automations-Regeln</h3>
                    <div style="display:grid; grid-template-columns:repeat(3, 1fr); gap:12px; font-size:12px;">
                        <div style="padding:12px; border:1px solid var(--border-color); border-radius:6px; background:var(--bg-subtle);">
                            <div style="font-weight:600; margin-bottom:4px;">Model Context Protocol (MCP)</div>
                            <code class="mono-val">POST /mcp (JSON-RPC 2.0)</code>
                        </div>
                        <div style="padding:12px; border:1px solid var(--border-color); border-radius:6px; background:var(--bg-subtle);">
                            <div style="font-weight:600; margin-bottom:4px;">WhatsApp &amp; E-Mail Inbound</div>
                            <code class="mono-val">POST /api/webhooks/whatsapp</code>
                        </div>
                        <div style="padding:12px; border:1px solid var(--border-color); border-radius:6px; background:var(--bg-subtle);">
                            <div style="font-weight:600; margin-bottom:4px;">Pipeline Automation</div>
                            <span style="color:var(--text-secondary);">Deal → Angebot = +Task (3T) &amp; E-Mail-Entwurf</span>
                        </div>
                    </div>
                </div>
            </section>

        </div>
    </div>

    <!-- ── HubSpot / Twenty 360° Contact Record Inspector Drawer ── -->
    <div class="record-drawer-backdrop" id="contactInspectorDrawer" onclick="if(event.target===this) closeContactDrawer()">
        <div class="record-drawer">
            <div class="drawer-header">
                <div class="drawer-entity-head">
                    <div class="drawer-avatar-lg" id="cd-avatar">JK</div>
                    <div>
                        <div style="font-size:15px; font-weight:700;" id="cd-title-name">Kontakt</div>
                        <div style="font-size:12px; color:var(--text-secondary);" id="cd-title-sub">Unternehmen</div>
                    </div>
                </div>
                <div style="display:flex; gap:8px;">
                    <button class="btn" onclick="openNewDealModalForContact(activeContactId)">+ Deal</button>
                    <button class="modal-close-btn" onclick="closeContactDrawer()">✕</button>
                </div>
            </div>

            <div class="drawer-body">
                <!-- Left Properties Column -->
                <div class="drawer-props-pane">
                    <div style="font-size:11px; font-weight:700; text-transform:uppercase; color:var(--text-muted); margin-bottom:12px;">Eigenschaften</div>
                    <div class="prop-row">
                        <label class="prop-label">Name</label>
                        <input type="text" id="cd-prop-name" class="prop-input" onchange="saveContactProperty()">
                    </div>
                    <div class="prop-row">
                        <label class="prop-label">Unternehmen</label>
                        <input type="text" id="cd-prop-company" class="prop-input" onchange="saveContactProperty()">
                    </div>
                    <div class="prop-row">
                        <label class="prop-label">E-Mail</label>
                        <input type="email" id="cd-prop-email" class="prop-input" onchange="saveContactProperty()">
                    </div>
                    <div class="prop-row">
                        <label class="prop-label">Telefon / WhatsApp</label>
                        <input type="text" id="cd-prop-phone" class="prop-input" onchange="saveContactProperty()">
                    </div>
                    <div class="prop-row">
                        <label class="prop-label">Lifecycle Status</label>
                        <select id="cd-prop-status" class="prop-input" onchange="saveContactProperty()">
                            <option value="lead">Lead</option>
                            <option value="qualified">Qualifiziert</option>
                            <option value="customer">Kunde</option>
                            <option value="churned">Inaktiv</option>
                        </select>
                    </div>
                    <div class="prop-row">
                        <label class="prop-label">Tags</label>
                        <input type="text" id="cd-prop-tags" class="prop-input" onchange="saveContactProperty()">
                    </div>

                    <div style="font-size:11px; font-weight:700; text-transform:uppercase; color:var(--text-muted); margin:18px 0 8px;">Verknüpfte Deals</div>
                    <div id="cd-deals-list" style="display:flex; flex-direction:column; gap:6px; font-size:12px;"></div>
                </div>

                <!-- Right Activity & Timeline Column -->
                <div class="drawer-activity-pane">
                    <div class="composer-tabs">
                        <button class="composer-tab active" id="tab-btn-note" onclick="setComposerChannel('note')">Notiz</button>
                        <button class="composer-tab" id="tab-btn-email" onclick="setComposerChannel('email')">E-Mail-Entwurf</button>
                        <button class="composer-tab" id="tab-btn-whatsapp" onclick="setComposerChannel('whatsapp')">WhatsApp</button>
                        <button class="composer-tab" id="tab-btn-call" onclick="setComposerChannel('call')">Anruf-Log</button>
                    </div>
                    <div class="composer-box">
                        <input type="text" id="composerSubject" class="form-input" style="display:none; margin-bottom:6px; height:28px; font-size:12px;" placeholder="Betreff der E-Mail...">
                        <textarea id="composerBody" class="form-input" style="height:58px; padding:8px; resize:none;" placeholder="Notiz oder Aktivität zu diesem Datensatz erfassen..."></textarea>
                        <div style="display:flex; justify-content:space-between; align-items:center; margin-top:8px;">
                            <span style="font-size:11px; color:var(--text-muted);" id="composerHint">Wird revisionssicher in der Timeline gespeichert</span>
                            <button class="btn btn-primary" onclick="submitContactActivity()">Speichern</button>
                        </div>
                    </div>
                    <div class="timeline-feed" id="cd-timeline-feed"></div>
                </div>
            </div>
        </div>
    </div>

    <!-- ── AI Copilot Panel ── -->
    <aside class="copilot-panel" id="copilotPanel">
        <div class="drawer-header">
            <div>
                <div style="font-size:13px; font-weight:600;" id="copilotPanelTitle">Conrad Copilot</div>
                <div style="font-size:11px; color:var(--text-muted);" id="copilotPanelSub">QWEN / HERMES · MCP CONNECTED</div>
            </div>
            <button class="modal-close-btn" onclick="toggleCopilot()">✕</button>
        </div>

        <div class="copilot-messages" id="copilotMessages">
            <div class="chat-bubble ai">
                <strong>Bereit.</strong> Frage nach Pipeline-Kennzahlen, lege Kontakte per Spracheingabe an oder steuere das System:<br><br>
                • „Wie steht unsere Pipeline heute?“<br>
                • „Trag Stefan Koch von Koch Sanitär ein“<br>
                • „Wie hoch ist unser aktueller ROAS?“
            </div>
        </div>

        <div style="padding:12px; border-top:1px solid var(--border-color); display:flex; gap:8px; background:var(--bg-surface);">
            <input type="text" id="copilotInput" class="form-input" placeholder="Befehl oder Frage..." onkeydown="if(event.key==='Enter') sendCopilotChat()">
            <button class="btn btn-primary" id="copilotSendBtn" onclick="sendCopilotChat()">Senden</button>
        </div>
    </aside>

    <!-- ── Cmd+K Command Palette Modal ── -->
    <div class="cmdk-backdrop" id="cmdkModal" onclick="if(event.target===this) closeCmdKModal()">
        <div class="cmdk-modal">
            <div class="cmdk-input-row">
                <svg class="icon" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                <input type="text" id="spotlightInput" class="cmdk-input" placeholder="Suche nach Kontakten, Deals, Rechnungen oder gib einen KI-Befehl ein..." oninput="renderSpotlightPreview()" onkeydown="if(event.key==='Enter') executeSpotlightCommand()">
                <span class="kbd-tag">ESC</span>
            </div>
            <div id="spotlightPreviewResults" style="max-height:320px; overflow-y:auto; padding:8px;"></div>
        </div>
    </div>

    <!-- ── Modal: Workspace & Role Switcher ── -->
    <div class="modal-backdrop" id="loginModal">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">Benutzerrolle &amp; Session</span>
                <button class="modal-close-btn" onclick="closeModal('loginModal')">✕</button>
            </div>
            <div style="display:flex; flex-direction:column; gap:8px; margin-bottom:16px;">
                <div style="border:1px solid var(--border-color); border-radius:6px; padding:12px; cursor:pointer;" onclick="setDemoUser('admin'); closeModal('loginModal');">
                    <div style="display:flex; justify-content:space-between;">
                        <strong>Merlin Conrad (Administrator)</strong>
                        <span class="badge warning">Architekt</span>
                    </div>
                    <div style="font-size:12px; color:var(--text-secondary); margin-top:4px;">admin@conrad-ki.com · Vollzugriff auf Schema &amp; GoBD</div>
                </div>
                <div style="border:1px solid var(--border-color); border-radius:6px; padding:12px; cursor:pointer;" onclick="setDemoUser('team'); closeModal('loginModal');">
                    <div style="display:flex; justify-content:space-between;">
                        <strong>Operations Team</strong>
                        <span class="badge info">Operativ</span>
                    </div>
                    <div style="font-size:12px; color:var(--text-secondary); margin-top:4px;">team@conrad-ki.com · Vertrieb, Kontakte &amp; Faktura</div>
                </div>
            </div>
            <div style="display:none;" id="demoAccountLabel">admin@conrad-ki.com</div>
            <div style="text-align:right;">
                <button class="btn" onclick="closeModal('loginModal')">Schließen</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: Neuer Kontakt ── -->
    <div class="modal-backdrop" id="modalNewContact">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">Neuen Kontakt anlegen</span>
                <button class="modal-close-btn" onclick="closeModal('modalNewContact')">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Vollständiger Name *</label>
                <input type="text" id="nc-name" class="form-input" placeholder="z.B. Stefan Koch">
            </div>
            <div class="form-group">
                <label class="form-label">Unternehmen</label>
                <input type="text" id="nc-company" class="form-input" placeholder="z.B. Koch Sanitär GmbH">
            </div>
            <div class="form-group">
                <label class="form-label">E-Mail</label>
                <input type="email" id="nc-email" class="form-input" placeholder="stefan@koch-sanitaer.de">
            </div>
            <div class="form-group">
                <label class="form-label">Telefon</label>
                <input type="tel" id="nc-phone" class="form-input" placeholder="+49 171 1234567">
            </div>
            <div class="form-group">
                <label class="form-label">Lifecycle Phase</label>
                <select id="nc-status" class="form-input">
                    <option value="lead">Lead</option>
                    <option value="qualified">Qualifiziert</option>
                    <option value="customer">Kunde</option>
                </select>
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:16px;">
                <button class="btn" onclick="closeModal('modalNewContact')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateContact()">Speichern</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: Neuer Deal ── -->
    <div class="modal-backdrop" id="modalNewDeal">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">Neuen Deal anlegen</span>
                <button class="modal-close-btn" onclick="closeModal('modalNewDeal')">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Kontakt / Unternehmen *</label>
                <select id="nd-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Deal-Bezeichnung *</label>
                <input type="text" id="nd-title" class="form-input" placeholder="z.B. KI-Postfachtriage &amp; CRM Setup">
            </div>
            <div class="form-group">
                <label class="form-label">Volumen (EUR Netto) *</label>
                <input type="number" id="nd-val" class="form-input" value="890">
            </div>
            <div class="form-group">
                <label class="form-label">Pipeline-Phase</label>
                <select id="nd-stage" class="form-input">
                    <option value="discovery">1. Erstkontakt</option>
                    <option value="proposal">2. Angebot</option>
                    <option value="negotiation">3. Verhandlung</option>
                    <option value="closed_won">4. Gewonnen</option>
                </select>
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:16px;">
                <button class="btn" onclick="closeModal('modalNewDeal')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateDeal()">Anlegen</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: Neue Rechnung ── -->
    <div class="modal-backdrop" id="modalNewInvoice">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">GoBD-Rechnung erstellen</span>
                <button class="modal-close-btn" onclick="closeModal('modalNewInvoice')">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Rechnungsempfänger *</label>
                <select id="ni-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Leistungsbeschreibung *</label>
                <input type="text" id="ni-desc" class="form-input" placeholder="z.B. Einrichtung KI-Automatisierung">
            </div>
            <div class="form-group">
                <label class="form-label">Nettobetrag (EUR) *</label>
                <input type="number" id="ni-amount" class="form-input" value="99">
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:16px;">
                <button class="btn" onclick="closeModal('modalNewInvoice')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateInvoice()">Buchen</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: GiroCode (EPC-QR) ── -->
    <div class="modal-backdrop" id="modalEpcQr">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">SEPA GiroCode (EPC-QR)</span>
                <button class="modal-close-btn" onclick="closeModal('modalEpcQr')">✕</button>
            </div>
            <div style="font-family:var(--font-mono); font-size:11px; background:var(--bg-subtle); padding:12px; border-radius:6px; border:1px solid var(--border-color); margin-bottom:12px;" id="epcQrCodeString"></div>
            <div id="epcDetailsBox" style="font-size:12px; color:var(--text-secondary); line-height:1.6;"></div>
            <div style="text-align:right; margin-top:14px;">
                <button class="btn" onclick="closeModal('modalEpcQr')">Schließen</button>
            </div>
        </div>
    </div>

    <script>
        let currentMode = 'user';
        let currentRole = 'admin';
        let currentView = 'dashboard';
        let contactsData = [];
        let dealsData = [];
        let invoicesData = [];
        let tasksData = [];
        let settingsData = {};
        let contactStatusFilter = 'all';
        let activeContactId = null;
        let activeComposerChannel = 'note';
        let touchDraggedDealId = null;

        async function initApp() {
            if ('serviceWorker' in navigator) {
                navigator.serviceWorker.register('/sw.js').catch(() => {});
            }
            await loadSettings();
            await refreshData();
            setupKeyboardNav();
        }

        function initials(name) {
            if (!name) return 'C';
            return name.split(' ').map(p => p[0]).join('').substring(0, 2).toUpperCase();
        }

        function statusBadge(status) {
            const map = {
                customer: ['success', 'Kunde'],
                qualified: ['info', 'Qualifiziert'],
                lead: ['warning', 'Lead'],
                churned: ['neutral', 'Inaktiv'],
                paid: ['success', 'Bezahlt'],
                sent: ['info', 'Versendet'],
                draft: ['neutral', 'Entwurf'],
                cancelled: ['danger', 'Storniert']
            };
            const [cls, label] = map[status] || ['neutral', status];
            return `<span class="badge ${cls}"><span class="badge-dot"></span>${escapeHtml(label)}</span>`;
        }

        function switchAppMode(mode) {
            currentMode = mode;
            document.getElementById('btnModeUser').classList.toggle('active', mode === 'user');
            document.getElementById('btnModeAdmin').classList.toggle('active', mode === 'admin');
            const pill = document.getElementById('topbarModePill');
            const label = document.getElementById('topbarModeLabel');
            if (mode === 'admin') {
                pill.className = 'badge warning';
                label.innerText = 'Architekt';
            } else {
                pill.className = 'badge info';
                label.innerText = 'Operativ';
            }
        }

        function switchView(viewId) {
            currentView = viewId;
            document.getElementById('appSidebar')?.classList.remove('mobile-open');
            const views = ['dashboard', 'contacts', 'deals', 'invoices', 'retainers', 'marketing', 'dms', 'settings'];
            views.forEach(v => {
                const el = document.getElementById(`view-${v}`);
                if (el) el.style.display = v === viewId ? 'block' : 'none';
                const btn = document.getElementById(`nav-btn-${v}`);
                if (btn) btn.classList.toggle('active', v === viewId);
            });

            const titles = {
                dashboard: 'Cockpit & Aufgaben',
                contacts: 'Personen & Firmen',
                deals: 'Deals Pipeline',
                invoices: 'Faktura & E-Rechnung',
                retainers: 'Retainer & Abos',
                marketing: 'Marketing Attribution',
                dms: 'GoBD Audit Journal',
                settings: 'Datenmodell & System'
            };
            document.getElementById('currentViewTitle').innerText = titles[viewId] || 'Workspace';

            if (viewId === 'marketing') loadMarketingView();
            if (viewId === 'retainers') loadRetainersView();
            if (viewId === 'dms') loadGobdAuditView();
            if (viewId === 'settings') loadSettingsView();
        }

        async function refreshData() {
            try {
                const [statsRes, contacts, deals, invoices, tasks] = await Promise.all([
                    fetch('/mcp', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'tools/call', params: { name: 'erp_get_cashflow_stats' } })
                    }).then(r => r.json()),
                    fetch('/api/contacts').then(r => r.json()),
                    fetch('/api/deals').then(r => r.json()),
                    fetch('/api/invoices').then(r => r.json()),
                    fetch('/api/tasks').then(r => r.json()).catch(() => [])
                ]);

                contactsData = contacts || [];
                dealsData = deals || [];
                invoicesData = invoices || [];
                tasksData = tasks || [];

                document.getElementById('badge-contacts-count').innerText = contactsData.length;
                document.getElementById('badge-deals-count').innerText = dealsData.length;

                if (statsRes?.result?.content?.[0]?.text) {
                    const stats = JSON.parse(statsRes.result.content[0].text);
                    document.getElementById('kpi-pipeline').innerText = (stats.pipeline_volume_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-revenue').innerText = (stats.paid_revenue_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-pending').innerText = (stats.pending_invoices_cents / 100).toLocaleString('de-DE', { style: 'currency', currency: 'EUR' });
                    document.getElementById('kpi-contacts').innerText = stats.total_contacts;
                }

                renderDashboardTables();
                renderContactsTable();
                renderDealsKanban();
                renderInvoicesTable();
            } catch (err) {
                console.error("Refresh error:", err);
            }
        }

        function renderDashboardTables() {
            const tbody = document.getElementById('dashboardRecentContacts');
            tbody.innerHTML = contactsData.slice(0, 6).map(c => `
                <tr onclick="openContactDrawer('${c.id}')">
                    <td>
                        <div class="cell-entity">
                            <span class="entity-avatar">${initials(c.name)}</span>
                            <span>${escapeHtml(c.name)}</span>
                        </div>
                    </td>
                    <td style="color:var(--text-secondary);">${escapeHtml(c.company || '—')}</td>
                    <td>${statusBadge(c.status)}</td>
                    <td class="mono-val" style="color:var(--text-muted);">${new Date(c.created_at).toLocaleDateString('de-DE')}</td>
                </tr>
            `).join('');

            const openTasks = tasksData.filter(t => t.status !== 'completed');
            document.getElementById('tasksCountBadge').innerText = `${openTasks.length} offen`;
            const tBox = document.getElementById('dashboardTasksList');
            if (!tasksData.length) {
                tBox.innerHTML = `<div style="color:var(--text-muted); font-size:12px;">Keine offenen Aufgaben. Verschiebe einen Deal auf „Angebot“, um automatisch einen Follow-up auszulösen.</div>`;
            } else {
                tBox.innerHTML = tasksData.slice(0, 6).map(t => `
                    <div style="display:flex; align-items:center; justify-content:space-between; padding:8px 10px; border:1px solid var(--border-color); border-radius:6px; background:var(--bg-subtle);">
                        <label style="display:flex; align-items:center; gap:8px; cursor:pointer; font-size:12px; ${t.status==='completed'?'text-decoration:line-through; color:var(--text-muted);':''}">
                            <input type="checkbox" ${t.status==='completed'?'checked':''} onchange="toggleTask('${t.id}')">
                            <span>${escapeHtml(t.title)}</span>
                        </label>
                        <span class="mono-val" style="color:var(--text-muted); font-size:11px;">${escapeHtml(t.due_date)}</span>
                    </div>
                `).join('');
            }
        }

        async function toggleTask(id) {
            await fetch(`/api/tasks/${id}/toggle`, { method: 'PATCH' });
            await refreshData();
        }

        function setContactStatusFilter(status) {
            contactStatusFilter = status;
            ['all', 'lead', 'qualified', 'customer'].forEach(s => {
                document.getElementById(`flt-${s}`)?.classList.toggle('active', s === status);
            });
            renderContactsTable();
        }

        function renderContactsTable() {
            const tbody = document.getElementById('contactsFullTable');
            const q = (document.getElementById('contactSearchInput')?.value || '').toLowerCase();
            const filtered = contactsData.filter(c => {
                if (contactStatusFilter !== 'all' && c.status !== contactStatusFilter) return false;
                if (!q) return true;
                return (c.name || '').toLowerCase().includes(q) ||
                       (c.company || '').toLowerCase().includes(q) ||
                       (c.email || '').toLowerCase().includes(q);
            });

            if (!filtered.length) {
                tbody.innerHTML = '<tr><td colspan="7" style="text-align:center; color:var(--text-muted);">Keine passenden Datensätze.</td></tr>';
                return;
            }

            tbody.innerHTML = filtered.map(c => {
                const tagsHtml = (c.tags || '').split(',').filter(Boolean).map(t => `<span class="tag-chip">${escapeHtml(t.trim())}</span>`).join('');
                return `
                    <tr onclick="openContactDrawer('${c.id}')">
                        <td>
                            <div class="cell-entity">
                                <span class="entity-avatar">${initials(c.name)}</span>
                                <span>${escapeHtml(c.name)}</span>
                            </div>
                        </td>
                        <td>${escapeHtml(c.company || '—')}</td>
                        <td class="mono-val" style="color:var(--text-secondary);">${escapeHtml(c.email || '—')}</td>
                        <td class="mono-val" style="color:var(--text-secondary);">${escapeHtml(c.phone || '—')}</td>
                        <td>${statusBadge(c.status)}</td>
                        <td>${tagsHtml || '—'}</td>
                        <td onclick="event.stopPropagation();">
                            <button class="btn" style="height:24px; padding:0 8px; font-size:11px;" onclick="openContactDrawer('${c.id}')">Akte öffnen</button>
                        </td>
                    </tr>
                `;
            }).join('');
        }

        function filterContactsList() {
            renderContactsTable();
        }

        // ── 360° Contact Record Drawer (HubSpot / Twenty Inspector) ──
        async function openContactDrawer(contactId) {
            const c = contactsData.find(x => x.id === contactId);
            if (!c) return;
            activeContactId = contactId;

            document.getElementById('cd-avatar').innerText = initials(c.name);
            document.getElementById('cd-title-name').innerText = c.name;
            document.getElementById('cd-title-sub').innerText = c.company || c.email || 'Kontakt-Datensatz';

            document.getElementById('cd-prop-name').value = c.name || '';
            document.getElementById('cd-prop-company').value = c.company || '';
            document.getElementById('cd-prop-email').value = c.email || '';
            document.getElementById('cd-prop-phone').value = c.phone || '';
            document.getElementById('cd-prop-status').value = c.status || 'lead';
            document.getElementById('cd-prop-tags').value = c.tags || '';

            const cDeals = dealsData.filter(d => d.contact_id === contactId);
            const dealsBox = document.getElementById('cd-deals-list');
            dealsBox.innerHTML = cDeals.length ? cDeals.map(d => `
                <div style="padding:8px; background:var(--bg-surface); border:1px solid var(--border-color); border-radius:6px;">
                    <div style="font-weight:600;">${escapeHtml(d.title)}</div>
                    <div style="display:flex; justify-content:space-between; margin-top:4px; color:var(--text-secondary);">
                        <span class="mono-val">${(d.value_cents/100).toLocaleString('de-DE',{style:'currency',currency:'EUR'})}</span>
                        <span class="badge neutral">${escapeHtml(d.stage)}</span>
                    </div>
                </div>
            `).join('') : '<div style="color:var(--text-muted);">Keine verknüpften Deals</div>';

            document.getElementById('contactInspectorDrawer').classList.add('open');
            await loadContactTimeline(contactId);
        }

        function closeContactDrawer() {
            document.getElementById('contactInspectorDrawer').classList.remove('open');
        }

        async function saveContactProperty() {
            if (!activeContactId) return;
            await fetch(`/api/contacts/${activeContactId}`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name: document.getElementById('cd-prop-name').value.trim(),
                    company: document.getElementById('cd-prop-company').value.trim(),
                    email: document.getElementById('cd-prop-email').value.trim(),
                    phone: document.getElementById('cd-prop-phone').value.trim(),
                    status: document.getElementById('cd-prop-status').value,
                    tags: document.getElementById('cd-prop-tags').value.trim()
                })
            });
            await refreshData();
        }

        function setComposerChannel(ch) {
            activeComposerChannel = ch;
            ['note', 'email', 'whatsapp', 'call'].forEach(t => {
                document.getElementById(`tab-btn-${t}`)?.classList.toggle('active', t === ch);
            });
            const subj = document.getElementById('composerSubject');
            const hint = document.getElementById('composerHint');
            if (ch === 'email') {
                subj.style.display = 'block';
                hint.innerText = 'Sicherheits-Guardrail: E-Mails werden immer als Entwurf (draft) gespeichert.';
            } else {
                subj.style.display = 'none';
                hint.innerText = `Aktivität (${ch.toUpperCase()}) wird direkt in der Kunden-Historie protokolliert.`;
            }
        }

        async function submitContactActivity() {
            if (!activeContactId) return;
            const body = document.getElementById('composerBody').value.trim();
            if (!body) return;
            const subject = document.getElementById('composerSubject').value.trim() || null;

            await fetch(`/api/contacts/${activeContactId}/timeline`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    contact_id: activeContactId,
                    channel: activeComposerChannel,
                    direction: 'outbound',
                    subject,
                    body
                })
            });
            document.getElementById('composerBody').value = '';
            document.getElementById('composerSubject').value = '';
            await loadContactTimeline(activeContactId);
        }

        async function loadContactTimeline(contactId) {
            const feed = document.getElementById('cd-timeline-feed');
            const list = await fetch(`/api/contacts/${contactId}/timeline`).then(r => r.json()).catch(() => []);
            if (!list.length) {
                feed.innerHTML = '<div style="color:var(--text-muted); text-align:center; padding-top:24px;">Noch keine Aktivitäten für diesen Kontakt erfasst.</div>';
                return;
            }
            feed.innerHTML = list.map(m => {
                const isDraft = m.status === 'draft';
                return `
                    <div class="timeline-item">
                        <div class="timeline-item-head">
                            <div>
                                <span class="badge ${isDraft ? 'warning' : 'info'}">${escapeHtml(m.channel.toUpperCase())}</span>
                                ${isDraft ? '<span class="badge warning" style="margin-left:4px;">ENTWURF (Freigabe nötig)</span>' : ''}
                                ${m.subject ? `<strong style="margin-left:6px; color:var(--text-primary);">${escapeHtml(m.subject)}</strong>` : ''}
                            </div>
                            <span class="mono-val">${new Date(m.timestamp).toLocaleString('de-DE')}</span>
                        </div>
                        <div style="white-space:pre-wrap; color:var(--text-primary); font-size:12.5px;">${escapeHtml(m.body)}</div>
                    </div>
                `;
            }).join('');
        }

        // ── Deals Kanban Board ──
        function renderDealsKanban() {
            const stages = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            const sums = { discovery: 0, proposal: 0, negotiation: 0, closed_won: 0 };

            stages.forEach(s => {
                const col = document.getElementById(`k-items-${s}`);
                if (col) col.innerHTML = '';
                const colWrap = document.getElementById(`k-col-${s}`);
                if (colWrap) {
                    colWrap.dataset.stage = s;
                    colWrap.ondragover = (e) => { e.preventDefault(); colWrap.style.borderColor = 'var(--accent)'; };
                    colWrap.ondragleave = () => { colWrap.style.borderColor = 'var(--border-color)'; };
                    colWrap.ondrop = async (e) => {
                        e.preventDefault();
                        colWrap.style.borderColor = 'var(--border-color)';
                        const dealId = e.dataTransfer.getData('text/plain');
                        if (dealId) await moveDealToStage(dealId, s);
                    };
                }
            });

            dealsData.forEach(d => {
                const stage = sums[d.stage] !== undefined ? d.stage : 'discovery';
                sums[stage] += d.value_cents;

                const col = document.getElementById(`k-items-${stage}`);
                if (col) {
                    const card = document.createElement('div');
                    card.className = 'kanban-card';
                    card.draggable = true;
                    card.ondragstart = (e) => { e.dataTransfer.setData('text/plain', d.id); card.style.opacity = '0.5'; };
                    card.ondragend = () => { card.style.opacity = '1'; };

                    card.addEventListener('touchstart', () => { touchDraggedDealId = d.id; card.style.opacity = '0.7'; }, { passive: true });
                    card.addEventListener('touchend', async (e) => {
                        card.style.opacity = '1';
                        if (!touchDraggedDealId) return;
                        const touch = e.changedTouches[0];
                        const targetEl = document.elementFromPoint(touch.clientX, touch.clientY);
                        const targetCol = targetEl ? targetEl.closest('.kanban-column') : null;
                        if (targetCol && targetCol.dataset.stage && targetCol.dataset.stage !== d.stage) {
                            await moveDealToStage(touchDraggedDealId, targetCol.dataset.stage);
                        }
                        touchDraggedDealId = null;
                    });

                    card.onclick = () => advanceDealStage(d.id, d.stage);
                    card.innerHTML = `
                        <div class="kanban-card-title">${escapeHtml(d.title)}</div>
                        <div class="kanban-card-company">
                            <span class="entity-avatar" style="width:18px; height:18px; font-size:9px;">${initials(d.contact_name || 'K')}</span>
                            <span>${escapeHtml(d.contact_name || 'Unbekannt')}</span>
                        </div>
                        <div class="kanban-card-footer">
                            <span class="mono-val" style="font-weight:700; color:var(--text-primary);">${(d.value_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</span>
                            <span style="color:var(--text-muted);">Phase +1 →</span>
                        </div>
                    `;
                    col.appendChild(card);
                }
            });

            stages.forEach(s => {
                const sumEl = document.getElementById(`k-sum-${s}`);
                if (sumEl) sumEl.innerText = (sums[s] / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'});
            });
        }

        async function moveDealToStage(dealId, targetStage) {
            await fetch('/mcp', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    jsonrpc: '2.0',
                    id: 1,
                    method: 'tools/call',
                    params: { name: 'crm_update_deal_stage', arguments: { deal_id: dealId, new_stage: targetStage } }
                })
            });
            await refreshData();
        }

        async function advanceDealStage(dealId, currentStage) {
            const flow = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            const idx = flow.indexOf(currentStage);
            if (idx === -1 || idx === flow.length - 1) return;
            await moveDealToStage(dealId, flow[idx + 1]);
        }

        // ── Invoices ──
        function renderInvoicesTable() {
            const tbody = document.getElementById('invoicesFullTable');
            if (!invoicesData.length) {
                tbody.innerHTML = '<tr><td colspan="7" style="text-align:center; color:var(--text-muted);">Keine Rechnungen.</td></tr>';
                return;
            }
            tbody.innerHTML = invoicesData.map(inv => {
                const isPaid = inv.status === 'paid';
                return `
                    <tr>
                        <td class="mono-val"><strong>${escapeHtml(inv.invoice_number)}</strong></td>
                        <td>${escapeHtml(inv.contact_name || inv.contact_id)}</td>
                        <td class="mono-val">${escapeHtml(inv.due_date)}</td>
                        <td class="mono-val">${(inv.net_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                        <td class="mono-val"><strong>${(inv.gross_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</strong></td>
                        <td>${statusBadge(inv.status)}</td>
                        <td>
                            <div style="display:flex; gap:6px;">
                                <button class="btn" style="height:24px; padding:0 8px; font-size:11px;" onclick="toggleInvoiceStatus('${inv.id}', '${inv.status}')">
                                    ${isPaid ? 'Bezahlt ✓' : 'Als bezahlt buchen'}
                                </button>
                                <a href="/api/invoices/${inv.id}/xrechnung" target="_blank" class="btn" style="height:24px; padding:0 8px; font-size:11px; text-decoration:none;">XRechnung XML</a>
                                <button class="btn" style="height:24px; padding:0 8px; font-size:11px;" onclick="openEpcModal('${inv.invoice_number}', ${inv.gross_cents})">GiroCode</button>
                            </div>
                        </td>
                    </tr>
                `;
            }).join('');
        }

        async function toggleInvoiceStatus(id, currentStatus) {
            const nextStatus = currentStatus === 'paid' ? 'sent' : 'paid';
            const res = await fetch(`/api/invoices/${id}/status`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ status: nextStatus })
            });
            if (!res.ok) alert(await res.text());
            await refreshData();
        }

        function openEpcModal(invNum, grossCents) {
            const eur = (grossCents / 100).toFixed(2);
            document.getElementById('epcQrCodeString').innerText = `BCD\\n002\\n1\\nSCT\\n\\nConrad KI\\nDE02100100100123456789\\nEUR${eur}\\n\\n\\n${invNum}`;
            document.getElementById('epcDetailsBox').innerHTML = `
                <div><strong>Empfänger:</strong> Conrad KI</div>
                <div><strong>Betrag:</strong> ${eur} € (0,00 % Gebühr)</div>
                <div><strong>Referenz:</strong> ${invNum}</div>
            `;
            openModal('modalEpcQr');
        }

        async function loadRetainersView() {
            const list = await fetch('/api/retainers').then(r => r.json()).catch(() => []);
            const tbody = document.getElementById('retainersFullTable');
            tbody.innerHTML = list.map(r => `
                <tr>
                    <td><strong>${escapeHtml(r.contact_name || r.contact_id)}</strong></td>
                    <td>${escapeHtml(r.plan_name)}</td>
                    <td class="mono-val"><strong>${(r.amount_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</strong></td>
                    <td>${r.interval_months} Monat(e)</td>
                    <td class="mono-val">${escapeHtml(r.next_billing_date)}</td>
                    <td><span class="badge ${r.active ? 'success' : 'neutral'}">${r.active ? 'Aktiv' : 'Pausiert'}</span></td>
                </tr>
            `).join('');
        }

        async function triggerRetainerRun() {
            const res = await fetch('/api/retainers/trigger-run', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ actor: document.getElementById('ui-user-name').innerText })
            }).then(r => r.json());
            alert(res.summary);
            await loadRetainersView();
            await refreshData();
        }

        async function loadMarketingView() {
            const [campaigns, roi] = await Promise.all([
                fetch('/api/marketing/campaigns').then(r => r.json()),
                fetch('/api/marketing/roi').then(r => r.json())
            ]);
            if (roi) {
                document.getElementById('m-kpi-spend').innerText = roi.total_spend_eur.toLocaleString('de-DE', {style:'currency', currency:'EUR'});
                document.getElementById('m-kpi-leads').innerText = roi.total_leads;
                document.getElementById('m-kpi-cac').innerText = roi.blended_cac_eur.toLocaleString('de-DE', {style:'currency', currency:'EUR'});
                document.getElementById('m-kpi-roas').innerText = `${roi.roas.toFixed(1)}x`;
            }
            document.getElementById('marketingCampaignsTable').innerHTML = (campaigns || []).map(c => `
                <tr>
                    <td><strong>${escapeHtml(c.name)}</strong></td>
                    <td><span class="badge neutral">${escapeHtml(c.platform.toUpperCase())}</span></td>
                    <td class="mono-val">${(c.spend_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                    <td class="mono-val">${c.clicks}</td>
                    <td class="mono-val">${c.leads}</td>
                    <td class="mono-val"><strong>${c.customers}</strong></td>
                    <td class="mono-val">${(c.revenue_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                    <td><span class="badge success">${escapeHtml(c.status)}</span></td>
                </tr>
            `).join('');
        }

        async function loadGobdAuditView() {
            const list = await fetch('/api/gobd/audit-log').then(r => r.json()).catch(() => []);
            document.getElementById('gobdAuditTable').innerHTML = list.map(entry => `
                <tr>
                    <td class="mono-val">#${entry.seq}</td>
                    <td class="mono-val" style="color:var(--text-secondary);">${new Date(entry.timestamp).toLocaleString('de-DE')}</td>
                    <td><span class="badge neutral">${escapeHtml(entry.action)}</span></td>
                    <td class="mono-val" style="color:var(--text-muted);">${escapeHtml((entry.entity_id || '').substring(0, 12))}</td>
                    <td>${escapeHtml(entry.details)}</td>
                    <td><strong>${escapeHtml(entry.actor)}</strong></td>
                </tr>
            `).join('');
        }

        async function verifyGobdChain() {
            const res = await fetch('/api/gobd/verify').then(r => r.json());
            const box = document.getElementById('gobdVerifyResult');
            box.style.display = 'block';
            box.style.background = res.valid ? 'var(--status-green-bg)' : 'var(--status-red-bg)';
            box.style.color = res.valid ? 'var(--status-green-fg)' : 'var(--status-red-fg)';
            box.innerHTML = `<strong>${res.message}</strong> (${res.total_entries} Blöcke verifiziert)`;
        }

        async function loadSettings() {
            try {
                settingsData = await fetch('/api/settings').then(r => r.json()) || {};
                if (settingsData.company_name) {
                    document.getElementById('ui-brand-name').innerText = settingsData.company_name;
                    document.getElementById('settingCompanyName').value = settingsData.company_name;
                }
                if (settingsData.theme_accent) setThemeAccent(settingsData.theme_accent, false);
                if (settingsData.theme_mode) {
                    document.documentElement.setAttribute('data-theme', settingsData.theme_mode);
                }
            } catch (_) {}
        }

        async function saveCompanyNameSetting() {
            const name = document.getElementById('settingCompanyName').value.trim();
            if (!name) return;
            await fetch('/api/settings', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ key: 'company_name', value: name })
            });
            document.getElementById('ui-brand-name').innerText = name;
        }

        function setThemeAccent(hex, persist = true) {
            document.documentElement.style.setProperty('--accent', hex);
            document.documentElement.style.setProperty('--accent-hover', hex);
            if (persist) {
                fetch('/api/settings', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ key: 'theme_accent', value: hex })
                });
            }
        }

        function toggleTheme() {
            const current = document.documentElement.getAttribute('data-theme') || 'light';
            const next = current === 'dark' ? 'light' : 'dark';
            document.documentElement.setAttribute('data-theme', next);
            fetch('/api/settings', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ key: 'theme_mode', value: next })
            });
        }

        async function loadSettingsView() {
            const cfs = await fetch('/api/custom-fields').then(r => r.json()).catch(() => []);
            const list = document.getElementById('customFieldsRenderList');
            list.innerHTML = cfs.length
                ? cfs.map(f => `<div style="padding:4px 0; border-bottom:1px solid var(--border-color);"><strong>${escapeHtml(f.label)}</strong> <code class="mono-val">(${escapeHtml(f.key)})</code> · ${escapeHtml(f.entity_type)}</div>`).join('')
                : 'Keine Custom Fields definiert.';
        }

        async function submitCustomField() {
            const key = document.getElementById('newCfKey').value.trim();
            const label = document.getElementById('newCfLabel').value.trim();
            if (!key || !label) return;
            await fetch('/api/custom-fields', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ entity_type: 'contact', key, label, field_type: 'text' })
            });
            document.getElementById('newCfKey').value = '';
            document.getElementById('newCfLabel').value = '';
            await loadSettingsView();
        }

        // ── Copilot & Cmd+K Spotlight ──
        function toggleCopilot() {
            document.getElementById('copilotPanel').classList.toggle('open');
            if (document.getElementById('copilotPanel').classList.contains('open')) {
                setTimeout(() => document.getElementById('copilotInput').focus(), 100);
            }
        }

        function openCmdKModal() {
            document.getElementById('cmdkModal').style.display = 'flex';
            renderSpotlightPreview();
            setTimeout(() => document.getElementById('spotlightInput').focus(), 50);
        }

        function closeCmdKModal() {
            document.getElementById('cmdkModal').style.display = 'none';
        }

        async function sendCopilotChat() {
            const input = document.getElementById('copilotInput');
            const prompt = input.value.trim();
            if (!prompt) return;
            const box = document.getElementById('copilotMessages');
            box.innerHTML += `<div class="chat-bubble user">${escapeHtml(prompt)}</div>`;
            input.value = '';
            box.scrollTop = box.scrollHeight;
            await handleAiInteraction(prompt, box);
        }

        async function executeSpotlightCommand() {
            const input = document.getElementById('spotlightInput');
            const prompt = input.value.trim();
            if (!prompt) return;
            closeCmdKModal();
            input.value = '';
            document.getElementById('copilotPanel').classList.add('open');
            const box = document.getElementById('copilotMessages');
            box.innerHTML += `<div class="chat-bubble user">${escapeHtml(prompt)}</div>`;
            await handleAiInteraction(prompt, box);
        }

        function renderSpotlightPreview() {
            const q = (document.getElementById('spotlightInput')?.value || '').trim().toLowerCase();
            const box = document.getElementById('spotlightPreviewResults');
            const matchedContacts = contactsData.filter(c => !q || (c.name||'').toLowerCase().includes(q) || (c.company||'').toLowerCase().includes(q)).slice(0, 4);
            const matchedDeals = dealsData.filter(d => !q || (d.title||'').toLowerCase().includes(q)).slice(0, 3);

            let html = '<div style="font-size:11px; font-weight:600; color:var(--text-muted); padding:4px 8px;">KONTAKTE & AKTEN</div>';
            matchedContacts.forEach(c => {
                html += `<div style="padding:7px 10px; border-radius:6px; cursor:pointer; display:flex; justify-content:space-between; font-size:12.5px;" onmouseover="this.style.background='var(--bg-hover)'" onmouseout="this.style.background='transparent'" onclick="closeCmdKModal(); openContactDrawer('${c.id}');">
                    <span><strong>${escapeHtml(c.name)}</strong> · ${escapeHtml(c.company || 'Privat')}</span>
                    <span style="color:var(--text-muted);">360° Akte →</span>
                </div>`;
            });
            html += '<div style="font-size:11px; font-weight:600; color:var(--text-muted); padding:8px 8px 4px;">DEALS</div>';
            matchedDeals.forEach(d => {
                html += `<div style="padding:7px 10px; border-radius:6px; cursor:pointer; display:flex; justify-content:space-between; font-size:12.5px;" onmouseover="this.style.background='var(--bg-hover)'" onmouseout="this.style.background='transparent'" onclick="closeCmdKModal(); switchView('deals');">
                    <span>${escapeHtml(d.title)}</span>
                    <span class="mono-val">${(d.value_cents/100).toLocaleString('de-DE',{style:'currency',currency:'EUR'})}</span>
                </div>`;
            });
            if (q) {
                html += `<div style="padding:8px 10px; border-top:1px solid var(--border-color); margin-top:6px; font-size:12px; color:var(--accent); cursor:pointer;" onclick="executeSpotlightCommand()">↵ KI-Befehl ausführen: „${escapeHtml(q)}“</div>`;
            }
            box.innerHTML = html;
        }

        async function handleAiInteraction(prompt, box) {
            try {
                const res = await fetch('/api/copilot/chat', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ prompt, mode: currentMode, role: currentRole })
                }).then(r => r.json());

                let html = formatCopilotMarkdown(res.reply);
                if (res.actions_performed?.length) {
                    html += `<div style="margin-top:6px; font-size:11px; color:var(--status-green-fg); font-weight:600;">✓ ${escapeHtml(res.actions_performed.join(', '))}</div>`;
                }
                if (res.ui_mutations) {
                    if (res.ui_mutations.theme_mode) document.documentElement.setAttribute('data-theme', res.ui_mutations.theme_mode);
                    if (res.ui_mutations.theme_accent) setThemeAccent(res.ui_mutations.theme_accent);
                    if (res.ui_mutations.refresh_schema) await loadSettingsView();
                }
                if (res.navigate_to) switchView(res.navigate_to);

                box.innerHTML += `<div class="chat-bubble ai">${html}</div>`;
                box.scrollTop = box.scrollHeight;
                await refreshData();
            } catch (_) {
                box.innerHTML += `<div class="chat-bubble ai">Verbindungsfehler zum Copilot.</div>`;
            }
        }

        function setDemoUser(type) {
            fetch('/api/auth/session', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ role: type === 'admin' ? 'admin' : 'operator' })
            }).catch(() => {});

            if (type === 'admin') {
                currentRole = 'admin';
                document.getElementById('ui-user-avatar').innerText = 'MC';
                document.getElementById('ui-user-name').innerText = 'Merlin Conrad';
                document.getElementById('ui-user-role').innerText = 'Administrator';
                switchAppMode('admin');
            } else {
                currentRole = 'operator';
                document.getElementById('ui-user-avatar').innerText = 'OP';
                document.getElementById('ui-user-name').innerText = 'Operations Team';
                document.getElementById('ui-user-role').innerText = 'Mitarbeiter';
                switchAppMode('user');
            }
        }

        function openModal(id) { document.getElementById(id).style.display = 'flex'; }
        function closeModal(id) { document.getElementById(id).style.display = 'none'; }
        function openLoginModal() { openModal('loginModal'); }

        function openCreateModal() {
            if (currentView === 'deals') openNewDealModal();
            else if (currentView === 'invoices') openNewInvoiceModal();
            else openNewContactModal();
        }

        function openNewContactModal() {
            openModal('modalNewContact');
            setTimeout(() => document.getElementById('nc-name').focus(), 50);
        }

        function openNewDealModal() {
            const sel = document.getElementById('nd-contact-select');
            sel.innerHTML = contactsData.map(c => `<option value="${c.id}">${escapeHtml(c.name)} (${escapeHtml(c.company || 'Privat')})</option>`).join('');
            openModal('modalNewDeal');
        }

        function openNewDealModalForContact(contactId) {
            openNewDealModal();
            if (contactId) document.getElementById('nd-contact-select').value = contactId;
        }

        function openNewInvoiceModal() {
            const sel = document.getElementById('ni-contact-select');
            sel.innerHTML = contactsData.map(c => `<option value="${c.id}">${escapeHtml(c.name)} (${escapeHtml(c.company || 'Privat')})</option>`).join('');
            openModal('modalNewInvoice');
        }

        async function submitCreateContact() {
            const name = document.getElementById('nc-name').value.trim();
            if (!name) return;
            await fetch('/api/contacts', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    name,
                    company: document.getElementById('nc-company').value.trim(),
                    email: document.getElementById('nc-email').value.trim(),
                    phone: document.getElementById('nc-phone').value.trim(),
                    status: document.getElementById('nc-status').value
                })
            });
            closeModal('modalNewContact');
            await refreshData();
        }

        async function submitCreateDeal() {
            const title = document.getElementById('nd-title').value.trim();
            const contact_id = document.getElementById('nd-contact-select').value;
            const val = parseFloat(document.getElementById('nd-val').value) || 0;
            const stage = document.getElementById('nd-stage').value;
            await fetch('/api/deals', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ title, contact_id, value_cents: Math.round(val * 100), stage })
            });
            closeModal('modalNewDeal');
            await refreshData();
        }

        async function submitCreateInvoice() {
            const contact_id = document.getElementById('ni-contact-select').value;
            const desc = document.getElementById('ni-desc').value.trim();
            const amount = parseFloat(document.getElementById('ni-amount').value) || 0;
            await fetch('/api/invoices', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    contact_id,
                    items: [{ description: desc, quantity: 1, unit_price_cents: Math.round(amount * 100), total_cents: Math.round(amount * 100) }]
                })
            });
            closeModal('modalNewInvoice');
            await refreshData();
        }

        function setupKeyboardNav() {
            window.addEventListener('keydown', (e) => {
                const inInput = ['INPUT', 'TEXTAREA', 'SELECT'].includes(document.activeElement.tagName);
                if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                    e.preventDefault();
                    openCmdKModal();
                    return;
                }
                if (e.key === 'Escape') {
                    document.querySelectorAll('.modal-backdrop').forEach(m => m.style.display = 'none');
                    closeCmdKModal();
                    closeContactDrawer();
                    document.getElementById('copilotPanel').classList.remove('open');
                    return;
                }
                if (inInput) return;
                if (e.key === '1') switchView('dashboard');
                if (e.key === '2') switchView('contacts');
                if (e.key === '3') switchView('deals');
                if (e.key === '4') switchView('invoices');
                if (e.key === '5') switchView('retainers');
                if (e.key === '6') switchView('marketing');
                if (e.key === '7') switchView('dms');
                if (e.key === '8') switchView('settings');
                if (e.key.toLowerCase() === 'c') toggleCopilot();
            });
        }

        function toggleMobileSidebar() {
            document.getElementById('appSidebar').classList.toggle('mobile-open');
        }

        function escapeHtml(text) {
            if (!text) return '';
            return String(text).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
        }

        function formatCopilotMarkdown(text) {
            if (!text) return '';
            return escapeHtml(text)
                .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
                .replace(/`([^`]+)`/g, '<code class="mono-val">$1</code>')
                .replace(/\n/g, '<br>');
        }

        window.onload = initApp;
    </script>
</body>
</html>"##
}

pub fn render_booking_html(slug: &str) -> String {
    let host_display = if slug.eq_ignore_ascii_case("merlin") {
        "Merlin Conrad (Conrad KI)"
    } else {
        slug
    };
    format!(
        r##"<!DOCTYPE html>
<html lang="de" data-theme="light">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Termin buchen — {host} | Conrad OS Scheduler</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --font-sans: "Inter", -apple-system, BlinkMacSystemFont, sans-serif;
            --bg-canvas: #f8fafc;
            --bg-surface: #ffffff;
            --border-color: #e2e8f0;
            --text-primary: #0f172a;
            --text-secondary: #475569;
            --accent: #2563eb;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background: var(--bg-canvas);
            color: var(--text-primary);
            font-family: var(--font-sans);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 24px;
            -webkit-font-smoothing: antialiased;
        }}
        .booking-shell {{
            background: var(--bg-surface);
            border: 1px solid var(--border-color);
            border-radius: 12px;
            box-shadow: 0 10px 25px rgba(15, 23, 42, 0.06);
            max-width: 800px;
            width: 100%;
            display: grid;
            grid-template-columns: 270px 1fr;
            overflow: hidden;
        }}
        @media (max-width: 720px) {{
            .booking-shell {{ grid-template-columns: 1fr; }}
        }}
        .host-pane {{
            padding: 24px;
            border-right: 1px solid var(--border-color);
            background: #f1f5f9;
        }}
        .avatar-box {{
            width: 40px;
            height: 40px;
            border-radius: 8px;
            background: var(--text-primary);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 14px;
            margin-bottom: 14px;
        }}
        .slot-grid {{
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 8px;
            margin: 14px 0 20px;
        }}
        .slot-btn {{
            padding: 8px 10px;
            border: 1px solid var(--border-color);
            border-radius: 6px;
            background: #fff;
            cursor: pointer;
            font-size: 12px;
            font-weight: 600;
            color: var(--text-primary);
        }}
        .slot-btn.selected, .slot-btn:hover {{
            border-color: var(--accent);
            background: #eff6ff;
            color: var(--accent);
        }}
        .input-field {{
            width: 100%;
            padding: 9px 11px;
            border: 1px solid var(--border-color);
            border-radius: 6px;
            font-size: 13px;
            margin-bottom: 10px;
            font-family: inherit;
        }}
        .btn-submit {{
            width: 100%;
            padding: 10px 16px;
            border: none;
            border-radius: 6px;
            background: var(--accent);
            color: #fff;
            font-weight: 600;
            font-size: 13px;
            cursor: pointer;
        }}
    </style>
</head>
<body>
    <div class="booking-shell">
        <div class="host-pane">
            <div class="avatar-box">MC</div>
            <div style="font-size:12px; color:var(--text-secondary); font-weight:600;">{host}</div>
            <h1 style="font-size:18px; margin:6px 0 10px;">Strategie- &amp; Systemgespräch</h1>
            <p style="font-size:12.5px; color:var(--text-secondary); line-height:1.5; margin-bottom:18px;">
                30-minütiger Termin zur Potenzialanalyse Ihrer Prozesse und Automatisierungs-Architektur.
            </p>
            <div style="font-size:12px; color:var(--text-secondary); display:flex; flex-direction:column; gap:6px;">
                <div>Dauer: <strong>30 Minuten</strong></div>
                <div>Zeitzone: <strong id="detectedTz">Europe/Berlin</strong></div>
            </div>
        </div>
        <div style="padding: 24px;">
            <h2 style="font-size:15px; margin-bottom:4px;">Verfügbare Termine</h2>
            <p style="font-size:12px; color:var(--text-secondary);">Wählen Sie einen freien Zeitslot</p>
            <div class="slot-grid" id="slotContainer"></div>

            <div id="bookingForm">
                <input type="text" id="b-name" class="input-field" placeholder="Vor- und Nachname *">
                <input type="email" id="b-email" class="input-field" placeholder="Geschäftliche E-Mail *">
                <input type="tel" id="b-phone" class="input-field" placeholder="Telefonnummer">
                <textarea id="b-notes" class="input-field" rows="2" placeholder="Thema / Notiz (optional)"></textarea>
                <button class="btn-submit" onclick="confirmBooking()">Termin verbindlich buchen</button>
            </div>
            <div id="bookingSuccess" style="display:none; padding:14px; border-radius:6px; background:#dcfce7; color:#15803d; font-size:13px; margin-top:12px;"></div>
        </div>
    </div>
    <script>
        const slug = "{slug_raw}";
        const userTz = Intl.DateTimeFormat().resolvedOptions().timeZone || 'Europe/Berlin';
        document.getElementById('detectedTz').innerText = userTz;

        let selectedIso = null;

        async function loadSlots() {{
            const res = await fetch(`/api/bookings/${{slug}}/slots`).then(r => r.json());
            const container = document.getElementById('slotContainer');
            container.innerHTML = res.slots.map((s, idx) => {{
                const d = new Date(s.start_time);
                const label = d.toLocaleString('de-DE', {{ weekday: 'short', day: '2-digit', month: '2-digit', hour: '2-digit', minute: '2-digit' }});
                if (idx === 0) selectedIso = s.start_time;
                return `<button class="slot-btn ${{idx===0?'selected':''}}" onclick="selectSlot(this, '${{s.start_time}}')">${{label}}</button>`;
            }}).join('');
        }}

        function selectSlot(btn, iso) {{
            document.querySelectorAll('.slot-btn').forEach(b => b.classList.remove('selected'));
            btn.classList.add('selected');
            selectedIso = iso;
        }}

        async function confirmBooking() {{
            const name = document.getElementById('b-name').value.trim();
            const email = document.getElementById('b-email').value.trim();
            if (!name || !email || !selectedIso) return alert('Bitte Pflichtfelder ausfüllen.');

            await fetch(`/api/bookings/${{slug}}`, {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{
                    name,
                    email,
                    phone: document.getElementById('b-phone').value.trim(),
                    start_time: selectedIso,
                    timezone: userTz,
                    notes: document.getElementById('b-notes').value.trim()
                }})
            }});

            document.getElementById('bookingForm').style.display = 'none';
            const box = document.getElementById('bookingSuccess');
            box.style.display = 'block';
            box.innerHTML = `<strong>Termin bestätigt.</strong><br>Der Termin für ${{name}} wurde im Kalender eingetragen und der Kontakt-Akte zugeordnet.`;
        }}

        loadSlots();
    </script>
</body>
</html>"##,
        host = host_display,
        slug_raw = slug
    )
}
