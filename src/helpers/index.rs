use regex::Regex;
use uuid::Uuid;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;
use std::io;

pub fn generate_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string().replace("-", "")
}

pub fn verify_and_match_password(password: &str, confirm_password: &str) -> Result<(), String> {
    if password.len() < 6 || password.len() > 12 {
        return Err("Password should have characters between 6 to 12".to_string());
    }

    if password != confirm_password {
        return Err("Password don't match".to_string());
    }

    Ok(())
}

pub fn is_email_valid(str: &str) -> bool {
    Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$")
        .unwrap()
        .is_match(str)
}

pub fn is_not_empty(str: &str) -> bool {
    !str.trim().is_empty()
}

pub fn hash_password(password: &str) -> io::Result<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> io::Result<bool> {
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false), // Password mismatch
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }
}
