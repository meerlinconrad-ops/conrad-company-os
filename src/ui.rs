pub fn render_app_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="de" data-theme="light">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Conrad OS — AI-First Company Operating System</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=SF+Pro+Display:wght@400;500;600;700&family=SF+Pro+Text:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>
        /* ── Apple HIG & Unslop Design System ── */
        :root {
            --font-display: -apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text", system-ui, sans-serif;
            --font-mono: "JetBrains Mono", SFMono-Regular, Menlo, monospace;

            /* Light Canvas (Apple Studio Default) */
            --bg-canvas: #f5f5f7;
            --bg-surface: #ffffff;
            --bg-surface-elevated: #ffffff;
            --bg-surface-subtle: #fbfbfd;
            --bg-tertiary: #f0f0f2;
            --border-subtle: rgba(0, 0, 0, 0.06);
            --border-medium: rgba(0, 0, 0, 0.12);
            --text-primary: #1d1d1f;
            --text-secondary: #6e6e73;
            --text-tertiary: #86868b;
            --shadow-card: 0 2px 14px rgba(0, 0, 0, 0.04), 0 0 1px rgba(0, 0, 0, 0.08);
            --shadow-elevated: 0 16px 36px rgba(0, 0, 0, 0.08), 0 0 1px rgba(0, 0, 0, 0.1);
            --shadow-modal: 0 24px 60px rgba(0, 0, 0, 0.15), 0 0 1px rgba(0, 0, 0, 0.1);

            /* Accent Colors */
            --accent: #0071e3; /* Apple Signature Blue */
            --accent-hover: #0077ed;
            --accent-soft: rgba(0, 113, 227, 0.08);
            --accent-border: rgba(0, 113, 227, 0.25);

            /* Semantic Apple Colors */
            --apple-green: #34c759;
            --apple-green-soft: rgba(52, 199, 89, 0.12);
            --apple-orange: #ff9500;
            --apple-orange-soft: rgba(255, 149, 0, 0.12);
            --apple-red: #ff3b30;
            --apple-red-soft: rgba(255, 59, 48, 0.12);
            --apple-purple: #af52de;
            --apple-purple-soft: rgba(175, 82, 222, 0.12);

            /* AI Glow */
            --ai-aura: linear-gradient(135deg, rgba(0, 113, 227, 0.7), rgba(175, 82, 222, 0.7), rgba(255, 149, 0, 0.5));
            --sidebar-width: 260px;
        }

        [data-theme="dark"] {
            --bg-canvas: #000000;
            --bg-surface: #1c1c1e;
            --bg-surface-elevated: #2c2c2e;
            --bg-surface-subtle: #161618;
            --bg-tertiary: #2c2c2e;
            --border-subtle: rgba(255, 255, 255, 0.08);
            --border-medium: rgba(255, 255, 255, 0.16);
            --text-primary: #f5f5f7;
            --text-secondary: #a1a1a6;
            --text-tertiary: #6e6e73;
            --shadow-card: 0 2px 14px rgba(0, 0, 0, 0.4), 0 0 1px rgba(255, 255, 255, 0.08);
            --shadow-elevated: 0 16px 40px rgba(0, 0, 0, 0.6), 0 0 1px rgba(255, 255, 255, 0.12);
            --shadow-modal: 0 24px 60px rgba(0, 0, 0, 0.75), 0 0 1px rgba(255, 255, 255, 0.15);
            --accent-soft: rgba(0, 113, 227, 0.2);
            --accent-border: rgba(0, 113, 227, 0.4);
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            -webkit-tap-highlight-color: transparent;
        }

        body {
            background-color: var(--bg-canvas);
            color: var(--text-primary);
            font-family: var(--font-display);
            font-size: 14px;
            line-height: 1.45;
            display: flex;
            height: 100vh;
            overflow: hidden;
            -webkit-font-smoothing: antialiased;
            transition: background-color 0.25s ease, color 0.25s ease;
        }

        /* ── Apple Sidebar Navigation ── */
        .sidebar {
            width: var(--sidebar-width);
            background: var(--bg-surface);
            border-right: 1px solid var(--border-subtle);
            display: flex;
            flex-direction: column;
            padding: 20px 16px;
            flex-shrink: 0;
            z-index: 20;
            transition: all 0.25s ease;
        }

        .brand-header {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 4px 6px 18px;
            border-bottom: 1px solid var(--border-subtle);
            margin-bottom: 16px;
        }

        .brand-icon {
            width: 32px;
            height: 32px;
            border-radius: 8px;
            background: var(--text-primary);
            color: var(--bg-surface);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 16px;
            font-weight: 700;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
        }

        .brand-titles {
            display: flex;
            flex-direction: column;
        }

        .brand-name {
            font-size: 14px;
            font-weight: 600;
            letter-spacing: -0.01em;
            color: var(--text-primary);
        }

        .brand-sub {
            font-size: 11px;
            color: var(--text-tertiary);
            letter-spacing: 0.02em;
        }

        /* Segmented Mode Switcher (Apple Pill) */
        .mode-segmented-control {
            background: var(--bg-tertiary);
            border-radius: 10px;
            padding: 3px;
            display: flex;
            margin-bottom: 18px;
            position: relative;
        }

        .mode-segmented-btn {
            flex: 1;
            padding: 6px 10px;
            border: none;
            background: transparent;
            color: var(--text-secondary);
            font-size: 12px;
            font-weight: 600;
            font-family: inherit;
            border-radius: 7px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }

        .mode-segmented-btn.active {
            background: var(--bg-surface);
            color: var(--text-primary);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
        }

        .mode-segmented-btn.admin.active {
            color: var(--apple-orange);
        }

        /* Navigation Links */
        .nav-section-label {
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            color: var(--text-tertiary);
            padding: 8px 10px 4px;
        }

        .nav-list {
            list-style: none;
            display: flex;
            flex-direction: column;
            gap: 2px;
            margin-bottom: 14px;
        }

        .nav-item-btn {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 8px 10px;
            border-radius: 8px;
            background: transparent;
            color: var(--text-secondary);
            border: none;
            font-size: 13px;
            font-weight: 500;
            cursor: pointer;
            width: 100%;
            text-align: left;
            transition: all 0.15s ease;
        }

        .nav-item-btn:hover {
            background: var(--bg-tertiary);
            color: var(--text-primary);
        }

        .nav-item-btn.active {
            background: var(--accent-soft);
            color: var(--accent);
            font-weight: 600;
        }

        .nav-badge {
            margin-left: auto;
            font-size: 11px;
            padding: 2px 6px;
            border-radius: 6px;
            background: var(--bg-tertiary);
            color: var(--text-tertiary);
            font-weight: 500;
        }

        .admin-only-badge {
            font-size: 9px;
            font-weight: 700;
            text-transform: uppercase;
            padding: 2px 5px;
            border-radius: 4px;
            background: var(--apple-orange-soft);
            color: var(--apple-orange);
            margin-left: auto;
        }

        /* User Profile Pill at bottom of sidebar */
        .sidebar-footer {
            margin-top: auto;
            padding-top: 14px;
            border-top: 1px solid var(--border-subtle);
            display: flex;
            align-items: center;
            gap: 10px;
            cursor: pointer;
            padding: 10px;
            border-radius: 10px;
            transition: background 0.15s ease;
        }

        .sidebar-footer:hover {
            background: var(--bg-tertiary);
        }

        .avatar {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background: linear-gradient(135deg, #0071e3, #af52de);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            font-weight: 600;
            flex-shrink: 0;
        }

        .user-info {
            display: flex;
            flex-direction: column;
            overflow: hidden;
            flex: 1;
        }

        .user-name {
            font-size: 12px;
            font-weight: 600;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .user-role {
            font-size: 11px;
            color: var(--text-tertiary);
        }

        /* ── Main Workspace Content ── */
        .main-wrapper {
            flex: 1;
            display: flex;
            flex-direction: column;
            height: 100vh;
            overflow: hidden;
            background: var(--bg-canvas);
        }

        /* Apple Frosted Topbar */
        .topbar {
            height: 56px;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(25px) saturate(180%);
            -webkit-backdrop-filter: blur(25px) saturate(180%);
            border-bottom: 1px solid var(--border-subtle);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 24px;
            flex-shrink: 0;
            z-index: 15;
            transition: background 0.25s ease;
        }

        [data-theme="dark"] .topbar {
            background: rgba(28, 28, 30, 0.85);
        }

        .topbar-left {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .topbar-title {
            font-size: 16px;
            font-weight: 600;
            letter-spacing: -0.015em;
        }

        .mode-indicator-pill {
            font-size: 11px;
            font-weight: 600;
            padding: 3px 8px;
            border-radius: 6px;
            display: inline-flex;
            align-items: center;
            gap: 5px;
        }

        .mode-indicator-pill.user {
            background: var(--accent-soft);
            color: var(--accent);
        }

        .mode-indicator-pill.admin {
            background: var(--apple-orange-soft);
            color: var(--apple-orange);
        }

        .topbar-right {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        /* Apple Intelligence Copilot Button */
        .btn-apple-ai {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 7px 14px;
            border-radius: 20px;
            background: var(--bg-surface);
            border: 1px solid transparent;
            position: relative;
            cursor: pointer;
            font-size: 13px;
            font-weight: 600;
            color: var(--text-primary);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
            transition: all 0.25s ease;
            background-clip: padding-box;
        }

        .btn-apple-ai::before {
            content: '';
            position: absolute;
            inset: -1px;
            border-radius: 21px;
            background: var(--ai-aura);
            z-index: -1;
            opacity: 0.85;
            transition: opacity 0.3s ease;
        }

        .btn-apple-ai:hover::before {
            opacity: 1;
        }

        .btn-apple-ai:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 14px rgba(0, 113, 227, 0.2);
        }

        /* Standard Apple Button Styles */
        .btn {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            padding: 8px 14px;
            border-radius: 8px;
            font-size: 13px;
            font-weight: 500;
            font-family: inherit;
            cursor: pointer;
            border: 1px solid var(--border-medium);
            background: var(--bg-surface);
            color: var(--text-primary);
            transition: all 0.15s ease;
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
        }

        .btn:hover {
            background: var(--bg-tertiary);
        }

        .btn:active {
            transform: scale(0.98);
        }

        .btn-primary {
            background: var(--accent);
            border-color: var(--accent);
            color: #ffffff;
        }

        .btn-primary:hover {
            background: var(--accent-hover);
        }

        /* Scrollable Main Views Area */
        .view-content-area {
            flex: 1;
            overflow-y: auto;
            padding: 24px 28px 100px;
        }

        .view-container {
            max-width: 1200px;
            margin: 0 auto;
            animation: fadeIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(4px); }
            to { opacity: 1; transform: translateY(0); }
        }

        /* Apple Clean Cards */
        .apple-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 16px;
            box-shadow: var(--shadow-card);
            padding: 22px;
            margin-bottom: 20px;
            transition: all 0.2s ease;
        }

        .apple-card-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 18px;
        }

        .apple-card-title {
            font-size: 15px;
            font-weight: 600;
            letter-spacing: -0.01em;
            color: var(--text-primary);
        }

        /* Metric Grid */
        .metric-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 16px;
            margin-bottom: 20px;
        }

        @media (max-width: 1024px) {
            .metric-grid { grid-template-columns: repeat(2, 1fr); }
        }

        @media (max-width: 640px) {
            .metric-grid { grid-template-columns: 1fr; }
        }

        .metric-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 14px;
            padding: 18px 20px;
            box-shadow: var(--shadow-card);
        }

        .metric-label {
            font-size: 12px;
            font-weight: 500;
            color: var(--text-secondary);
            margin-bottom: 6px;
        }

        .metric-value {
            font-size: 24px;
            font-weight: 700;
            letter-spacing: -0.02em;
            color: var(--text-primary);
            line-height: 1.15;
        }

        .metric-sub {
            font-size: 11px;
            color: var(--text-tertiary);
            margin-top: 4px;
        }

        /* Apple Clean Tables */
        .table-responsive {
            width: 100%;
            overflow-x: auto;
        }

        table {
            width: 100%;
            border-collapse: collapse;
            text-align: left;
            font-size: 13px;
        }

        th {
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--text-tertiary);
            padding: 12px 14px;
            border-bottom: 1px solid var(--border-subtle);
            white-space: nowrap;
        }

        td {
            padding: 13px 14px;
            border-bottom: 1px solid var(--border-subtle);
            color: var(--text-primary);
            vertical-align: middle;
        }

        tr:last-child td {
            border-bottom: none;
        }

        tr:hover td {
            background: var(--bg-surface-subtle);
        }

        /* Apple Status Pills */
        .status-pill {
            display: inline-flex;
            align-items: center;
            gap: 5px;
            padding: 3px 8px;
            border-radius: 6px;
            font-size: 11px;
            font-weight: 600;
        }

        .status-pill.success {
            background: var(--apple-green-soft);
            color: var(--apple-green);
        }

        .status-pill.warning {
            background: var(--apple-orange-soft);
            color: var(--apple-orange);
        }

        .status-pill.danger {
            background: var(--apple-red-soft);
            color: var(--apple-red);
        }

        .status-pill.info {
            background: var(--accent-soft);
            color: var(--accent);
        }

        .status-pill.neutral {
            background: var(--bg-tertiary);
            color: var(--text-secondary);
        }

        /* Kanban Board */
        .kanban-board {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 14px;
            align-items: start;
        }

        @media (max-width: 900px) {
            .kanban-board { grid-template-columns: 1fr; }
        }

        .kanban-column {
            background: var(--bg-surface-subtle);
            border: 1px solid var(--border-subtle);
            border-radius: 14px;
            padding: 14px;
            min-height: 400px;
        }

        .kanban-col-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 12px;
            font-weight: 600;
            color: var(--text-secondary);
            margin-bottom: 12px;
            padding-bottom: 8px;
            border-bottom: 1px solid var(--border-subtle);
        }

        .kanban-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 12px;
            margin-bottom: 10px;
            cursor: pointer;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.03);
            transition: all 0.15s ease;
        }

        .kanban-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
            border-color: var(--border-medium);
        }

        .kanban-card-title {
            font-size: 13px;
            font-weight: 600;
            margin-bottom: 4px;
        }

        .kanban-card-meta {
            display: flex;
            justify-content: space-between;
            font-size: 11px;
            color: var(--text-secondary);
            margin-top: 8px;
        }

        /* ── Slide-Over AI Copilot Drawer (Apple Intelligence) ── */
        .copilot-panel {
            position: fixed;
            top: 0;
            right: -420px;
            width: 400px;
            height: 100vh;
            background: var(--bg-surface);
            border-left: 1px solid var(--border-subtle);
            box-shadow: var(--shadow-modal);
            transition: right 0.3s cubic-bezier(0.16, 1, 0.3, 1);
            display: flex;
            flex-direction: column;
            z-index: 50;
        }

        .copilot-panel.open {
            right: 0;
        }

        .copilot-panel-header {
            padding: 16px 20px;
            border-bottom: 1px solid var(--border-subtle);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        .copilot-badge {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .copilot-aura-dot {
            width: 10px;
            height: 10px;
            border-radius: 50%;
            background: var(--ai-aura);
            animation: pulseAura 2s infinite ease-in-out;
        }

        @keyframes pulseAura {
            0%, 100% { transform: scale(1); opacity: 0.8; }
            50% { transform: scale(1.25); opacity: 1; }
        }

        .copilot-messages {
            flex: 1;
            overflow-y: auto;
            padding: 18px 20px;
            display: flex;
            flex-direction: column;
            gap: 12px;
        }

        .chat-bubble {
            max-width: 86%;
            padding: 11px 15px;
            border-radius: 14px;
            font-size: 13px;
            line-height: 1.45;
            word-break: break-word;
        }

        .chat-bubble.user {
            background: var(--accent);
            color: #ffffff;
            align-self: flex-end;
            border-bottom-right-radius: 4px;
        }

        .chat-bubble.ai {
            background: var(--bg-tertiary);
            color: var(--text-primary);
            align-self: flex-start;
            border-bottom-left-radius: 4px;
        }

        .chat-nav-chip {
            display: inline-flex;
            align-items: center;
            gap: 5px;
            margin-top: 8px;
            padding: 5px 10px;
            border-radius: 6px;
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            color: var(--accent);
            font-weight: 600;
            font-size: 11px;
            cursor: pointer;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
        }

        .chat-nav-chip:hover {
            border-color: var(--accent);
        }

        .copilot-input-wrap {
            padding: 14px 18px 20px;
            border-top: 1px solid var(--border-subtle);
            background: var(--bg-surface-subtle);
        }

        .copilot-input-box {
            display: flex;
            gap: 8px;
            background: var(--bg-surface);
            border: 1px solid var(--border-medium);
            border-radius: 12px;
            padding: 4px 6px 4px 12px;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
        }

        .copilot-input {
            flex: 1;
            border: none;
            background: transparent;
            outline: none;
            font-family: inherit;
            font-size: 13px;
            color: var(--text-primary);
        }

        /* ── Spotlight / Floating AI Action Bar (Bottom of Screen) ── */
        .spotlight-bar-wrapper {
            position: fixed;
            bottom: 24px;
            left: calc(var(--sidebar-width) + 24px);
            right: 24px;
            max-width: 640px;
            margin: 0 auto;
            z-index: 30;
            pointer-events: none;
        }

        .spotlight-bar {
            pointer-events: auto;
            background: rgba(255, 255, 255, 0.92);
            backdrop-filter: blur(20px) saturate(180%);
            -webkit-backdrop-filter: blur(20px) saturate(180%);
            border: 1px solid var(--border-subtle);
            border-radius: 16px;
            padding: 8px 14px;
            display: flex;
            align-items: center;
            gap: 10px;
            box-shadow: var(--shadow-elevated);
        }

        [data-theme="dark"] .spotlight-bar {
            background: rgba(28, 28, 30, 0.92);
        }

        .spotlight-input {
            flex: 1;
            border: none;
            background: transparent;
            font-family: inherit;
            font-size: 14px;
            color: var(--text-primary);
            outline: none;
        }

        .spotlight-hint {
            font-size: 11px;
            color: var(--text-tertiary);
            background: var(--bg-tertiary);
            padding: 2px 6px;
            border-radius: 4px;
            font-family: var(--font-mono);
        }

        /* ── Modal Dialogs ── */
        .modal-backdrop {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.35);
            backdrop-filter: blur(8px);
            -webkit-backdrop-filter: blur(8px);
            z-index: 100;
            align-items: center;
            justify-content: center;
            animation: fadeIn 0.15s ease;
        }

        .modal-card {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 18px;
            box-shadow: var(--shadow-modal);
            width: min(92vw, 480px);
            padding: 24px;
            animation: scaleUp 0.18s cubic-bezier(0.16, 1, 0.3, 1);
        }

        @keyframes scaleUp {
            from { transform: scale(0.96); opacity: 0; }
            to { transform: scale(1); opacity: 1; }
        }

        .modal-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 18px;
        }

        .modal-title {
            font-size: 16px;
            font-weight: 600;
        }

        .modal-close-btn {
            background: transparent;
            border: none;
            font-size: 18px;
            color: var(--text-tertiary);
            cursor: pointer;
        }

        .form-group {
            margin-bottom: 14px;
        }

        .form-label {
            display: block;
            font-size: 12px;
            font-weight: 500;
            color: var(--text-secondary);
            margin-bottom: 5px;
        }

        .form-input {
            width: 100%;
            padding: 9px 12px;
            border: 1px solid var(--border-medium);
            border-radius: 8px;
            background: var(--bg-surface);
            color: var(--text-primary);
            font-size: 13px;
            font-family: inherit;
            outline: none;
            transition: border-color 0.15s ease;
        }

        .form-input:focus {
            border-color: var(--accent);
            box-shadow: 0 0 0 3px var(--accent-soft);
        }

        /* ── Demo Credentials Floating Banner ── */
        .demo-bar {
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 12px;
            padding: 10px 16px;
            margin-bottom: 18px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 12px;
            box-shadow: var(--shadow-card);
        }

        .demo-credentials-tag {
            font-family: var(--font-mono);
            background: var(--bg-tertiary);
            padding: 2px 6px;
            border-radius: 4px;
            font-size: 11px;
            color: var(--text-primary);
        }

        /* Mobile Adjustments */
        @media (max-width: 768px) {
            #mobileMenuBtn {
                display: inline-flex !important;
            }
            .sidebar {
                position: fixed;
                left: -100%;
                top: 0;
                bottom: 0;
                width: 280px;
                box-shadow: var(--shadow-modal);
                z-index: 100;
            }
            .sidebar.mobile-open {
                left: 0;
            }
            .copilot-panel {
                width: 100% !important;
                right: -100% !important;
            }
            .copilot-panel.open {
                right: 0 !important;
            }
            .spotlight-bar-wrapper {
                left: 12px;
                right: 12px;
                bottom: 12px;
            }
            .view-content-area {
                padding: 14px 12px 90px;
            }
        }
    </style>
