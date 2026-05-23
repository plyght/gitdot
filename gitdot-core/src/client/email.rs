use std::time::Duration;

use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::error::EmailError;

const SMTP_TIMEOUT: Duration = Duration::from_secs(15);

#[async_trait]
pub trait EmailClient: Send + Sync + Clone + 'static {
    async fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        html: &str,
    ) -> Result<(), EmailError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SmtpTlsMode {
    Implicit,
    StartTls,
    None,
}

#[derive(Debug, Clone)]
pub struct SmtpClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub fn new(
        host: &str,
        port: u16,
        username: String,
        password: SecretString,
        tls: SmtpTlsMode,
    ) -> Result<Self, EmailError> {
        let builder = match tls {
            SmtpTlsMode::Implicit => AsyncSmtpTransport::<Tokio1Executor>::relay(host)?,
            SmtpTlsMode::StartTls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)?,
            SmtpTlsMode::None => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host),
        };
        let transport = builder
            .port(port)
            .timeout(Some(SMTP_TIMEOUT))
            .credentials(Credentials::new(
                username,
                password.expose_secret().to_string(),
            ))
            .build();
        Ok(Self { transport })
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl EmailClient for SmtpClient {
    async fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        html: &str,
    ) -> Result<(), EmailError> {
        let from: Mailbox = from.parse()?;
        let to: Mailbox = to.parse()?;
        let message = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html.to_string())?;
        self.transport.send(message).await?;
        Ok(())
    }
}
