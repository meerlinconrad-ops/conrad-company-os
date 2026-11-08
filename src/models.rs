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
// Copilot / Qwen Communication Models
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotRequest {
    pub prompt: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotResponse {
    pub reply: String,
    pub actions_performed: Vec<String>,
    pub model_used: String,
}
