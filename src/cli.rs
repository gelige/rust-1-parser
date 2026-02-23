//! Command-line argument parsing utilities

use crate::error::CliError;

/// A trait for working with CLI arguments
pub trait CliConfig: Default {
    /// Set an argument and its value
    fn set_arg(&mut self, flag: &str, value: String) -> Result<(), CliError>;

    /// Validate configuration arguments
    fn validate_args(&self) -> Result<(), CliError>;
}

/// Parses command-line arguments into configuration object
pub fn parse_args<T: CliConfig>(args: &[String]) -> Result<T, CliError> {
    let mut config = T::default();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        let flag = arg
            .strip_prefix("--")
            .ok_or_else(|| CliError::UnknownArgument { name: arg.clone() })?;
        let value = args
            .get(i + 1)
            .ok_or_else(|| CliError::MissingValue { name: arg.clone() })?;
        config.set_arg(flag, value.to_string())?;
        i += 2;
    }
    config.validate_args()?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestConfig {
        input: String,
        output: String,
    }

    impl CliConfig for TestConfig {
        fn set_arg(&mut self, flag: &str, value: String) -> Result<(), CliError> {
            match flag {
                "input" => self.input = value.clone(),
                "output" => self.output = value.clone(),
                _ => {
                    return Err(CliError::UnknownArgument {
                        name: format!("--{}", flag),
                    });
                }
            }
            Ok(())
        }

        fn validate_args(&self) -> Result<(), CliError> {
            for (flag, val) in [("--input", &self.input), ("--output", &self.output)] {
                if val.is_empty() {
                    return Err(CliError::MissingArgument {
                        name: flag.to_string(),
                    });
                }
            }
            Ok(())
        }
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_known_flags() {
        let result =
            parse_args::<TestConfig>(&args(&["prog", "--input", "a.csv", "--output", "b.csv"]));
        let config = result.unwrap();
        assert_eq!(config.input, "a.csv");
        assert_eq!(config.output, "b.csv");
    }

    #[test]
    fn returns_error_for_unknown_argument() {
        let result = parse_args::<TestConfig>(&args(&["prog", "--input", "a.csv", "--foo", "bar"]));
        assert!(matches!(result, Err(CliError::UnknownArgument { .. })));
    }

    #[test]
    fn returns_error_when_value_is_missing() {
        let result = parse_args::<TestConfig>(&args(&["prog", "--input"]));
        assert!(matches!(result, Err(CliError::MissingValue { .. })));
    }
}
