pub fn normalize_email(input: &str) -> Option<String> {
    let cleaned: String = input.trim().chars().filter(|c| !is_invisible(*c)).collect();

    let email = cleaned.to_lowercase();

    // validação mínima (sem paranoia)
    if is_basic_valid(&email) {
        Some(email)
    } else {
        None
    }
}

fn is_basic_valid(email: &str) -> bool {
    let mut parts = email.split('@');

    let local = parts.next();
    let domain = parts.next();

    // deve ter exatamente um '@'
    if parts.next().is_some() {
        return false;
    }

    match (local, domain) {
        (Some(l), Some(d)) => {
            !l.is_empty() && !d.is_empty() && d.contains('.') // domínio básico válido
        }
        _ => false,
    }
}

fn is_invisible(c: char) -> bool {
    matches!(c, '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}')
}

/*

fn main() {
    let input = "  User+Test@Example.COM ";

    match normalize_email(input) {
        Some(email) => println!("Normalizado: {}", email),
        None => println!("Email inválido"),
    }
}*/
