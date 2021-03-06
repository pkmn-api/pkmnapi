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

/// Generate hash string
///
/// # Example
///
/// ```
/// use pkmnapi_sql::utils;
///
/// let hash = utils::hash(&"bar".to_owned().into_bytes());
///
/// assert_eq!(hash, "37b51d194a7513e45b56f6524f2d51f2");
/// ```
pub fn hash(value: &Vec<u8>) -> String {
    format!("{:02x}", md5::compute(value))
}

/// Generate eTag
///
/// # Example
///
/// ```
/// use pkmnapi_sql::utils;
///
/// let etag = utils::etag(&"bar".to_owned().into_bytes());
///
/// assert_eq!(etag, "w/\"37b51d194a7513e45b56f6524f2d51f2\"");
/// ```
pub fn etag(body: &Vec<u8>) -> String {
    let content = hash(body);

    format!("w/\"{}\"", content)
}
