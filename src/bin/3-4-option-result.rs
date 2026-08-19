use std::num::ParseIntError;

fn parse_port(raw: &str) -> Result<u16, ParseIntError> {
    let port = raw.parse::<u16>()?;
    Ok(port)
}

fn configured_port(raw: Option<&str>) -> Result<u16, ParseIntError> {
    match raw {
        Some(value) => parse_port(value),
        None => Ok(8080),
    }
}

fn main() -> Result<(), ParseIntError> {
    println!("Configured port: {}", configured_port(Some("9000"))?);
    println!("Default port: {}", configured_port(None)?);
    println!("Invalid input is Err: {}", parse_port("many").is_err());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{configured_port, parse_port};

    #[test]
    fn parses_valid_and_rejects_invalid_ports() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
        assert!(parse_port("many").is_err());
    }

    #[test]
    fn uses_a_default_when_the_option_is_none() {
        assert_eq!(configured_port(None).unwrap(), 8080);
    }
}
