#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub status: String, // "lead", "qualified", "customer", "churned"
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub contact_id: String,
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
pub struct DocumentMeta {
    pub id: String,
    pub contact_id: Option<String>,
    pub filename: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionMessage {
    pub id: String,
    pub contact_id: String,
    pub channel: String, // "email", "whatsapp", "call", "meeting"
    pub direction: String, // "inbound", "outbound"
    pub subject: Option<String>,
    pub body: String,
    pub status: String, // "draft", "sent", "delivered", "read"
    pub timestamp: DateTime<Utc>,
}
