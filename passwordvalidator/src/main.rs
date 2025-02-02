fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <password> <length>", args[0]);
        return Err("Please provide password and length".to_string());
    }
    let password = &args[1].trim().to_string();
    let length: usize = args[2].trim().parse().expect("Please provide a valid length");
    println!("{}", validate_password(password.as_str(), length));
    Ok(())
}

fn validate_password(password: &str, length: usize) -> String {
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_numeric = password.chars().any(|c| c.is_digit(10));
    let has_special = "!@#$%^&*()_+-=<>?".chars().any(|c| password.contains(c));
    if password.len() >= length && has_uppercase && has_lowercase && has_numeric && has_special {
        return "Password is Strong".to_string();
    } else {
        return "Password is weak".to_string();
    }
}
