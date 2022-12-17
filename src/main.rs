#![allow(clippy::useless_format)]
use clap::{value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use colored::Colorize;
use std::io;
/// Simple program to upload file to 0x0.st

fn build_cli() -> Command {
    Command::new("0x0")
        .bin_name("0x0")
        .arg(
            Arg::new("file")
                .help("File to upload")
                .alias("f")
                .action(ArgAction::Set)
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .alias("g")
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let matches = build_cli().get_matches(); // Get the arguments from the user
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        print_completions(generator, &mut cmd);
        std::process::exit(0);
    } // Generate the completions

    let file = matches.get_one::<String>("file"); // Get the file path
    if file.is_none() {
        println!(
            "{}",
            format!("Error: you did not provide any file!").bold().red()
        );
        std::process::exit(0);
    }

    // Create the multipart form
    let uploader = reqwest::blocking::multipart::Form::new().file("file", file.unwrap());
    if uploader.is_err() {
        // Check for the errors
        println!(
            "{}",
            format!("Error! Check the file path again.").bold().red()
        );
        std::process::exit(1);
    } else if uploader.is_ok() {
        // Or print the uploading message
        println!("- {}", format!("Uploading file...").bold().yellow());
        println!("- {}", format!("Done!").bold().green());
    }

    // Make Post request to 0x0.st
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://0x0.st")
        .multipart(uploader.unwrap())
        .send();

    // Check for the errors
    if res.is_err() {
        println!(
            "{}",
            format!("Error! Check the file path again.").bold().red()
        );
        std::process::exit(1);
    }
    println!("{}", res.unwrap().text().unwrap());
}
