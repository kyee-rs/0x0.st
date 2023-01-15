use clap::{value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use colored::Colorize;
use std::{io, path::PathBuf};

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
    let matches = build_cli().get_matches();

    // -------------------------------------------------------------------------------------
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        print_completions(generator, &mut cmd);
    }
    // -------------------------------------------------------------------------------------

    let file = matches.get_one::<PathBuf>("file");

    // Check if the user provided a file
    if file.is_none() {
        eprintln!("{}", "Error: you did not provide any file!".bold().red());
        return;
    }

    // Create the multipart form
    let form = reqwest::blocking::multipart::Form::new().file("file", file.unwrap());
    if form.is_err() {
        eprintln!("{}", "Error creating the form.".bold().red());
        return;
    }

    let formunwrap = form.unwrap();

    println!("- {}", "Uploading file...".bold().yellow());

    // Make Post request to 0x0.st
    let client = reqwest::blocking::Client::new();
    let response = client.post("https://0x0.st").multipart(formunwrap).send();
    // Check for the errors
    if let Err(err) = response {
        eprintln!(
            "{}",
            err.to_string()
                .replace("error sending request for url (https://0x0.st/): ", "")
        );
        return;
    }
    println!("- {}", "Done!".bold().green());
    println!("- {}", response.unwrap().text().unwrap());
}
