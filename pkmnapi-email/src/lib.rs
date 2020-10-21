//! Pkmnapi email module
//!
//! # Example
//!
//! ```
//! use pkmnapi_email::*;
//!
//! let email = PkmnapiEmail::new(
//!     &"foo@bar.com".to_owned(),
//!     PkmnapiEmailTemplate::AccessToken("baz".to_owned()),
//! );
//! ```

extern crate dotenv;
extern crate lettre;
extern crate lettre_email;
extern crate tera;

use dotenv::dotenv;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};
use std::env;
use tera::Tera;

static BASE_HTML: &'static str = include_str!("../templates/base.html");
static ACCESS_TOKEN_TXT: &'static str = include_str!("../templates/access_token.txt");
static ACCESS_TOKEN_HTML: &'static str = include_str!("../templates/access_token.html");
static DELETE_CODE_TXT: &'static str = include_str!("../templates/delete_code.txt");
static DELETE_CODE_HTML: &'static str = include_str!("../templates/delete_code.html");

/// PkmnapiEmail
///
/// # Example
///
/// ```
/// use pkmnapi_email::*;
///
/// let email = PkmnapiEmail::new(
///     &"foo@bar.com".to_owned(),
///     PkmnapiEmailTemplate::AccessToken("baz".to_owned()),
/// );
/// ```
pub struct PkmnapiEmail {
    to_email: String,
    template: PkmnapiEmailTemplate,
}

impl PkmnapiEmail {
    /// Create new PkmnapiEmail
    ///
    /// # Panics
    ///
    /// Panics if the `FROM_EMAIL`, `SMTP_USER`, or `SMTP_PASS` environment variables are not set
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_email::*;
    ///
    /// let email = PkmnapiEmail::new(
    ///     &"foo@bar.com".to_owned(),
    ///     PkmnapiEmailTemplate::AccessToken("baz".to_owned()),
    /// );
    /// ```
    pub fn new(to_email: &String, template: PkmnapiEmailTemplate) -> PkmnapiEmail {
        dotenv().ok();

        env::var("FROM_EMAIL").expect("FROM_EMAIL must be set");
        env::var("SMTP_USER").expect("SMTP_USER must be set");
        env::var("SMTP_PASS").expect("SMTP_PASS must be set");

        PkmnapiEmail {
            to_email: to_email.to_string(),
            template,
        }
    }

    pub fn send(&self) -> Result<(), String> {
        let from_email = env::var("FROM_EMAIL").unwrap();
        let smtp_user = env::var("SMTP_USER").unwrap();
        let smtp_pass = env::var("SMTP_PASS").unwrap();

        let subject = self.template.subject()?;
        let body_html = self.template.body_html(&subject)?;
        let body_text = self.template.body_text()?;

        let email = match EmailBuilder::new()
            .to(self.to_email.clone())
            .from(Mailbox::new_with_name("Pkmnapi".to_owned(), from_email))
            .subject(subject)
            .alternative(body_html, body_text)
            .build()
        {
            Ok(email) => email.into(),
            Err(_) => return Err("Could not build email".to_owned()),
        };

        let credentials = (smtp_user, smtp_pass).into_credentials();

        let mut client = match SmtpClient::new_simple("smtp.gmail.com") {
            Ok(client) => client.credentials(credentials).transport(),
            Err(_) => return Err("Could not establish email connection".to_owned()),
        };

        match client.send(email) {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not send email".to_owned()),
        }
    }
}

pub enum PkmnapiEmailTemplate {
    AccessToken(String),
    DeleteCode(String, String),
}

impl PkmnapiEmailTemplate {
    pub fn subject(&self) -> Result<String, String> {
        let subject = match self {
            PkmnapiEmailTemplate::AccessToken(_) => "Pkmnapi access token".to_owned(),
            PkmnapiEmailTemplate::DeleteCode(_, _) => "Pkmnapi delete access token".to_owned(),
        };

        Ok(subject)
    }

    pub fn body_html(&self, subject: &String) -> Result<String, String> {
        let content = match self {
            PkmnapiEmailTemplate::AccessToken(access_token) => {
                let mut context = tera::Context::new();

                context.insert("access_token", &access_token);

                let content = match Tera::one_off(ACCESS_TOKEN_HTML, &context, true) {
                    Ok(content) => content,
                    Err(e) => return Err(e.to_string()),
                };

                content
            }
            PkmnapiEmailTemplate::DeleteCode(email_address, delete_code) => {
                let mut context = tera::Context::new();

                context.insert("email_address", &email_address);
                context.insert("delete_code", &delete_code);

                let content = match Tera::one_off(DELETE_CODE_HTML, &context, true) {
                    Ok(content) => content,
                    Err(e) => return Err(e.to_string()),
                };

                content
            }
        };

        let mut context = tera::Context::new();

        context.insert("subject", &subject);
        context.insert("content", &content);

        let body = match Tera::one_off(BASE_HTML, &context, true) {
            Ok(body) => body,
            Err(e) => return Err(e.to_string()),
        };

        Ok(body)
    }

    pub fn body_text(&self) -> Result<String, String> {
        let body = match self {
            PkmnapiEmailTemplate::AccessToken(access_token) => {
                let mut context = tera::Context::new();

                context.insert("access_token", &access_token);

                let body = match Tera::one_off(ACCESS_TOKEN_TXT, &context, true) {
                    Ok(body) => body,
                    Err(e) => return Err(e.to_string()),
                };

                body
            }
            PkmnapiEmailTemplate::DeleteCode(email_address, delete_code) => {
                let mut context = tera::Context::new();

                context.insert("email_address", &email_address);
                context.insert("delete_code", &delete_code);

                let body = match Tera::one_off(DELETE_CODE_TXT, &context, true) {
                    Ok(body) => body,
                    Err(e) => return Err(e.to_string()),
                };

                body
            }
        };

        Ok(body)
    }
}
