//! Utils module

use hex;
use hmac::{Hmac, Mac, NewMac};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha2::Sha256;
use std::env;
use std::iter;

type HmacSha256 = Hmac<Sha256>;

/// Generate random ID of size length
///
/// # Example
///
/// ```
/// use pkmnapi_sql::utils::*;
///
/// let id = random_id(32);
///
/// assert_eq!(id.len(), 32);
///
/// let id = random_id(64);
///
/// assert_eq!(id.len(), 64);
/// ```
pub fn random_id(length: usize) -> String {
    let mut rng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(length)
        .collect()
}

/// Generate HMAC string
///
/// # Panics
///
/// Panics if the `SECRET_KEY` environment variable is not set
///
/// # Example
///
/// ```
/// use pkmnapi_sql::utils;
/// use std::env;
///
/// env::set_var("SECRET_KEY", "foo");
///
/// let hash = utils::hmac(&String::from("bar"));
///
/// assert_eq!(
///     hash,
///     "f9320baf0249169e73850cd6156ded0106e2bb6ad8cab01b7bbbebe6d1065317"
/// );
/// ```
pub fn hmac(value: &String) -> String {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let mut mac = HmacSha256::new_varkey(&secret_key.into_bytes()).unwrap();

    mac.update(value.as_bytes());

    let result = mac.finalize();

    hex::encode(result.into_bytes())
}
