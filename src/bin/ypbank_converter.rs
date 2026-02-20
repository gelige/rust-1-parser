use rust_parser::cli::{CliConfig, parse_args};
use rust_parser::error::CliError;
use rust_parser::format::format_bin::BinParser;
use rust_parser::format::format_csv::CsvParser;
use rust_parser::format::format_txt::TxtParser;
use rust_parser::parser::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::process::ExitCode;

const USAGE: &str = "Usage: ypbank_converter --input <file> --input-format <fmt> --output-format <fmt> [--output <file>]";

#[derive(Default)]
struct Config {
    input: String,
    input_format: String,
    output_format: String,
    output: String,
}

impl CliConfig for Config {
    fn set_arg(&mut self, flag: &str, value: String) -> Result<(), CliError> {
        match flag {
            "input" => self.input = value.clone(),
            "input-format" => self.input_format = value.clone(),
            "output-format" => self.output_format = value.clone(),
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
        for (flag, val) in [
            ("--input", &self.input),
            ("--input-format", &self.input_format),
            ("--output-format", &self.output_format),
        ] {
            if val.is_empty() {
                return Err(CliError::MissingArgument {
                    name: flag.to_string(),
                });
            }
        }
        Ok(())
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args).and_then(convert) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn convert(config: Config) -> Result<(), CliError> {
    // Read from file
    let file = File::open(&config.input).map_err(|e| CliError::IO {
        message: e.to_string(),
    })?;
    let storage = match config.input_format.as_str() {
        "bin" => BinParser::from_read(&mut BufReader::new(file))?,
        "csv" => CsvParser::from_read(&mut BufReader::new(file))?,
        "txt" => TxtParser::from_read(&mut BufReader::new(file))?,
        fmt => {
            return Err(CliError::InvalidFormat {
                name: fmt.to_string(),
            });
        }
    };

    // Write to file
    let mut writer: Box<dyn Write> = if config.output.is_empty() {
        let stdout = io::stdout();
        Box::new(BufWriter::new(stdout.lock()))
    } else {
        let file = File::create(&config.output).map_err(|e| CliError::IO {
            message: e.to_string(),
        })?;
        Box::new(BufWriter::new(file))
    };

    match config.output_format.as_str() {
        "bin" => BinParser::from_storage(storage).write_to(&mut writer)?,
        "csv" => CsvParser::from_storage(storage).write_to(&mut writer)?,
        "txt" => TxtParser::from_storage(storage).write_to(&mut writer)?,
        fmt => {
            return Err(CliError::InvalidFormat {
                name: fmt.to_string(),
            });
        }
    }
    Ok(())
}
