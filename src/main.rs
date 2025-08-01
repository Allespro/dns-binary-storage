use pico_args::Arguments;
use std::error::Error;
use std::fs;
use std::fmt;

mod dns_resolver;
mod data_compressor;

#[derive(Debug)]
enum AppError {
    InvalidArgs(String),
    MissingArgument(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),
            AppError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
        }
    }
}

impl Error for AppError {}

fn print_help() {
    println!("DNS Data Storage Tool

USAGE:
    {} <COMMAND> [OPTIONS]

COMMANDS:
    doh         Use DNS-over-HTTPS resolver
    to-records  Compress data for DNS storage

For 'doh' command:
    {} doh <DOMAIN> --output <OUTPUT_PATH>

For 'to-records' command:
    {} to-records <DOMAIN> --input <INPUT_PATH> --output <OUTPUT_PATH>

OPTIONS:
    -h, --help     Print help information
    -V, --version  Print version information",
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_NAME"));
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print_help();
        return Ok(());
    }

    if args.contains(["-V", "--version"]) {
        print_version();
        return Ok(());
    }

    let command: String = match args.free_from_str() {
        Ok(cmd) => cmd,
        Err(_) => {
            print_help();
            return Ok(());
        }
    };

    match command.as_str() {
        "doh" => {
            let domain: String = args.free_from_str()
                .map_err(|_| AppError::MissingArgument("domain for doh command".to_string()))?;

            let output_path: String = args.value_from_str(["-o", "--output"])
                .map_err(|_| AppError::MissingArgument("--output path".to_string()))?;

            let remaining = args.finish();
            if !remaining.is_empty() {
                return Err(AppError::InvalidArgs(format!("unexpected arguments: {:?}", remaining)).into());
            }

            let result = dns_resolver::doh::resolve(&domain, "https://cloudflare-dns.com/dns-query")?;
            println!("DoH resolution result: {}", result);
            let result_bytes = data_compressor::base64_to_bytes(&result)?;
            fs::write(&output_path, result_bytes)?;
            println!("Saved to: {}", output_path);
        }

        "to-records" => {
            let domain: String = args.free_from_str()
                .map_err(|_| AppError::MissingArgument("domain for to-records command".to_string()))?;

            let input_path: String = args.value_from_str(["-i", "--input"])
                .map_err(|_| AppError::MissingArgument("--input path".to_string()))?;

            let output_path: String = args.value_from_str(["-o", "--output"])
                .map_err(|_| AppError::MissingArgument("--output path".to_string()))?;

            let remaining = args.finish();
            if !remaining.is_empty() {
                return Err(AppError::InvalidArgs(format!("unexpected arguments: {:?}", remaining)).into());
            }

            let data = fs::read(input_path)?;
            let dns_records = data_compressor::bytes_to_dns(&data, &domain, 1, 150)?;
            fs::write(&output_path, dns_records)?;
            println!("Saved to: {}", output_path);
        }

        _ => {
            return Err(AppError::InvalidArgs(format!("unknown command: {}", command)).into());
        }
    }

    Ok(())
}