use std::borrow::Cow;
use validator::ValidationError;

const MIN_PASSWORD_LENGTH: usize = 12;

/// Validates the strength of a password based on specific criteria.
///
/// This function ensures that a given password:
/// 1. Has a minimum length of 12 characters.
/// 2. Contains at least one uppercase letter.
/// 3. Contains at least one lowercase letter.
/// 4. Contains at least one numeric digit.
/// 5. Contains at least one symbol (non-alphanumeric character).
pub fn password_strength(password: &str) -> Result<(), ValidationError> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ValidationError::new("password_too_short")
            .with_message(Cow::from("Password must be at least 12 characters long")));
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_number = false;
    let mut has_symbol = false;

    for c in password.chars() {
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_ascii_digit() {
            has_number = true;
        } else if !c.is_alphanumeric() {
            has_symbol = true;
        }

        if has_uppercase && has_lowercase && has_number && has_symbol {
            return Ok(());
        }
    }

    let validation_err = ValidationError::new("password_not_complex")
        .with_message(Cow::from("Password must contain all 4 of the following categories: uppercase, lowercase, numbers, and symbols."));
    Err(validation_err)
}
