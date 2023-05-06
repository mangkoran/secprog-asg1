use crate::db::*;
use crate::models::User;

/// Returns a hashed password using [Sha512](https://www.akkadia.org/drepper/SHA-crypt.txt) method.
///
/// # Arguments
/// * - `password` - A password string to be hashed
pub fn encrypt_password(password: &str) -> String {
    use sha_crypt::{sha512_check, sha512_crypt_b64, Sha512Params};

    const SALT: &str = "seasalt";
    const ROUNDS: usize = 10_000;

    // First setup the Sha512Params arguments with:
    let params = Sha512Params::new(ROUNDS).expect("RandomError!");

    // Hash the password for storage
    // Format = $<ID>$<SALT>$<HASH>
    let hash =
        sha512_crypt_b64(password.as_bytes(), SALT.as_bytes(), &params).expect("Should not fail");
    let extended_hash = format!("$6$rounds={ROUNDS}${SALT}${hash}");

    // Verify the hashed password
    assert!(sha512_check(password, extended_hash.as_str()).is_ok());

    // Return the hash only
    return hash;
}

pub fn check_user(username: &str, password: &str) -> Result<User, bool> {
    let user = get_user(username).expect("Error loading user");

    let hashed_password = encrypt_password(password);

    if user.password == hashed_password {
        Ok(user)
    } else {
        Err(false)
    }
}
