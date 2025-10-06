
pub type ValidationResult = Result<(), String>;


pub fn validate_required(value: &str, field_name: &str) -> ValidationResult {
    if value.trim().is_empty() {
        return Err(format!("{field_name} is required"));
    }
    Ok(())
}


pub fn validate_email(email: &str) -> ValidationResult {
    if email.trim().is_empty() {
        return Err("Email is required".to_string());
    }
    if !email.contains('@') || !email.contains('.') {
        return Err("Please enter a valid email address".to_string());
    }
    // Additional email validation
    if email.split('@').count() != 2 {
        return Err("Email must contain exactly one @ symbol".to_string());
    }
    Ok(())
}


pub fn validate_password(password: &str) -> ValidationResult {
    if password.is_empty() {
        return Err("Password is required".to_string());
    }
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err("Password must contain uppercase, lowercase, and digit".to_string());
    }

    Ok(())
}


pub fn validate_url(url: &str) -> ValidationResult {
    if url.trim().is_empty() {
        return Err("URL is required".to_string());
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    Ok(())
}


pub fn validate_min_length(value: &str, min: usize, field_name: &str) -> ValidationResult {
    if value.len() < min {
        return Err(format!("{field_name} must be at least {min} characters"));
    }
    Ok(())
}


pub fn validate_max_length(value: &str, max: usize, field_name: &str) -> ValidationResult {
    if value.len() > max {
        return Err(format!("{field_name} must be at most {max} characters"));
    }
    Ok(())
}

pub fn validate_number_range(value: i32, min: i32, max: i32, field_name: &str) -> ValidationResult {
    if value < min || value > max {
        return Err(format!("{field_name} must be between {min} and {max}"));
    }
    Ok(())
}

pub fn validate_all(validators: Vec<ValidationResult>) -> ValidationResult {
    for result in validators {
        result?;
    }
    Ok(())
}


pub fn validate_password_match(password: &str, confirm: &str) -> ValidationResult {
    if password != confirm {
        return Err("Passwords do not match".to_string());
    }
    Ok(())
}


pub fn validate_stellar_public_key(public_key: &str) -> ValidationResult {
    if public_key.trim().is_empty() {
        return Err("Public key is required".to_string());
    }
    if !public_key.starts_with('G') {
        return Err("Stellar public key must start with 'G'".to_string());
    }
    if public_key.len() != 56 {
        return Err("Stellar public key must be 56 characters".to_string());
    }
    Ok(())
}

pub fn validate_future_date(date_str: &str) -> ValidationResult {
    if date_str.trim().is_empty() {
        return Err("Date is required".to_string());
    }
    Ok(())
}

pub fn validate_positive_number(value: i32, field_name: &str) -> ValidationResult {
    if value <= 0 {
        return Err(format!("{field_name} must be a positive number"));
    }
    Ok(())
}
