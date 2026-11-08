import os
import sys
import time
import json
import urllib.request
from playwright.sync_api import sync_playwright

BASE_URL = os.environ.get("CONRAD_BASE_URL", "http://127.0.0.1:8090")

output_dirs = [
    os.path.expanduser("~/.gemini/antigravity/brain/3240b892-2591-4d13-8741-e0423ecb8af4/screenshots"),
    os.path.expanduser("~/.gemini/antigravity/brain/80ebcafc-9630-479e-8d45-4b0579cbc677/screenshots"),
]
for d in output_dirs:
    os.makedirs(d, exist_ok=True)

def save_shot(page, filename, full_page=False):
    for d in output_dirs:
        page.screenshot(path=f"{d}/{filename}", full_page=full_page)
    print(f"✓ Captured {filename}")

def mcp_call(tool_name, arguments=None):
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments or {}
        }
    }
    req = urllib.request.Request(
        f"{BASE_URL}/mcp",
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    with urllib.request.urlopen(req, timeout=10) as resp:
        data = json.loads(resp.read().decode("utf-8"))
        assert "result" in data, f"MCP error on {tool_name}: {data}"
        return data["result"]

print(f"🚀 Running Full E2E Verification Suite against {BASE_URL} ...")

# ── 0. Verify MCP Tools ──
for tool, args in [
    ("erp_get_cashflow_stats", {}),
    ("crm_search_contacts", {"query": ""}),
    ("marketing_get_roi", {}),
    ("retainer_list_subscriptions", {}),
    ("gobd_verify_audit_log", {}),
    ("harness_list_plugins", {}),
]:
    res = mcp_call(tool, args)
    print(f"✓ MCP tool verified: {tool}")

rec_res = mcp_call("harness_list_plugin_records", {"plugin_id": "plugin.realestate"})
print("✓ MCP tool verified: harness_list_plugin_records (plugin.realestate)")

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)

    # ── 1. Desktop Suite (1440x900) ──
    page = browser.new_page(viewport={"width": 1440, "height": 900})
    page.goto(BASE_URL, wait_until="networkidle")
    time.sleep(0.8)

    save_shot(page, "desktop_01_dashboard.png", full_page=True)

    # Odoo 9-Dot App Matrix Modal
    page.click("#odooAppLauncherBtn")
    time.sleep(0.4)
    save_shot(page, "desktop_01b_app_matrix.png")
    page.keyboard.press("Escape")
    time.sleep(0.3)

    # Contacts View
    page.click("#nav-btn-contacts")
    time.sleep(0.5)
    save_shot(page, "desktop_02_contacts.png", full_page=True)

    # Open 360° Contact Inspector Drawer
    page.click("#contactsFullTable tr:first-child")
    time.sleep(0.6)
    save_shot(page, "desktop_02b_contact_drawer_360.png")
    page.keyboard.press("Escape")
    time.sleep(0.3)

    # Open Cmd+K Spotlight Command Palette
    page.click(".cmdk-trigger-btn")
    time.sleep(0.4)
    save_shot(page, "desktop_02c_cmdk_palette.png")
    page.keyboard.press("Escape")
    time.sleep(0.3)

    # Deals Kanban
    page.click("#nav-btn-deals")
    time.sleep(0.5)
    save_shot(page, "desktop_03_deals.png", full_page=True)

    # Invoices & GoBD
    page.click("#nav-btn-invoices")
    time.sleep(0.5)
    save_shot(page, "desktop_04_invoices.png", full_page=True)

    # Retainers
    page.click("#nav-btn-retainers")
    time.sleep(0.5)
    save_shot(page, "desktop_05_retainers.png", full_page=True)

    # Marketing Attribution
    page.click("#nav-btn-marketing")
    time.sleep(0.5)
    save_shot(page, "desktop_06_marketing.png", full_page=True)

    # Workflows & Automation
    page.click("#nav-btn-workflows")
    time.sleep(0.5)
    save_shot(page, "desktop_06b_workflows.png", full_page=True)

    # Plugin Studio (Odoo x DeepSeek Harness)
    page.click("#nav-btn-plugins")
    time.sleep(0.5)
    save_shot(page, "desktop_06c_plugin_studio.png", full_page=True)

    # Dynamic Plugin: Real Estate (Immobilien & Mietobjekte)
    page.click("#nav-btn-plug-realestate")
    time.sleep(0.5)
    save_shot(page, "desktop_06d_plugin_realestate.png", full_page=True)

    # Dynamic Plugin: Projects (Projekte & Zeiterfassung)
    page.click("#nav-btn-plug-projects")
    time.sleep(0.5)
    save_shot(page, "desktop_06e_plugin_projects.png", full_page=True)

    # System & Schema
    page.click("#nav-btn-settings")
    time.sleep(0.5)
    save_shot(page, "desktop_07_system.png", full_page=True)

    # AI Copilot via Omni-Bar (Self-Building Plugin / Query)
    page.fill("#aiOmniInput", "Zeige mir alle Immobilien und Projekte")
    page.keyboard.press("Enter")
    time.sleep(0.8)
    save_shot(page, "desktop_08_copilot_active.png")

    # Native Cal.com Booking Page (/book/merlin)
    page.goto(f"{BASE_URL}/book/merlin", wait_until="networkidle")
    time.sleep(0.5)
    save_shot(page, "desktop_09_booking_calcom.png")
    page.close()

    # ── 2. Mobile Suite (iPhone 15 Pro: 393 x 852) ──
    mob = browser.new_page(
        viewport={"width": 393, "height": 852},
        user_agent="Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1"
    )
    mob.goto(BASE_URL, wait_until="networkidle")
    time.sleep(0.8)

    save_shot(mob, "mobile_01_dashboard.png", full_page=True)

    # Open mobile menu
    mob.click("#mobileMenuBtn")
    time.sleep(0.4)
    save_shot(mob, "mobile_02_sidebar_open.png")

    # Navigate to Contacts via mobile menu
    mob.click("#nav-btn-contacts")
    time.sleep(0.5)
    save_shot(mob, "mobile_03_contacts.png", full_page=True)

    # Navigate to Deals
    mob.click("#mobileMenuBtn")
    time.sleep(0.3)
    mob.click("#nav-btn-deals")
    time.sleep(0.5)
    save_shot(mob, "mobile_04_deals.png", full_page=True)

    # Navigate to Invoices
    mob.click("#mobileMenuBtn")
    time.sleep(0.3)
    mob.click("#nav-btn-invoices")
    time.sleep(0.5)
    save_shot(mob, "mobile_05_invoices.png", full_page=True)

    # Open Copilot drawer on Mobile
    mob.click(".btn-apple-ai")
    time.sleep(0.5)
    save_shot(mob, "mobile_06_copilot.png")

    mob.close()
    browser.close()

print("\n🎉 ALL MCP TOOLS, DESKTOP VIEWS, DYNAMIC PLUGINS, AND MOBILE SCREENS VERIFIED!")
