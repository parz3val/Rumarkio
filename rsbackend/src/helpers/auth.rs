use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
pub fn encrypt_password(string: String) -> Result<String, BcryptError> {
    match hash(string, DEFAULT_COST) {
        Ok(hashed) => Ok(hashed),
        Err(e) => Err(e),
    }
}

pub fn verify_password(
    password_string: String,
    password_hash: String,
) -> Result<bool, BcryptError> {
    match verify(password_string, &password_hash) {
        Ok(matched) => Ok(matched),
        Err(e) => Err(e),
    }
}
