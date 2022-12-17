#![allow(clippy::useless_format)]
use clap::{value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use colored::Colorize;
use std::{io, path::PathBuf};
use uuid::Uuid;
/// Simple program to upload file to 0x0.st

fn build_cli() -> Command {
    Command::new("0x0")
        .bin_name("0x0")
        .arg(
            Arg::new("file")
                .help("File to upload")
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Set)
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("expires")
                .help("Expire time in hours (e.g. 3)")
                .short('e')
                .long("expires")
                .value_parser(value_parser!(i8))
                .action(ArgAction::Set)
                .value_hint(ValueHint::Other),
        )
        .arg(
            Arg::new("secret")
                .help("Use UUIDv4 to cypher your url. [personal recommendation]")
                .action(ArgAction::SetTrue)
                .short('s'),
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .short('g')
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let matches = build_cli().get_matches(); // Get the arguments from the user

    // -------------------------------------------------------------------------------------
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        print_completions(generator, &mut cmd);
        std::process::exit(0);
    }
    // -------------------------------------------------------------------------------------

    let expires = matches.get_one::<i8>("expires"); // Get the expire time
    let secret = matches.get_one::<bool>("secret"); // Get the secret state
    let file = matches.get_one::<PathBuf>("file"); // Get the file path

    // Check if the user provided a file
    if file.is_none() {
        println!(
            "{}",
            format!("Error: you did not provide any file!").bold().red()
        );
        std::process::exit(0);
    }

    // Create the multipart form
    let mut form = reqwest::blocking::multipart::Form::new();
    form = form.file("file", file.unwrap()).unwrap();
    if let Some(expires) = expires {
        form = form.text("expires", expires.to_string());
    }
    if secret.is_some() {
        form = form.text("secret", Uuid::new_v4().to_string());
    }

    // Print the uploading message
    println!("- {}", format!("Uploading file...").bold().yellow());
    println!("- {}", format!("Done!").bold().green());

    // Make Post request to 0x0.st
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://0x0.st").multipart(form).send();

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
