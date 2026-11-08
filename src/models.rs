#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// =========================================================================
// CRM Models
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub status: String, // "lead", "qualified", "customer", "churned"
    pub tags: String,   // comma separated or JSON array
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewContact {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub status: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub contact_id: String,
    pub contact_name: Option<String>,
    pub title: String,
    pub value_cents: i64,
    pub currency: String,
    pub stage: String, // "discovery", "proposal", "negotiation", "closed_won", "closed_lost"
    pub expected_close_date: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDeal {
    pub contact_id: String,
    pub title: String,
    pub value_cents: i64,
    pub currency: Option<String>,
    pub stage: Option<String>,
    pub expected_close_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionMessage {
    pub id: String,
    pub contact_id: String,
    pub channel: String,   // "email", "whatsapp", "call", "meeting", "note"
    pub direction: String, // "inbound", "outbound"
    pub subject: Option<String>,
    pub body: String,
    pub status: String, // "draft", "sent", "delivered", "read"
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInteraction {
    pub contact_id: String,
    pub channel: String,
    pub direction: String,
    pub subject: Option<String>,
    pub body: String,
}

// =========================================================================
// ERP & Invoicing Models
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductService {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_cents: i64,
    pub vat_rate_percent: f64,
    pub unit: String, // "Monat", "Stunde", "Pauschal", "Stück"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProductService {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_cents: i64,
    pub vat_rate_percent: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price_cents: i64,
    pub total_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String, // e.g. "RE-2026-001"
    pub contact_id: String,
    pub contact_name: Option<String>,
    pub deal_id: Option<String>,
    pub status: String, // "draft", "sent", "paid", "overdue", "cancelled"
    pub issued_date: String,
    pub due_date: String,
    pub net_cents: i64,
    pub vat_cents: i64,
    pub gross_cents: i64,
    pub items_json: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInvoice {
    pub contact_id: String,
    pub deal_id: Option<String>,
    pub invoice_number: Option<String>,
    pub items: Vec<InvoiceItem>,
    pub due_days: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashflowStats {
    pub total_contacts: i64,
    pub active_leads: i64,
    pub pipeline_volume_cents: i64,
    pub paid_revenue_cents: i64,
    pub pending_invoices_cents: i64,
    pub deals_won_count: i64,
}

// =========================================================================
// DMS (Document Management System) Models
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub id: String,
    pub contact_id: Option<String>,
    pub invoice_id: Option<String>,
    pub title: String,
    pub filename: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub category: String, // "contract", "invoice", "offer", "receipt", "general"
    pub tags: String,
    pub extracted_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDocument {
    pub contact_id: Option<String>,
    pub invoice_id: Option<String>,
    pub title: String,
    pub filename: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub extracted_text: Option<String>,
}

// =========================================================================
// Custom Fields (Dynamic Customization Engine)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDef {
    pub id: String,
    pub entity_type: String, // "contact", "deal", "invoice", "document"
    pub key: String,         // e.g. "branche", "steuernummer", "prioritaet"
    pub label: String,       // e.g. "Branche", "USt-IdNr.", "Priorität"
    pub field_type: String,  // "text", "number", "select", "boolean"
    pub options_json: Option<String>, // for select fields: '["KMU", "Handwerk", "Enterprise"]'
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCustomFieldDef {
    pub entity_type: String,
    pub key: String,
    pub label: String,
    pub field_type: String,
    pub options: Option<Vec<String>>,
}

// =========================================================================
// Booking & Scheduling (Cal.com in Pure Rust)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingSlot {
    pub id: String,
    pub slug: String,
    pub contact_id: Option<String>,
    pub guest_name: String,
    pub guest_email: String,
    pub guest_phone: Option<String>,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub status: String, // "confirmed", "cancelled"
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBooking {
    pub slug: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub start_time: String, // ISO-8601
    pub duration_minutes: Option<i64>,
    pub timezone: Option<String>,
    pub notes: Option<String>,
}

// =========================================================================
// Phase 2: HubSpot Parity — Workflows, Tasks, Webhooks & Auth
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRule {
    pub id: String,
    pub name: String,
    pub trigger_type: String,   // e.g. "deal_stage_changed", "contact_created", "inbound_message"
    pub trigger_filter: String, // e.g. "proposal", "lead", "whatsapp"
    pub action_type: String,    // e.g. "create_task_and_email_draft", "assign_tag"
    pub delay_days: i64,
    pub template_subject: Option<String>,
    pub template_body: String,
    pub active: bool,
    pub executions_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkflowRule {
    pub name: String,
    pub trigger_type: String,
    pub trigger_filter: String,
    pub action_type: String,
    pub delay_days: Option<i64>,
    pub template_subject: Option<String>,
    pub template_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFollowUpTask {
    pub contact_id: String,
    pub deal_id: Option<String>,
    pub title: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub status: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpTask {
    pub id: String,
    pub contact_id: String,
    pub deal_id: Option<String>,
    pub title: String,
    pub due_date: String,
    pub status: String, // "open", "completed"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppWebhookPayload {
    pub from: String,        // E.164 phone or name
    pub sender_name: Option<String>,
    pub text: String,
    pub message_id: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailInboundPayload {
    pub from_email: String,
    pub from_name: Option<String>,
    pub subject: String,
    pub body: String,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionPayload {
    pub role: String, // "admin" or "operator"
    pub email: Option<String>,
    pub password: Option<String>,
}

// =========================================================================
// Self-Hosted Payment Gateway (YouPI-PG in Rust / SEPA GiroCode)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction {
    pub id: String,
    pub invoice_id: String,
    pub amount_cents: i64,
    pub payment_method: String, // "sepa_instant", "giro_code", "bank_transfer"
    pub status: String,         // "pending", "confirmed", "failed"
    pub reference: String,
    pub matched_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotRequest {
    pub prompt: String,
    pub context: Option<String>,
    pub mode: Option<String>, // "user" or "admin"
    pub role: Option<String>, // "admin" or "operator"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotResponse {
    pub reply: String,
    pub actions_performed: Vec<String>,
    pub model_used: String,
    pub navigate_to: Option<String>,
    pub ui_mutations: Option<serde_json::Value>,
}

// =========================================================================
// Germany Compliance & Holzhey Company OS Extensions
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GobdAuditEntry {
    pub id: String,
    pub seq: i64,
    pub timestamp: DateTime<Utc>,
    pub entity_type: String, // "invoice", "payment", "contact", "deal"
    pub entity_id: String,
    pub action: String,      // "CREATE", "LOCK", "STATUS_CHANGE", "STORNO", "DATEV_EXPORT"
    pub details: String,
    pub actor: String,
    pub prev_hash: String,
    pub entry_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StornoRequest {
    pub invoice_id: String,
    pub reason: String,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpotImportResult {
    pub imported_count: usize,
    pub skipped_count: usize,
    pub errors: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpotCsvPayload {
    pub csv_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatevExportConfig {
    pub year: i32,
    pub month: Option<i32>,
    pub skr: String, // "skr03" or "skr04"
    pub consultant_number: Option<i64>,
    pub client_number: Option<i64>,
}

// =========================================================================
// Funnel Lead & Acquisition Engine
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelLeadPayload {
    pub email: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub notes: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelLeadResponse {
    pub success: bool,
    pub contact_id: String,
    pub deal_id: String,
    pub message: String,
}

// =========================================================================
// Marketing Ad Campaign Tracker
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingCampaign {
    pub id: String,
    pub name: String,
    pub platform: String, // "meta", "google", "linkedin", "organic"
    pub spend_cents: i64,
    pub clicks: i64,
    pub leads: i64,
    pub customers: i64,
    pub revenue_cents: i64,
    pub status: String, // "active", "paused", "completed"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMarketingCampaign {
    pub name: String,
    pub platform: String,
    pub spend_cents: i64,
    pub clicks: Option<i64>,
    pub leads: Option<i64>,
    pub customers: Option<i64>,
    pub revenue_cents: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingRoiSummary {
    pub total_spend_eur: f64,
    pub total_leads: i64,
    pub total_customers: i64,
    pub total_revenue_eur: f64,
    pub blended_cpc_eur: f64,
    pub blended_cac_eur: f64,
    pub roas: f64,
}

// =========================================================================
// Recurring Retainer Billing (ZUGFeRD 2.2 / GoBD Autopilot)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetainerSubscription {
    pub id: String,
    pub contact_id: String,
    pub contact_name: Option<String>,
    pub plan_name: String,
    pub amount_cents: i64,
    pub vat_rate_percent: f64,
    pub interval_months: i64,
    pub next_billing_date: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRetainerSubscription {
    pub contact_id: String,
    pub plan_name: String,
    pub amount_cents: i64,
    pub vat_rate_percent: Option<f64>,
    pub interval_months: Option<i64>,
    pub next_billing_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetainerBillingRunResult {
    pub billed_count: usize,
    pub generated_invoices: Vec<String>,
    pub total_amount_eur: f64,
    pub summary: String,
}
