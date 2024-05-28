#![allow(clippy::result_large_err)]
use clap::Parser;
use human_format::Formatter;
use indicatif::ProgressBar;
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use regex::Regex;
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

#[derive(Parser)]
struct Cli {
    input: String,
    output: String,
    amount: String,
    /// toggle between human readable and real numbers
    #[arg(short, long)]
    real: bool,
}

use serde::Deserialize;

#[derive(Clone)]
struct Convert {
    input_currency: String,
    output_currency: String,
    amount: f64,
}
#[derive(Deserialize)]
struct Currency {
    rates: HashMap<String, f64>,
}

#[derive(Error, Debug)]
enum ConvertError {
    #[error("unknown currency: '{0}'")]
    UnknownCurrency(String),
    #[error("request returned code: '{0}'")]
    HttpError(u16),
    #[error("failed to send request to external api")]
    RequestError,
}
impl Convert {
    fn new(i: &str, o: &str, a: f64) -> Self {
        Self {
            input_currency: i.into(),
            output_currency: o.to_uppercase(),
            amount: a,
        }
    }
    fn convert(self) -> Result<f64, ConvertError> {
        let currency = self.deserialise()?;

        match currency.rates.get(&self.output_currency) {
            Some(currency) => {
                let result = currency * self.amount;
                Ok(result)
            }
            None => Err(ConvertError::UnknownCurrency(self.output_currency)),
        }
    }
    fn request(&self) -> Result<String, ureq::Error> {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("sending request to api...");
        let url = format!(
            "https://api.exchangerate-api.com/v4/latest/{}",
            self.input_currency
        );
        let req = match ureq::get(&url).timeout(Duration::from_secs(30)).call() {
            Ok(req) => {
                bar.finish_and_clear();
                req
            }
            Err(ureq::Error::Status(code, response)) => {
                return Err(ureq::Error::Status(code, response))
            }
            Err(e) => {
                return Err(e);
            }
        };
        Ok(req.into_string()?)
    }
    fn deserialise(&self) -> Result<Currency, ConvertError> {
        let req = match self.request() {
            Ok(req) => req,
            Err(ureq::Error::Status(code, ..)) => {
                if code == 404 {
                    return Err(ConvertError::UnknownCurrency(self.input_currency.clone()));
                } else {
                    return Err(ConvertError::HttpError(code));
                }
            }
            Err(_) => return Err(ConvertError::RequestError),
        };

        let currency: Currency = serde_json::from_str(&req).expect("valid json");

        Ok(currency)
    }
}
// parser
#[derive(Error, Debug)]
enum ParseNumError {
    #[error("Invalid Number")]
    InvalidNumber,
    #[error("Invalid Suffix")]
    InvalidSuffix,
    #[error("Invalid Format")]
    InvalidFormat,
}
fn parse_number(s: &str) -> Result<f64, ParseNumError> {
    let re = Regex::new(r"(?i)^(\d+(\.\d+)?)([kmbt]?)$").unwrap();
    if let Some(caps) = re.captures(s) {
        let number: f64 = caps[1].parse().map_err(|_| ParseNumError::InvalidNumber)?;
        let multiplier = match &caps[3].to_lowercase()[..] {
            "k" => 1_000.0,
            "m" => 1_000_000.0,
            "b" => 1_000_000_000.0,
            "t" => 1_000_000_000_000.0,
            "" => 1.0,
            _ => return Err(ParseNumError::InvalidSuffix),
        };
        Ok(number * multiplier)
    } else {
        Err(ParseNumError::InvalidFormat)
    }
}
// main
fn main() -> Result<()> {
    let cli = Cli::parse();
    let parsed_amount = parse_number(&cli.amount).into_diagnostic()?;
    let convert = Convert::new(&cli.input, &cli.output, parsed_amount);
    let output = convert.clone().convert().into_diagnostic()?;

    // show real numbers if 'real' arg is present
    let (input, output) = if cli.real {
        (parsed_amount.to_string(), output.to_string())
    } else {
        // otherwise humanify output (default)
        let f_input = Formatter::new()
            .with_decimals(2)
            .with_separator("")
            .format(parsed_amount);
        let f_output = Formatter::new()
            .with_decimals(2)
            .with_separator("")
            .format(output);
        (f_input, f_output)
    };

    println!(
        "{} {} = {} {}",
        // pretty input
        input,
        &convert.input_currency.to_uppercase().bold(),
        // pretty output
        output,
        &convert.output_currency.bold(),
    );
    Ok(())
}