</head>
<body>

    <!-- ── Apple Sidebar Navigation ── -->
    <aside class="sidebar" id="appSidebar">
        <div class="brand-header">
            <div class="brand-icon"></div>
            <div class="brand-titles">
                <span class="brand-name" id="ui-brand-name">Conrad OS</span>
                <span class="brand-sub">COMPANY OS · RUST CORE</span>
            </div>
        </div>

        <!-- Mode Segmented Control (Pill Switcher) -->
        <div class="mode-segmented-control" role="tablist">
            <button class="mode-segmented-btn active" id="btnModeUser" onclick="switchAppMode('user')" title="Operativer Modus für das tägliche Kundengeschäft">
                <span>👤</span> User
            </button>
            <button class="mode-segmented-btn admin" id="btnModeAdmin" onclick="switchAppMode('admin')" title="System-Architekt Modus für Layout, Themes & Plugins">
                <span>🛠️</span> Admin
            </button>
        </div>

        <div class="nav-section-label">Übersicht</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn active" id="nav-btn-dashboard" onclick="switchView('dashboard')">
                    <span>📊</span> Betriebs-Cockpit
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-contacts" onclick="switchView('contacts')">
                    <span>👥</span> Kontakte &amp; Leads
                    <span class="nav-badge" id="badge-contacts-count">0</span>
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-deals" onclick="switchView('deals')">
                    <span>💼</span> Vertriebs-Pipeline
                    <span class="nav-badge" id="badge-deals-count">0</span>
                </button>
            </li>
        </ul>

        <div class="nav-section-label">Finanzen &amp; Wachstum</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn" id="nav-btn-invoices" onclick="switchView('invoices')">
                    <span>🧾</span> Faktura &amp; E-Rechnung
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-retainers" onclick="switchView('retainers')">
                    <span>🔄</span> Retainer &amp; Abos
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-marketing" onclick="switchView('marketing')">
                    <span>📈</span> Marketing &amp; ROI
                </button>
            </li>
        </ul>

        <div class="nav-section-label">System &amp; Compliance</div>
        <ul class="nav-list">
            <li>
                <button class="nav-item-btn" id="nav-btn-dms" onclick="switchView('dms')">
                    <span>📁</span> DMS &amp; GoBD-Archiv
                </button>
            </li>
            <li>
                <button class="nav-item-btn" id="nav-btn-settings" onclick="switchView('settings')">
                    <span>⚙️</span> System-Architektur
                    <span class="admin-only-badge">Admin</span>
                </button>
            </li>
        </ul>

        <!-- Bottom User Profile Pill -->
        <div class="sidebar-footer" onclick="openLoginModal()" title="Klicken zum Wechseln des Demo-Benutzers">
            <div class="avatar" id="ui-user-avatar">MC</div>
            <div class="user-info">
                <span class="user-name" id="ui-user-name">Merlin Conrad</span>
                <span class="user-role" id="ui-user-role">Administrator</span>
            </div>
            <span style="font-size:10px; color:var(--text-tertiary);">▾</span>
        </div>
    </aside>

    <!-- ── Main Workspace ── -->
    <div class="main-wrapper">
        <!-- Apple Frosted Topbar -->
        <header class="topbar">
            <div class="topbar-left">
                <button class="btn" style="display:none; padding:6px 10px;" id="mobileMenuBtn" onclick="toggleMobileSidebar()">☰</button>
                <span class="topbar-title" id="currentViewTitle">Betriebs-Cockpit</span>
                <span class="mode-indicator-pill user" id="topbarModePill">
                    <span style="width:6px; height:6px; border-radius:50%; background:currentColor;"></span>
                    <span id="topbarModeLabel">User-Modus (Operativ)</span>
                </span>
            </div>

            <div class="topbar-right">
                <button class="btn" onclick="toggleTheme()" title="Zwischen Light &amp; Dark Mode umschalten">
                    <span id="themeToggleIcon">🌙</span>
                </button>
                <button class="btn-apple-ai" onclick="toggleCopilot()">
                    <span>✨</span>
                    <span id="copilotBtnLabel">Copilot</span>
                </button>
                <button class="btn btn-primary" id="topbarActionBtn" onclick="openCreateModal()">
                    <span>+</span> Neu
                </button>
            </div>
        </header>

        <!-- Scrollable Content Views -->
        <div class="view-content-area">
            
            <!-- Demo Login Banner -->
            <div class="demo-bar">
                <div style="display:flex; align-items:center; gap:8px;">
                    <span style="font-size:14px;">🔑</span>
                    <span><strong>Demo-Zugang aktiv:</strong> Angemeldet als <span class="demo-credentials-tag" id="demoAccountLabel">admin@conrad-ki.com</span> (Passwort: <span class="demo-credentials-tag">conrad2026!</span>)</span>
                </div>
                <div style="display:flex; gap:6px;">
                    <button class="btn" style="padding:4px 10px; font-size:11px;" onclick="setDemoUser('admin')">Als Admin</button>
                    <button class="btn" style="padding:4px 10px; font-size:11px;" onclick="setDemoUser('team')">Als Mitarbeiter</button>
                </div>
            </div>

            <!-- View 1: Dashboard -->
            <section id="view-dashboard" class="view-container">
                <div class="metric-grid">
                    <div class="metric-card">
                        <div class="metric-label">Pipeline-Volumen</div>
                        <div class="metric-value" id="kpi-pipeline">0,00 €</div>
                        <div class="metric-sub">Aktive Angebote in Verhandlung</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">Realisierter Umsatz</div>
                        <div class="metric-value" id="kpi-revenue" style="color:var(--apple-green);">0,00 €</div>
                        <div class="metric-sub">Bezahlte Rechnungen (GoBD)</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">Offene Forderungen</div>
                        <div class="metric-value" id="kpi-pending" style="color:var(--apple-orange);">0,00 €</div>
                        <div class="metric-sub">Ausstehende Zahlungen</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">Kontakte &amp; Leads</div>
                        <div class="metric-value" id="kpi-contacts">0</div>
                        <div class="metric-sub">Erfasst im Conrad OS</div>
                    </div>
                </div>

                <div class="apple-card">
                    <div class="apple-card-header">
                        <span class="apple-card-title">Neueste Kontakte &amp; Akquise-Leads</span>
                        <button class="btn" onclick="switchView('contacts')">Alle Kontakte ansehen →</button>
                    </div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Unternehmen</th>
                                    <th>E-Mail</th>
                                    <th>Status</th>
                                    <th>Erstellt</th>
                                </tr>
                            </thead>
                            <tbody id="dashboardRecentContacts">
                                <tr><td colspan="5" style="text-align:center; color:var(--text-tertiary);">Lade Kontakte...</td></tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 2: Kontakte & Leads -->
            <section id="view-contacts" class="view-container" style="display:none;">
                <div class="apple-card">
                    <div class="apple-card-header">
                        <div style="display:flex; gap:10px; align-items:center; flex:1; max-width:400px;">
                            <input type="text" id="contactSearchInput" class="form-input" placeholder="🔍 Suche nach Name, Firma, E-Mail..." oninput="filterContactsList()">
                        </div>
                        <button class="btn btn-primary" onclick="openNewContactModal()">+ Neuer Kontakt</button>
                    </div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Firma</th>
                                    <th>Kontakt</th>
                                    <th>Status</th>
                                    <th>Tags</th>
                                    <th>Aktion</th>
                                </tr>
                            </thead>
                            <tbody id="contactsFullTable"></tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 3: Vertriebs-Pipeline (Deals) -->
            <section id="view-deals" class="view-container" style="display:none;">
                <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:16px;">
                    <p style="font-size:13px; color:var(--text-secondary);">Klick auf eine Deal-Karte bewegt den Vorgang in die nächste Phase.</p>
                    <button class="btn btn-primary" onclick="openNewDealModal()">+ Neuer Deal</button>
                </div>
                <div class="kanban-board">
                    <div class="kanban-column" id="k-col-discovery">
                        <div class="kanban-col-header"><span>1. ERSTKONTAKT</span><span id="k-sum-discovery">0 €</span></div>
                        <div id="k-items-discovery"></div>
                    </div>
                    <div class="kanban-column" id="k-col-proposal">
                        <div class="kanban-col-header"><span>2. ANGEBOT</span><span id="k-sum-proposal">0 €</span></div>
                        <div id="k-items-proposal"></div>
                    </div>
                    <div class="kanban-column" id="k-col-negotiation">
                        <div class="kanban-col-header"><span>3. VERHANDLUNG</span><span id="k-sum-negotiation">0 €</span></div>
                        <div id="k-items-negotiation"></div>
                    </div>
                    <div class="kanban-column" id="k-col-closed_won">
                        <div class="kanban-col-header"><span>4. GEWONNEN</span><span id="k-sum-closed_won">0 €</span></div>
                        <div id="k-items-closed_won"></div>
                    </div>
                </div>
            </section>

            <!-- View 4: Faktura & E-Rechnung -->
            <section id="view-invoices" class="view-container" style="display:none;">
                <div class="apple-card">
                    <div class="apple-card-header">
                        <div>
                            <span class="apple-card-title">GoBD-Faktura &amp; ZUGFeRD 2.2 / XRechnung</span>
                            <p style="font-size:12px; color:var(--text-secondary); margin-top:2px;">Revisionssichere E-Rechnungen nach europäischer Norm EN 16931.</p>
                        </div>
                        <button class="btn btn-primary" onclick="openNewInvoiceModal()">+ Neue Rechnung erstellen</button>
                    </div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Nummer</th>
                                    <th>Kunde</th>
                                    <th>Fällig</th>
                                    <th>Netto</th>
                                    <th>Brutto</th>
                                    <th>Status</th>
                                    <th>Aktionen</th>
                                </tr>
                            </thead>
                            <tbody id="invoicesFullTable"></tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 5: Retainer & Abos -->
            <section id="view-retainers" class="view-container" style="display:none;">
                <div class="apple-card" style="background:linear-gradient(180deg, var(--bg-surface) 0%, var(--bg-surface-subtle) 100%);">
                    <div class="apple-card-header">
                        <div>
                            <span class="apple-card-title">🔄 Automatisierter Monats-Retainer Abrechnungslauf</span>
                            <p style="font-size:12px; color:var(--text-secondary); margin-top:4px;">Erzeugt GoBD-festgeschriebene Monatsrechnungen mit ZUGFeRD 2.2 XML für fällige Verträge.</p>
                        </div>
                        <button class="btn btn-primary" onclick="triggerRetainerRun()">⚡ Abrechnungslauf jetzt starten</button>
                    </div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Kunde</th>
                                    <th>Plan / Leistung</th>
                                    <th>Betrag</th>
                                    <th>Intervall</th>
                                    <th>Nächste Abrechnung</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody id="retainersFullTable"></tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 6: Marketing & ROI -->
            <section id="view-marketing" class="view-container" style="display:none;">
                <div class="metric-grid">
                    <div class="metric-card">
                        <div class="metric-label">Ad Spend (Ausgaben)</div>
                        <div class="metric-value" id="m-kpi-spend">0,00 €</div>
                        <div class="metric-sub">Meta &amp; Google Ads</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">Generierte Leads</div>
                        <div class="metric-value" id="m-kpi-leads">0</div>
                        <div class="metric-sub">Aus Web-Funnel &amp; Ads</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">Blended CAC</div>
                        <div class="metric-value" id="m-kpi-cac" style="color:var(--apple-green);">0,00 €</div>
                        <div class="metric-sub">Kosten pro Neukunde</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-label">ROAS</div>
                        <div class="metric-value" id="m-kpi-roas" style="color:var(--apple-purple);">0.0x</div>
                        <div class="metric-sub">Umsatz / Werbebudget</div>
                    </div>
                </div>

                <div class="apple-card">
                    <div class="apple-card-header">
                        <span class="apple-card-title">Werbekampagnen &amp; Performance</span>
                    </div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Kampagne</th>
                                    <th>Plattform</th>
                                    <th>Ausgaben</th>
                                    <th>Klicks</th>
                                    <th>Leads</th>
                                    <th>Kunden</th>
                                    <th>Umsatz</th>
                                    <th>Status</th>
                                </tr>
                            </thead>
                            <tbody id="marketingCampaignsTable"></tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 7: DMS & GoBD Archiv -->
            <section id="view-dms" class="view-container" style="display:none;">
                <div class="apple-card">
                    <div class="apple-card-header">
                        <div>
                            <span class="apple-card-title">Kryptografische GoBD-Hash-Chain (SHA-256)</span>
                            <p style="font-size:12px; color:var(--text-secondary); margin-top:2px;">Revisionssichere Unveränderbarkeit aller Buchungsvorgänge.</p>
                        </div>
                        <button class="btn" onclick="verifyGobdChain()">🔍 Prüfsumme verifizieren</button>
                    </div>
                    <div id="gobdVerifyResult" style="padding:10px 14px; border-radius:8px; margin-bottom:14px; font-size:12px; display:none;"></div>
                    <div class="table-responsive">
                        <table>
                            <thead>
                                <tr>
                                    <th>Seq #</th>
                                    <th>Zeitstempel</th>
                                    <th>Vorgang</th>
                                    <th>Entity ID</th>
                                    <th>Details</th>
                                    <th>Akteur</th>
                                </tr>
                            </thead>
                            <tbody id="gobdAuditTable"></tbody>
                        </table>
                    </div>
                </div>
            </section>

            <!-- View 8: System-Architektur (Admin Customizer & MCP) -->
            <section id="view-settings" class="view-container" style="display:none;">
                <div class="apple-card">
                    <div class="apple-card-header">
                        <div>
                            <span class="apple-card-title">🛠️ System-Architekt &amp; Dynamic Customizer</span>
                            <p style="font-size:12px; color:var(--text-secondary); margin-top:2px;">Passen Sie Layout, Farbakzente und Module im laufenden Betrieb an.</p>
                        </div>
                    </div>

                    <div style="display:grid; grid-template-columns:1fr 1fr; gap:20px; margin-bottom:24px;">
                        <div>
                            <label class="form-label">Unternehmensname (Branding)</label>
                            <input type="text" id="settingCompanyName" class="form-input" value="Conrad KI">
                            <button class="btn" style="margin-top:8px;" onclick="saveCompanyNameSetting()">Name speichern</button>
                        </div>
                        <div>
                            <label class="form-label">System-Akzentfarbe</label>
                            <div style="display:flex; gap:8px; margin-top:6px;">
                                <button class="btn" style="border-left:4px solid #0071E3;" onclick="setThemeAccent('#0071E3')">Apple Blue</button>
                                <button class="btn" style="border-left:4px solid #34C759;" onclick="setThemeAccent('#34C759')">Emerald</button>
                                <button class="btn" style="border-left:4px solid #C8531A;" onclick="setThemeAccent('#C8531A')">Terra Cotta</button>
                                <button class="btn" style="border-left:4px solid #AF52DE;" onclick="setThemeAccent('#AF52DE')">Purple</button>
                            </div>
                        </div>
                    </div>

                    <h4 style="font-size:13px; font-weight:600; margin-bottom:10px;">Model Context Protocol (MCP) Endpunkt</h4>
                    <p style="font-size:12px; color:var(--text-secondary); margin-bottom:8px;">Vollständiger JSON-RPC 2.0 Server für lokale Modelle (Qwen 1.5B, Claude, Hermes):</p>
                    <div style="background:var(--bg-tertiary); padding:10px 14px; border-radius:8px; font-family:var(--font-mono); font-size:12px; color:var(--text-primary); margin-bottom:20px;">
                        POST /mcp · Content-Type: application/json
                    </div>

                    <h4 style="font-size:13px; font-weight:600; margin-bottom:10px;">🧩 Dynamische Custom Fields</h4>
                    <div style="display:flex; gap:8px; max-width:600px; margin-bottom:14px;">
                        <input type="text" id="newCfKey" class="form-input" placeholder="Technischer Key (z.B. branche)">
                        <input type="text" id="newCfLabel" class="form-input" placeholder="Label (z.B. Branche)">
                        <button class="btn btn-primary" onclick="submitCustomField()">Hinzufügen</button>
                    </div>
                    <div id="customFieldsRenderList" style="font-size:12px; color:var(--text-secondary);"></div>
                </div>
            </section>

        </div>

        <!-- ── Floating AI Spotlight Bar (Bottom) ── -->
        <div class="spotlight-bar-wrapper">
            <div class="spotlight-bar">
                <span style="font-size:16px;">✨</span>
                <input type="text" id="spotlightInput" class="spotlight-input" placeholder="Mit KI sprechen: „Zeig mir offene Rechnungen“ oder „Schalte auf Dark Mode“..." onkeydown="if(event.key==='Enter') executeSpotlightCommand()">
                <span class="spotlight-hint">Enter</span>
            </div>
        </div>
    </div>

    <!-- ── Apple Intelligence Slide-Over Copilot Panel ── -->
    <aside class="copilot-panel" id="copilotPanel">
        <div class="copilot-panel-header">
            <div class="copilot-badge">
                <div class="copilot-aura-dot"></div>
                <div>
                    <div style="font-size:13px; font-weight:600;" id="copilotPanelTitle">Conrad Copilot</div>
                    <div style="font-size:10px; color:var(--text-tertiary);" id="copilotPanelSub">QWEN 1.5B · MCP CONNECTED</div>
                </div>
            </div>
            <button class="modal-close-btn" onclick="toggleCopilot()">✕</button>
        </div>

        <div class="copilot-messages" id="copilotMessages">
            <div class="chat-bubble ai">
                Hallo! Ich bin dein integrierter KI-Copilot. Du kannst mit mir sprechen und das System steuern:<br><br>
                💡 <em>„Wie steht unsere Pipeline heute?“</em><br>
                💡 <em>„Trag Hans Meyer von Meyer Dachbau als Lead ein“</em><br>
                💡 <em>„Schalte in den Admin-Modus um und aktiviere Dark Mode“</em>
            </div>
        </div>

        <div class="copilot-input-wrap">
            <div class="copilot-input-box">
                <input type="text" id="copilotInput" class="copilot-input" placeholder="Befehl oder Frage eingeben..." onkeydown="if(event.key==='Enter') sendCopilotChat()">
                <button class="btn btn-primary" id="copilotSendBtn" style="padding:6px 12px; border-radius:8px;" onclick="sendCopilotChat()">Senden</button>
            </div>
        </div>
    </aside>

    <!-- ── Modal: Demo Login Switcher ── -->
    <div class="modal-backdrop" id="loginModal">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">Demo-Benutzer wählen</span>
                <button class="modal-close-btn" onclick="closeModal('loginModal')">✕</button>
            </div>
            <p style="font-size:13px; color:var(--text-secondary); margin-bottom:18px;">
                Wählen Sie eine Benutzerrolle zum sofortigen Testen aller Funktionen:
            </p>

            <div style="display:flex; flex-direction:column; gap:10px; margin-bottom:18px;">
                <div style="border:1px solid var(--border-medium); border-radius:10px; padding:12px; cursor:pointer;" onclick="setDemoUser('admin'); closeModal('loginModal');">
                    <div style="display:flex; justify-content:space-between; align-items:center;">
                        <strong>Merlin Conrad (Admin)</strong>
                        <span class="status-pill warning">System-Architekt</span>
                    </div>
                    <div style="font-size:12px; color:var(--text-secondary); margin-top:4px;">admin@conrad-ki.com · Vollzugriff auf Layout, Theme &amp; Plugins</div>
                </div>

                <div style="border:1px solid var(--border-medium); border-radius:10px; padding:12px; cursor:pointer;" onclick="setDemoUser('team'); closeModal('loginModal');">
                    <div style="display:flex; justify-content:space-between; align-items:center;">
                        <strong>Operations Team (Mitarbeiter)</strong>
                        <span class="status-pill info">User-Modus</span>
                    </div>
                    <div style="font-size:12px; color:var(--text-secondary); margin-top:4px;">team@conrad-ki.com · Kundengespräche, Leads &amp; E-Rechnungen</div>
                </div>
            </div>

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
                <label class="form-label">Name / Ansprechpartner *</label>
                <input type="text" id="nc-name" class="form-input" placeholder="z.B. Stefan Koch">
            </div>
            <div class="form-group">
                <label class="form-label">Unternehmen</label>
                <input type="text" id="nc-company" class="form-input" placeholder="z.B. Koch Sanitär &amp; Heizung">
            </div>
            <div class="form-group">
                <label class="form-label">E-Mail-Adresse</label>
                <input type="email" id="nc-email" class="form-input" placeholder="stefan@koch-sanitaer.de">
            </div>
            <div class="form-group">
                <label class="form-label">Telefonnummer</label>
                <input type="tel" id="nc-phone" class="form-input" placeholder="+49 171 1234567">
            </div>
            <div class="form-group">
                <label class="form-label">Status</label>
                <select id="nc-status" class="form-input">
                    <option value="lead">Lead (Interessent)</option>
                    <option value="qualified">Qualifiziert</option>
                    <option value="customer">Kunde (Aktiv)</option>
                </select>
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:20px;">
                <button class="btn" onclick="closeModal('modalNewContact')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateContact()">Kontakt speichern</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: Neuer Deal ── -->
    <div class="modal-backdrop" id="modalNewDeal">
        <div class="modal-card">
            <div class="modal-header">
                <span class="modal-title">Neuen Verkaufsvorgang (Deal) anlegen</span>
                <button class="modal-close-btn" onclick="closeModal('modalNewDeal')">✕</button>
            </div>
            <div class="form-group">
                <label class="form-label">Kunde / Kontakt *</label>
                <select id="nd-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Titel des Deals *</label>
                <input type="text" id="nd-title" class="form-input" placeholder="z.B. Pilotprojekt KI-Postfachtriage">
            </div>
            <div class="form-group">
                <label class="form-label">Auftragswert (€) *</label>
                <input type="number" id="nd-val" class="form-input" placeholder="890.00" value="890">
            </div>
            <div class="form-group">
                <label class="form-label">Start-Phase</label>
                <select id="nd-stage" class="form-input">
                    <option value="discovery">1. Erstkontakt</option>
                    <option value="proposal">2. Angebot</option>
                    <option value="negotiation">3. Verhandlung</option>
                    <option value="closed_won">4. Gewonnen</option>
                </select>
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:20px;">
                <button class="btn" onclick="closeModal('modalNewDeal')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateDeal()">Deal anlegen</button>
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
                <label class="form-label">Kunde / Rechnungsempfänger *</label>
                <select id="ni-contact-select" class="form-input"></select>
            </div>
            <div class="form-group">
                <label class="form-label">Positionsbeschreibung *</label>
                <input type="text" id="ni-desc" class="form-input" placeholder="z.B. KI-Workflows &amp; Wartung Monatspauschale">
            </div>
            <div class="form-group">
                <label class="form-label">Nettobetrag (€) *</label>
                <input type="number" id="ni-amount" class="form-input" placeholder="99.00" value="99">
            </div>
            <div style="display:flex; justify-content:flex-end; gap:8px; margin-top:20px;">
                <button class="btn" onclick="closeModal('modalNewInvoice')">Abbrechen</button>
                <button class="btn btn-primary" onclick="submitCreateInvoice()">Rechnung buchen</button>
            </div>
        </div>
    </div>

    <!-- ── Modal: GiroCode (EPC-QR) ── -->
    <div class="modal-backdrop" id="modalEpcQr">
        <div class="modal-card" style="text-align:center;">
            <div class="modal-header">
                <span class="modal-title">Europäischer GiroCode (SEPA-QR)</span>
                <button class="modal-close-btn" onclick="closeModal('modalEpcQr')">✕</button>
            </div>
            <p style="font-size:12px; color:var(--text-secondary); margin-bottom:14px;">
                0,00 % Transaktionsgebühren. Kunde scannt direkt mit Sparkasse, Volksbank, DKB, N26 oder Revolut.
            </p>
            <div style="background:#ffffff; padding:16px; border-radius:12px; display:inline-block; margin-bottom:12px; border:1px solid var(--border-subtle);">
                <div style="font-family:var(--font-mono); font-size:11px; color:#1d1d1f;">
                    <div style="font-weight:700; margin-bottom:4px;">EPC / SEPA GIROCODE</div>
                    <div style="font-size:44px; margin:8px 0;">🏁</div>
                    <div style="font-size:10px; color:#86868b;" id="epcQrCodeString">BCD/002/1/SCT/...</div>
                </div>
            </div>
            <div id="epcDetailsBox" style="font-family:var(--font-mono); font-size:12px; color:var(--text-secondary); text-align:left; background:var(--bg-tertiary); padding:10px; border-radius:8px;"></div>
            <div style="text-align:right; margin-top:16px;">
                <button class="btn" onclick="closeModal('modalEpcQr')">Schließen</button>
            </div>
        </div>
    </div>

    <!-- ── Application Logic ── -->
    <script>
        // State
        let currentMode = 'user'; // 'user' or 'admin'
        let currentRole = 'admin'; // 'admin' or 'operator'
        let currentView = 'dashboard';
        let contactsData = [];
        let dealsData = [];
        let invoicesData = [];
        let settingsData = {};

        async function initApp() {
            await loadSettings();
            await refreshData();
            setupKeyboardNav();
        }

        // Mode Switcher: User vs. Admin
        function switchAppMode(mode) {
            currentMode = mode;
            const btnUser = document.getElementById('btnModeUser');
            const btnAdmin = document.getElementById('btnModeAdmin');
            const topbarPill = document.getElementById('topbarModePill');
            const topbarLabel = document.getElementById('topbarModeLabel');

            if (mode === 'admin') {
                btnUser.classList.remove('active');
                btnAdmin.classList.add('active');
                topbarPill.className = 'mode-indicator-pill admin';
                topbarLabel.innerText = 'Admin-Modus (Architekt)';
                document.getElementById('copilotPanelTitle').innerText = 'System Architekt Copilot';
                document.getElementById('copilotPanelSub').innerText = 'LAYOUT & PLUGIN MUTATIONS ACTIVE';
                document.querySelectorAll('.admin-only-badge').forEach(b => b.style.display = 'inline-block');
            } else {
                btnAdmin.classList.remove('active');
                btnUser.classList.add('active');
                topbarPill.className = 'mode-indicator-pill user';
                topbarLabel.innerText = 'User-Modus (Operativ)';
                document.getElementById('copilotPanelTitle').innerText = 'Conrad Copilot';
                document.getElementById('copilotPanelSub').innerText = 'QWEN 1.5B · MCP CONNECTED';
            }
        }

        // View Switcher
        function switchView(viewId) {
            currentView = viewId;
            const sidebar = document.getElementById('appSidebar');
            if (sidebar) sidebar.classList.remove('mobile-open');
            const views = ['dashboard', 'contacts', 'deals', 'invoices', 'retainers', 'marketing', 'dms', 'settings'];
            views.forEach(v => {
                const el = document.getElementById(`view-${v}`);
                if (el) el.style.display = v === viewId ? 'block' : 'none';
                const btn = document.getElementById(`nav-btn-${v}`);
                if (btn) btn.classList.toggle('active', v === viewId);
            });

            const titles = {
                dashboard: 'Betriebs-Cockpit',
                contacts: 'Kontakte & Leads',
                deals: 'Vertriebs-Pipeline',
                invoices: 'Faktura & E-Rechnung',
                retainers: 'Retainer & Abos',
                marketing: 'Marketing & ROI Tracker',
                dms: 'DMS & GoBD Archiv',
                settings: 'System-Architektur & Customizer'
            };

            document.getElementById('currentViewTitle').innerText = titles[viewId] || 'Workspace';

            if (viewId === 'marketing') loadMarketingView();
            if (viewId === 'retainers') loadRetainersView();
            if (viewId === 'dms') loadGobdAuditView();
            if (viewId === 'settings') loadSettingsView();
        }

        // Data Refresh
        async function refreshData() {
            try {
                const [statsRes, contacts, deals, invoices] = await Promise.all([
                    fetch('/mcp', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'tools/call', params: { name: 'erp_get_cashflow_stats' } })
                    }).then(r => r.json()),
                    fetch('/api/contacts').then(r => r.json()),
                    fetch('/api/deals').then(r => r.json()),
                    fetch('/api/invoices').then(r => r.json())
                ]);

                contactsData = contacts || [];
                dealsData = deals || [];
                invoicesData = invoices || [];

                // Badges
                document.getElementById('badge-contacts-count').innerText = contactsData.length;
                document.getElementById('badge-deals-count').innerText = dealsData.length;

                // KPIs
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
                console.error("Data refresh failed:", err);
            }
        }

        // Render Contacts
        function renderDashboardTables() {
            const tbody = document.getElementById('dashboardRecentContacts');
            if (!contactsData.length) {
                tbody.innerHTML = '<tr><td colspan="5" style="text-align:center; color:var(--text-tertiary);">Keine Kontakte vorhanden.</td></tr>';
                return;
            }
            tbody.innerHTML = contactsData.slice(0, 5).map(c => `
                <tr>
                    <td><strong>${escapeHtml(c.name)}</strong></td>
                    <td>${escapeHtml(c.company || '—')}</td>
                    <td>${escapeHtml(c.email || '—')}</td>
                    <td><span class="status-pill ${c.status === 'customer' ? 'success' : 'info'}">${escapeHtml(c.status)}</span></td>
                    <td style="color:var(--text-tertiary); font-size:12px;">${new Date(c.created_at).toLocaleDateString('de-DE')}</td>
                </tr>
            `).join('');
        }

        function renderContactsTable() {
            const tbody = document.getElementById('contactsFullTable');
            if (!contactsData.length) {
                tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; color:var(--text-tertiary);">Keine Kontakte hinterlegt.</td></tr>';
                return;
            }
            tbody.innerHTML = contactsData.map(c => `
                <tr>
                    <td><strong>${escapeHtml(c.name)}</strong></td>
                    <td>${escapeHtml(c.company || '—')}</td>
                    <td>
                        <div>${escapeHtml(c.email || '')}</div>
                        <div style="font-size:11px; color:var(--text-tertiary);">${escapeHtml(c.phone || '')}</div>
                    </td>
                    <td><span class="status-pill ${c.status === 'customer' ? 'success' : 'info'}">${escapeHtml(c.status)}</span></td>
                    <td><span style="font-size:11px; color:var(--text-secondary);">${escapeHtml(c.tags || '—')}</span></td>
                    <td><button class="btn" style="padding:4px 8px; font-size:11px;" onclick="openNewDealModalForContact('${c.id}')">+ Deal</button></td>
                </tr>
            `).join('');
        }

        function filterContactsList() {
            const q = document.getElementById('contactSearchInput').value.toLowerCase();
            const rows = document.querySelectorAll('#contactsFullTable tr');
            rows.forEach(r => {
                r.style.display = r.innerText.toLowerCase().includes(q) ? '' : 'none';
            });
        }

        // Render Deals Kanban
        function renderDealsKanban() {
            const stages = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            const sums = { discovery: 0, proposal: 0, negotiation: 0, closed_won: 0 };

            stages.forEach(s => {
                const col = document.getElementById(`k-items-${s}`);
                if (col) col.innerHTML = '';
            });

            dealsData.forEach(d => {
                const stage = sums[d.stage] !== undefined ? d.stage : 'discovery';
                sums[stage] += d.value_cents;

                const col = document.getElementById(`k-items-${stage}`);
                if (col) {
                    const card = document.createElement('div');
                    card.className = 'kanban-card';
                    card.onclick = () => advanceDealStage(d.id, d.stage);
                    card.innerHTML = `
                        <div class="kanban-card-title">${escapeHtml(d.title)}</div>
                        <div style="font-size:12px; color:var(--text-secondary);">${escapeHtml(d.contact_name || 'Kontakt')}</div>
                        <div class="kanban-card-meta">
                            <strong style="color:var(--accent); font-size:13px;">${(d.value_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</strong>
                            <span>→ Vorrücken</span>
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

        async function advanceDealStage(dealId, currentStage) {
            const flow = ['discovery', 'proposal', 'negotiation', 'closed_won'];
            const idx = flow.indexOf(currentStage);
            if (idx === -1 || idx === flow.length - 1) return;
            const next = flow[idx + 1];

            await fetch('/mcp', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    jsonrpc: '2.0',
                    id: 1,
                    method: 'tools/call',
                    params: { name: 'crm_update_deal_stage', arguments: { deal_id: dealId, new_stage: next } }
                })
            });
            await refreshData();
        }

        // Render Invoices Table
        function renderInvoicesTable() {
            const tbody = document.getElementById('invoicesFullTable');
            if (!invoicesData.length) {
                tbody.innerHTML = '<tr><td colspan="7" style="text-align:center; color:var(--text-tertiary);">Keine Rechnungen vorhanden.</td></tr>';
                return;
            }
            tbody.innerHTML = invoicesData.map(inv => {
                const isPaid = inv.status === 'paid';
                return `
                    <tr>
                        <td><strong>${escapeHtml(inv.invoice_number)}</strong></td>
                        <td>${escapeHtml(inv.contact_name || inv.contact_id)}</td>
                        <td>${escapeHtml(inv.due_date)}</td>
                        <td>${(inv.net_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                        <td><strong>${(inv.gross_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</strong></td>
                        <td><span class="status-pill ${isPaid ? 'success' : 'warning'}">${escapeHtml(inv.status)}</span></td>
                        <td>
                            <div style="display:flex; gap:6px;">
                                <button class="btn" style="padding:4px 8px; font-size:11px;" onclick="toggleInvoiceStatus('${inv.id}', '${inv.status}')">
                                    ${isPaid ? 'Als offen markieren' : 'Als bezahlt'}
                                </button>
                                <a href="/api/invoices/${inv.id}/xrechnung" target="_blank" class="btn" style="padding:4px 8px; font-size:11px; text-decoration:none;">
                                    XML
                                </a>
                                <button class="btn" style="padding:4px 8px; font-size:11px;" onclick="openEpcModal('${inv.invoice_number}', ${inv.gross_cents})">
                                    GiroCode
                                </button>
                            </div>
                        </td>
                    </tr>
                `;
            }).join('');
        }

        async function toggleInvoiceStatus(id, currentStatus) {
            const nextStatus = currentStatus === 'paid' ? 'sent' : 'paid';
            await fetch(`/api/invoices/${id}/status`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ status: nextStatus })
            });
            await refreshData();
        }

        function openEpcModal(invNum, grossCents) {
            const eur = (grossCents / 100).toFixed(2);
            document.getElementById('epcQrCodeString').innerText = `BCD/002/1/SCT/EUR/${eur}/${invNum}`;
            document.getElementById('epcDetailsBox').innerHTML = `
                <div><strong>Empfänger:</strong> Conrad KI / Merlin Conrad</div>
                <div><strong>IBAN:</strong> DE02100100100123456789</div>
                <div><strong>Rechnungsbetrag:</strong> ${eur} €</div>
                <div><strong>Verwendungszweck:</strong> ${invNum}</div>
            `;
            openModal('modalEpcQr');
        }

        // Render Retainers View
        async function loadRetainersView() {
            try {
                const list = await fetch('/api/retainers').then(r => r.json());
                const tbody = document.getElementById('retainersFullTable');
                if (!list.length) {
                    tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; color:var(--text-tertiary);">Keine laufenden Retainer hinterlegt.</td></tr>';
                    return;
                }
                tbody.innerHTML = list.map(r => `
                    <tr>
                        <td><strong>${escapeHtml(r.contact_name || r.contact_id)}</strong></td>
                        <td>${escapeHtml(r.plan_name)}</td>
                        <td><strong style="color:var(--apple-green);">${(r.amount_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</strong></td>
                        <td>Alle ${r.interval_months} Mon.</td>
                        <td><code>${escapeHtml(r.next_billing_date)}</code></td>
                        <td><span class="status-pill ${r.active ? 'success' : 'warning'}">${r.active ? 'Aktiv' : 'Pausiert'}</span></td>
                    </tr>
                `).join('');
            } catch (err) {
                console.error("Retainers load failed:", err);
            }
        }

        async function triggerRetainerRun() {
            if (!confirm("Möchten Sie den automatischen monatlichen Retainer-Abrechnungslauf jetzt starten?")) return;
            try {
                const res = await fetch('/api/retainers/trigger-run', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ actor: `${document.getElementById('ui-user-name').innerText}` })
                }).then(r => r.json());
                alert(`✅ ${res.summary}`);
                await loadRetainersView();
                await refreshData();
            } catch (err) {
                alert("Fehler beim Abrechnungslauf: " + err);
            }
        }

        // Render Marketing & ROI View
        async function loadMarketingView() {
            try {
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

                const tbody = document.getElementById('marketingCampaignsTable');
                if (!campaigns.length) {
                    tbody.innerHTML = '<tr><td colspan="8" style="text-align:center; color:var(--text-tertiary);">Keine Kampagnen erfasst.</td></tr>';
                    return;
                }
                tbody.innerHTML = campaigns.map(c => `
                    <tr>
                        <td><strong>${escapeHtml(c.name)}</strong></td>
                        <td><span class="status-pill neutral">${escapeHtml(c.platform.toUpperCase())}</span></td>
                        <td>${(c.spend_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                        <td>${c.clicks}</td>
                        <td>${c.leads}</td>
                        <td><strong style="color:var(--apple-green);">${c.customers}</strong></td>
                        <td>${(c.revenue_cents / 100).toLocaleString('de-DE', {style:'currency', currency:'EUR'})}</td>
                        <td><span class="status-pill ${c.status === 'active' ? 'success' : 'neutral'}">${escapeHtml(c.status)}</span></td>
                    </tr>
                `).join('');
            } catch (err) {
                console.error("Marketing load failed:", err);
            }
        }

        // Render GoBD Audit View
        async function loadGobdAuditView() {
            try {
                const list = await fetch('/api/gobd/audit-log').then(r => r.json());
                const tbody = document.getElementById('gobdAuditTable');
                if (!list.length) {
                    tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; color:var(--text-tertiary);">Keine Audit-Einträge.</td></tr>';
                    return;
                }
                tbody.innerHTML = list.map(entry => `
                    <tr>
                        <td><code>#${entry.seq}</code></td>
                        <td style="font-size:12px; color:var(--text-tertiary);">${new Date(entry.timestamp).toLocaleString('de-DE')}</td>
                        <td><span class="status-pill neutral">${escapeHtml(entry.action)}</span></td>
                        <td><code>${escapeHtml(entry.entity_id ? entry.entity_id.substring(0, 12) + '...' : '—')}</code></td>
                        <td>${escapeHtml(entry.details)}</td>
                        <td><strong style="font-size:12px;">${escapeHtml(entry.actor)}</strong></td>
                    </tr>
                `).join('');
            } catch (err) {
                console.error("GoBD log failed:", err);
            }
        }

        async function verifyGobdChain() {
            try {
                const res = await fetch('/api/gobd/verify').then(r => r.json());
                const box = document.getElementById('gobdVerifyResult');
                box.style.display = 'block';
                if (res.valid) {
                    box.style.background = 'var(--apple-green-soft)';
                    box.style.color = 'var(--apple-green)';
                    box.innerHTML = `<strong>✅ ${res.message}</strong> (${res.total_entries} Journal-Einträge)`;
                } else {
                    box.style.background = 'var(--apple-red-soft)';
                    box.style.color = 'var(--apple-red)';
                    box.innerHTML = `<strong>⚠️ Revisionsprüfung fehlgeschlagen:</strong> Unstimmigkeit in Hash-Kette.`;
                }
            } catch (err) {
                alert("Verifikation fehlgeschlagen: " + err);
            }
        }

        // Settings & Customizer
        async function loadSettings() {
            try {
                settingsData = await fetch('/api/settings').then(r => r.json()) || {};
                if (settingsData.company_name) {
                    document.getElementById('ui-brand-name').innerText = settingsData.company_name;
                    document.getElementById('settingCompanyName').value = settingsData.company_name;
                }
                if (settingsData.theme_accent) {
                    setThemeAccent(settingsData.theme_accent, false);
                }
                if (settingsData.theme_mode) {
                    document.documentElement.setAttribute('data-theme', settingsData.theme_mode);
                    document.getElementById('themeToggleIcon').innerText = settingsData.theme_mode === 'dark' ? '☀️' : '🌙';
                }
            } catch (e) {
                console.log("Default settings loaded");
            }
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
            alert('Unternehmensname aktualisiert.');
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
            document.getElementById('themeToggleIcon').innerText = next === 'dark' ? '☀️' : '🌙';
            fetch('/api/settings', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ key: 'theme_mode', value: next })
            });
        }

        async function loadSettingsView() {
            try {
                const cfs = await fetch('/api/custom-fields').then(r => r.json());
                const list = document.getElementById('customFieldsRenderList');
                if (!cfs || !cfs.length) {
                    list.innerHTML = 'Keine dynamischen Custom Fields definiert.';
                    return;
                }
                list.innerHTML = cfs.map(f => `• <strong>${escapeHtml(f.label)}</strong> (<code>${escapeHtml(f.key)}</code> · ${f.entity_type})`).join('<br>');
            } catch (err) {
                console.error("Custom fields error:", err);
            }
        }

        async function submitCustomField() {
            const key = document.getElementById('newCfKey').value.trim();
            const label = document.getElementById('newCfLabel').value.trim();
            if (!key || !label) return alert('Bitte Key und Label ausfüllen.');

            await fetch('/api/custom-fields', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    entity_type: 'contact',
                    key,
                    label,
                    field_type: 'text'
                })
            });
            document.getElementById('newCfKey').value = '';
            document.getElementById('newCfLabel').value = '';
            await loadSettingsView();
        }

        // ── Conversational Copilot & Spotlight ──
        function toggleCopilot() {
            document.getElementById('copilotPanel').classList.toggle('open');
            if (document.getElementById('copilotPanel').classList.contains('open')) {
                setTimeout(() => document.getElementById('copilotInput').focus(), 150);
            }
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

            input.value = '';
            toggleCopilot();
            const box = document.getElementById('copilotMessages');
            box.innerHTML += `<div class="chat-bubble user">${escapeHtml(prompt)}</div>`;
            box.scrollTop = box.scrollHeight;

            await handleAiInteraction(prompt, box);
        }

        async function handleAiInteraction(prompt, box) {
            try {
                const res = await fetch('/api/copilot/chat', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ prompt, mode: currentMode, role: currentRole })
                }).then(r => r.json());

                let html = escapeHtml(res.reply).replace(/\n/g, '<br>');

                // If AI executed actions
                if (res.actions_performed && res.actions_performed.length) {
                    html += `<div style="margin-top:8px; font-size:11px; color:var(--apple-green); font-weight:600;">⚡ MCP Tool ausgeführt: ${escapeHtml(res.actions_performed.join(', '))}</div>`;
                }

                // If AI mutated UI in Admin Mode
                if (res.ui_mutations) {
                    if (res.ui_mutations.theme_mode) {
                        document.documentElement.setAttribute('data-theme', res.ui_mutations.theme_mode);
                    }
                    if (res.ui_mutations.theme_accent) {
                        setThemeAccent(res.ui_mutations.theme_accent);
                    }
                    if (res.ui_mutations.refresh_schema) {
                        await loadSettingsView();
                    }
                }

                // If AI requests navigation
                if (res.navigate_to) {
                    switchView(res.navigate_to);
                    html += `<div><button class="chat-nav-chip" onclick="switchView('${res.navigate_to}')">🧭 Zu ${res.navigate_to} wechseln</button></div>`;
                }

                box.innerHTML += `<div class="chat-bubble ai">${html}</div>`;
                box.scrollTop = box.scrollHeight;

                await refreshData();
            } catch (err) {
                box.innerHTML += `<div class="chat-bubble ai" style="color:var(--apple-red);">Fehler bei der Kommunikation mit dem Copilot.</div>`;
            }
        }

        // Demo User Switcher
        function setDemoUser(type) {
            if (type === 'admin') {
                currentRole = 'admin';
                document.getElementById('ui-user-avatar').innerText = 'MC';
                document.getElementById('ui-user-name').innerText = 'Merlin Conrad';
                document.getElementById('ui-user-role').innerText = 'Administrator';
                document.getElementById('demoAccountLabel').innerText = 'admin@conrad-ki.com';
                switchAppMode('admin');
            } else {
                currentRole = 'operator';
                document.getElementById('ui-user-avatar').innerText = 'TM';
                document.getElementById('ui-user-name').innerText = 'Team Operator';
                document.getElementById('ui-user-role').innerText = 'Mitarbeiter';
                document.getElementById('demoAccountLabel').innerText = 'team@conrad-ki.com';
                switchAppMode('user');
            }
        }

        // Modal Helpers
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
            document.getElementById('nd-contact-select').value = contactId;
        }

        function openNewInvoiceModal() {
            const sel = document.getElementById('ni-contact-select');
            sel.innerHTML = contactsData.map(c => `<option value="${c.id}">${escapeHtml(c.name)} (${escapeHtml(c.company || 'Privat')})</option>`).join('');
            openModal('modalNewInvoice');
        }

        async function submitCreateContact() {
            const name = document.getElementById('nc-name').value.trim();
            if (!name) return alert('Bitte Name eingeben');

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
                body: JSON.stringify({
                    title,
                    contact_id,
                    value_cents: Math.round(val * 100),
                    stage
                })
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
                    document.getElementById('spotlightInput').focus();
                    return;
                }
                if (e.key === 'Escape') {
                    document.querySelectorAll('.modal-backdrop').forEach(m => m.style.display = 'none');
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

        window.onload = initApp;
    </script>
</body>
</html>"#
}
