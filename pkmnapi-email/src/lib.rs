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

use dotenv::dotenv;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};
use std::env;

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
    /// Panics if the `SMTP_USER` or `SMTP_PASS` environment variables are not set
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

        env::var("SMTP_USER").expect("SMTP_USER must be set");
        env::var("SMTP_PASS").expect("SMTP_PASS must be set");

        PkmnapiEmail {
            to_email: to_email.to_string(),
            template,
        }
    }

    pub fn send(&self) -> Result<(), String> {
        let smtp_user = env::var("SMTP_USER").unwrap();
        let smtp_pass = env::var("SMTP_PASS").unwrap();

        let email = match EmailBuilder::new()
            .to(self.to_email.clone())
            .from(Mailbox::new_with_name(
                "Pkmnapi".to_owned(),
                smtp_user.clone(),
            ))
            .subject(self.template.subject())
            .text(self.template.body())
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
}

impl PkmnapiEmailTemplate {
    pub fn subject(&self) -> String {
        match self {
            PkmnapiEmailTemplate::AccessToken(_) => "Pkmnapi access token".to_owned(),
        }
    }

    pub fn body(&self) -> String {
        match self {
            PkmnapiEmailTemplate::AccessToken(access_token) => {
                format!("This is your Pkmnapi access token: {}", access_token)
            }
        }
    }
}
