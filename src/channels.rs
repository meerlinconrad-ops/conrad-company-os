#![allow(dead_code)]
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MessageDispatcher: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body_html: &str) -> Result<String>;
    async fn send_whatsapp(&self, phone_e164: &str, message_text: &str) -> Result<String>;
}

pub struct MockDispatcher;

#[async_trait]
impl MessageDispatcher for MockDispatcher {
    async fn send_email(&self, to: &str, subject: &str, _body_html: &str) -> Result<String> {
        tracing::info!("Mock Dispatcher: Email queued for {} (Subject: {})", to, subject);
        Ok(format!("mock-email-{}", to))
    }

    async fn send_whatsapp(&self, phone_e164: &str, message_text: &str) -> Result<String> {
        tracing::info!("Mock Dispatcher: WhatsApp queued for {} (Text: {})", phone_e164, message_text);
        Ok(format!("mock-wa-{}", phone_e164))
    }
}
