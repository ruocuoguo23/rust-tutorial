use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn print_suit(card: PokerSuit) {
    println!("{card:?}");
}

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

fn first_word(raw: Option<&str>) -> Option<&str> {
    raw?.split_whitespace().next()
}

#[derive(Debug)]
pub enum PortError {
    Parse(ParseIntError),
    Zero,
}

impl fmt::Display for PortError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "invalid port: {error}"),
            Self::Zero => write!(formatter, "port zero is excluded by this example's policy"),
        }
    }
}

impl Error for PortError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Zero => None,
        }
    }
}

impl From<ParseIntError> for PortError {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

fn service_port(raw: &str) -> Result<u16, PortError> {
    let port = parse_port(raw)?; // From converts the parse error, without logging or retrying.
    if port == 0 {
        return Err(PortError::Zero);
    }
    Ok(port)
}

pub fn run() -> Result<(), PortError> {
    print_suit(PokerSuit::Clubs);
    print_suit(PokerSuit::Spades);
    print_suit(PokerSuit::Hearts);
    print_suit(PokerSuit::Diamonds);

    println!("Configured port: {}", configured_port(Some("9000"))?);
    println!("Default port: {}", configured_port(None)?);
    println!("Invalid input is Err: {}", parse_port("many").is_err());
    println!(
        "Option ?: {:?}, {:?}",
        first_word(Some("hello rust")),
        first_word(None)
    );
    Ok(())
}

pub fn error_design() {
    for raw in ["8080", "0", "many", "70000"] {
        match service_port(raw) {
            Ok(port) => println!("Service port: {port}"),
            Err(error) => println!("{raw}: {error}; has source={}", error.source().is_some()),
        }
    }
    // parse_port("0") is valid u16 parsing; rejection is our service policy, not a Rust rule.
    // parse_port("many").unwrap(); // Would panic; recoverable input errors use Result.
}

#[cfg(test)]
mod tests {
    use super::{configured_port, first_word, parse_port, service_port, PortError};
    use std::error::Error;

    #[test]
    fn parses_valid_and_rejects_invalid_ports() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
        assert!(parse_port("many").is_err());
        assert_eq!(parse_port("0").unwrap(), 0);
        assert_eq!(parse_port("65535").unwrap(), 65535);
        assert!(parse_port("65536").is_err());
        assert!(parse_port("-1").is_err());
        assert!(parse_port("").is_err());
    }

    #[test]
    fn uses_a_default_when_the_option_is_none() {
        assert_eq!(configured_port(None).unwrap(), 8080);
        assert_eq!(configured_port(Some("9000")).unwrap(), 9000);
        assert!(configured_port(Some("many")).is_err());
    }

    #[test]
    fn option_question_mark_preserves_missing_and_empty_inputs() {
        assert_eq!(first_word(None), None);
        assert_eq!(first_word(Some("  ")), None);
        assert_eq!(first_word(Some(" hello rust")), Some("hello"));
    }

    #[test]
    fn distinguishes_parse_failures_from_policy_failures() {
        assert_eq!(service_port("1").unwrap(), 1);
        assert_eq!(service_port("65535").unwrap(), 65535);
        let zero = service_port("0").unwrap_err();
        assert!(matches!(zero, PortError::Zero));
        assert!(zero.source().is_none());
        for raw in ["many", "65536", ""] {
            let error = service_port(raw).unwrap_err();
            assert!(matches!(error, PortError::Parse(_)));
            assert!(error.source().is_some());
        }
    }
}
