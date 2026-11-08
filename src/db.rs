use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::{Contact, Deal, InteractionMessage, NewContact, NewDeal};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // High-performance WAL mode & Pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA cache_size = -64000;
             PRAGMA temp_store = MEMORY;",
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                company TEXT,
                status TEXT NOT NULL DEFAULT 'lead',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS deals (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                title TEXT NOT NULL,
                value_cents INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'EUR',
                stage TEXT NOT NULL DEFAULT 'discovery',
                expected_close_date TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                contact_id TEXT,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size_bytes INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                channel TEXT NOT NULL,
                direction TEXT NOT NULL,
                subject TEXT,
                body TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'sent',
                timestamp TEXT NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email);
            CREATE INDEX IF NOT EXISTS idx_deals_stage ON deals(stage);
            CREATE INDEX IF NOT EXISTS idx_messages_contact ON messages(contact_id);",
        )?;
        Ok(())
    }

    pub fn create_contact(&self, input: NewContact) -> Result<Contact> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let status = input.status.unwrap_or_else(|| "lead".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO contacts (id, name, email, phone, company, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                input.name,
                input.email,
                input.phone,
                input.company,
                status,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        Ok(Contact {
            id,
            name: input.name,
            email: input.email,
            phone: input.phone,
            company: input.company,
            status,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn search_contacts(&self, query: &str) -> Result<Vec<Contact>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, name, email, phone, company, status, created_at, updated_at
             FROM contacts
             WHERE name LIKE ?1 OR email LIKE ?1 OR company LIKE ?1 OR phone LIKE ?1
             ORDER BY updated_at DESC
             LIMIT 50",
        )?;

        let contacts = stmt
            .query_map([pattern], |row| {
                let created_str: String = row.get(6)?;
                let updated_str: String = row.get(7)?;
                Ok(Contact {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    phone: row.get(3)?,
                    company: row.get(4)?,
                    status: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(contacts)
    }

    pub fn list_contacts(&self) -> Result<Vec<Contact>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, email, phone, company, status, created_at, updated_at
             FROM contacts
             ORDER BY updated_at DESC
             LIMIT 100",
        )?;

        let contacts = stmt
            .query_map([], |row| {
                let created_str: String = row.get(6)?;
                let updated_str: String = row.get(7)?;
                Ok(Contact {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    phone: row.get(3)?,
                    company: row.get(4)?,
                    status: row.get(5)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(contacts)
    }

    pub fn create_deal(&self, input: NewDeal) -> Result<Deal> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let currency = input.currency.unwrap_or_else(|| "EUR".to_string());
        let stage = input.stage.unwrap_or_else(|| "discovery".to_string());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO deals (id, contact_id, title, value_cents, currency, stage, expected_close_date, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                input.contact_id,
                input.title,
                input.value_cents,
                currency,
                stage,
                input.expected_close_date,
                now.to_rfc3339(),
            ],
        )?;

        Ok(Deal {
            id,
            contact_id: input.contact_id,
            title: input.title,
            value_cents: input.value_cents,
            currency,
            stage,
            expected_close_date: input.expected_close_date,
            created_at: now,
        })
    }

    pub fn list_deals(&self) -> Result<Vec<Deal>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, contact_id, title, value_cents, currency, stage, expected_close_date, created_at
             FROM deals
             ORDER BY created_at DESC",
        )?;

        let deals = stmt
            .query_map([], |row| {
                let created_str: String = row.get(7)?;
                Ok(Deal {
                    id: row.get(0)?,
                    contact_id: row.get(1)?,
                    title: row.get(2)?,
                    value_cents: row.get(3)?,
                    currency: row.get(4)?,
                    stage: row.get(5)?,
                    expected_close_date: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(deals)
    }

    pub fn add_message(&self, contact_id: &str, channel: &str, direction: &str, subject: Option<&str>, body: &str) -> Result<InteractionMessage> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (id, contact_id, channel, direction, subject, body, status, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'delivered', ?7)",
            params![id, contact_id, channel, direction, subject, body, now.to_rfc3339()],
        )?;

        Ok(InteractionMessage {
            id,
            contact_id: contact_id.to_string(),
            channel: channel.to_string(),
            direction: direction.to_string(),
            subject: subject.map(String::from),
            body: body.to_string(),
            status: "delivered".to_string(),
            timestamp: now,
        })
    }
}
