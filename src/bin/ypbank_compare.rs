use rust_parser::cli::{CliConfig, parse_args};
use rust_parser::error::CliError;
use rust_parser::format::format_bin::BinParser;
use rust_parser::format::format_csv::CsvParser;
use rust_parser::format::format_txt::TxtParser;
use rust_parser::parser::Parser;
use rust_parser::storage::YPBankStorage;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::ExitCode;

const USAGE: &str = "Usage: ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv";

#[derive(Default)]
struct Config {
    file1: String,
    format1: String,
    file2: String,
    format2: String,
}

impl CliConfig for Config {
    fn set_arg(&mut self, flag: &str, value: String) -> Result<(), CliError> {
        match flag {
            "file1" => self.file1 = value.clone(),
            "format1" => self.format1 = value.clone(),
            "file2" => self.file2 = value.clone(),
            "format2" => self.format2 = value.clone(),
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
            ("--file1", &self.file1),
            ("--format1", &self.format1),
            ("--file2", &self.file2),
            ("--format2", &self.format2),
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
    match parse_args(&args).and_then(compare_files) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn compare_files(config: Config) -> Result<(), CliError> {
    let storage1 = read_file_format(&config.file1, &config.format1)?;
    let storage2 = read_file_format(&config.file2, &config.format2)?;

    for record in storage1.records() {
        if !storage2.records().contains(record) {
            println!(
                "!!! The transaction records in '{}' and '{}' are NOT IDENTICAL.",
                config.file1, config.file2
            );
            return Ok(());
        }
    }

    println!(
        "The transaction records in '{}' and '{}' are identical.",
        config.file1, config.file2
    );
    Ok(())
}

fn read_file_format(file: &str, format: &str) -> Result<YPBankStorage, CliError> {
    let file = File::open(file).map_err(|e| CliError::IO {
        message: e.to_string(),
        error: e,
    })?;
    let storage = match format {
        "bin" => BinParser::from_read(&mut BufReader::new(file))?,
        "csv" => CsvParser::from_read(&mut BufReader::new(file))?,
        "txt" => TxtParser::from_read(&mut BufReader::new(file))?,
        fmt => {
            return Err(CliError::InvalidFormat {
                name: fmt.to_string(),
            });
        }
    };
    Ok(storage)
}
