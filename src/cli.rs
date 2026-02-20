use crate::error::CliError;

/// A trait for working with CLI arguments
pub trait CliConfig: Default {
    fn set_arg(&mut self, flag: &str, value: String) -> Result<(), CliError>;
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
