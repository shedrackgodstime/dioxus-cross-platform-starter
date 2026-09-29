use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Username must be between 3 and 24 alphanumeric characters")]
    InvalidUsername,
    #[error("Invalid email address format")]
    InvalidEmail,
    #[error("Input cannot be empty")]
    EmptyInput,
}

/// Pure email validator without regex or system libraries.
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    let email = email.trim();
    if email.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(ValidationError::InvalidEmail);
    }

    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return Err(ValidationError::InvalidEmail);
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(ValidationError::InvalidEmail);
    }

    Ok(())
}

/// Pure username validator.
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let username = username.trim();
    if username.len() < 3 || username.len() > 24 {
        return Err(ValidationError::InvalidUsername);
    }

    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(ValidationError::InvalidUsername);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_valid_username() {
        assert!(validate_username("rustacean").is_ok());
        assert!(validate_username("ab").is_err());
        assert!(validate_username("bad name with spaces!").is_err());
    }
}
