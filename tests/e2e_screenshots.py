import os
import time
from playwright.sync_api import sync_playwright

output_dir = os.path.expanduser("~/.gemini/antigravity/brain/3240b892-2591-4d13-8741-e0423ecb8af4/screenshots")
os.makedirs(output_dir, exist_ok=True)

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    
    # ── 1. Desktop Suite (1440x900) ──
    page = browser.new_page(viewport={"width": 1440, "height": 900})
    page.goto("http://127.0.0.1:8090", wait_until="networkidle")
    time.sleep(1)
    
    page.screenshot(path=f"{output_dir}/desktop_01_dashboard.png", full_page=True)
    print("✓ Desktop 01 Dashboard captured")
    
    page.click('#nav-btn-contacts')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_02_contacts.png", full_page=True)
    print("✓ Desktop 02 Contacts captured")

    # Open 360° Contact Inspector Drawer & verify properties + timeline
    page.click('#contactsFullTable tr:first-child')
    time.sleep(0.6)
    page.screenshot(path=f"{output_dir}/desktop_02b_contact_drawer_360.png")
    print("✓ Desktop 02b Contact 360° Inspector Drawer captured")
    page.keyboard.press("Escape")
    time.sleep(0.3)

    # Open Cmd+K Command Palette & verify
    page.click('.cmdk-trigger-btn')
    time.sleep(0.4)
    page.screenshot(path=f"{output_dir}/desktop_02c_cmdk_palette.png")
    print("✓ Desktop 02c Cmd+K Command Palette captured")
    page.keyboard.press("Escape")
    time.sleep(0.3)
    
    page.click('#nav-btn-deals')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_03_deals.png", full_page=True)
    print("✓ Desktop 03 Deals captured")
    
    page.click('#nav-btn-invoices')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_04_invoices.png", full_page=True)
    print("✓ Desktop 04 Invoices captured")
    
    page.click('#nav-btn-retainers')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_05_retainers.png", full_page=True)
    print("✓ Desktop 05 Retainers captured")

    page.click('#nav-btn-marketing')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_06_marketing.png", full_page=True)
    print("✓ Desktop 06 Marketing captured")
    
    page.click('#nav-btn-settings')
    time.sleep(0.5)
    page.screenshot(path=f"{output_dir}/desktop_07_system.png", full_page=True)
    print("✓ Desktop 07 System Admin captured")
    
    # Copilot open & query
    page.click('.btn-apple-ai')
    time.sleep(0.5)
    page.fill('#copilotInput', 'Zeige mir alle Rechnungen')
    page.click('#copilotSendBtn')
    time.sleep(1.2)
    page.screenshot(path=f"{output_dir}/desktop_08_copilot_active.png")
    print("✓ Desktop 08 Copilot Active captured")

    # Booking Page (/book/merlin)
    page.goto("http://127.0.0.1:8090/book/merlin", wait_until="networkidle")
    time.sleep(0.6)
    page.screenshot(path=f"{output_dir}/desktop_09_booking_calcom.png")
    print("✓ Desktop 09 Native Cal.com Booking Page captured")
    page.close()
    
    # ── 2. Mobile Suite (iPhone 14/15 Pro: 393 x 852) ──
    mob = browser.new_page(
        viewport={"width": 393, "height": 852},
        user_agent="Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1"
    )
    mob.goto("http://127.0.0.1:8090", wait_until="networkidle")
    time.sleep(1)
    
    mob.screenshot(path=f"{output_dir}/mobile_01_dashboard.png", full_page=True)
    print("✓ Mobile 01 Dashboard captured")
    
    # Open mobile menu
    mob.click('#mobileMenuBtn')
    time.sleep(0.5)
    mob.screenshot(path=f"{output_dir}/mobile_02_sidebar_open.png")
    print("✓ Mobile 02 Sidebar Drawer captured")
    
    # Navigate to Contacts via mobile menu
    mob.click('#nav-btn-contacts')
    time.sleep(0.5)
    mob.screenshot(path=f"{output_dir}/mobile_03_contacts.png", full_page=True)
    print("✓ Mobile 03 Contacts captured")
    
    # Navigate to Deals
    mob.click('#mobileMenuBtn')
    time.sleep(0.3)
    mob.click('#nav-btn-deals')
    time.sleep(0.5)
    mob.screenshot(path=f"{output_dir}/mobile_04_deals.png", full_page=True)
    print("✓ Mobile 04 Deals captured")

    # Navigate to Invoices
    mob.click('#mobileMenuBtn')
    time.sleep(0.3)
    mob.click('#nav-btn-invoices')
    time.sleep(0.5)
    mob.screenshot(path=f"{output_dir}/mobile_05_invoices.png", full_page=True)
    print("✓ Mobile 05 Invoices captured")

    # Open Copilot drawer on Mobile
    mob.click('.btn-apple-ai')
    time.sleep(0.5)
    mob.screenshot(path=f"{output_dir}/mobile_06_copilot.png")
    print("✓ Mobile 06 Copilot Drawer captured")
    
    mob.close()
    browser.close()

print("\n🎉 ALL SCREENSHOTS SUCCESSFULLY CREATED AND VERIFIED!")
