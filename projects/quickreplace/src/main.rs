use std::{env, fs, process};

use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).into_owned())
}

fn main() {
    let args = parse_args();

    let data = fs::read_to_string(&args.filename).unwrap_or_else(|error| {
        eprintln!(
            "{} failed to read from file '{}': {error}",
            "Error:".red().bold(),
            args.filename
        );
        process::exit(1);
    });

    let replaced_data = replace(&args.target, &args.replacement, &data).unwrap_or_else(|error| {
        eprintln!("{} failed to replace text: {error}", "Error:".red().bold());
        process::exit(1);
    });

    fs::write(&args.output, replaced_data).unwrap_or_else(|error| {
        eprintln!(
            "{} failed to write to file '{}': {error}",
            "Error:".red().bold(),
            args.output
        );
        process::exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::replace;

    #[test]
    fn replaces_all_matches() {
        assert_eq!(
            replace("world", "Rust", "Hello, world! Goodbye, world!").unwrap(),
            "Hello, Rust! Goodbye, Rust!"
        );
    }

    #[test]
    fn reports_invalid_patterns() {
        assert!(replace("[", "anything", "text").is_err());
    }
}
