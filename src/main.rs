#![allow(clippy::useless_format)]
use clap::Parser;
use colored::Colorize;

/// Simple program to upload file to 0x0.st
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = reqwest::blocking::multipart::Form::new().file("file", args.file);
    if file.is_err() {
        println!(
            "{}",
            format!("Error! Check the file path again.").bold().red()
        );
        std::process::exit(1);
    } else if file.is_ok() {
        println!("{}", format!("Uploading file...").bold().green());
    }
    // Make Post request to 0x0.st
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://0x0.st")
        .multipart(file.unwrap())
        .send();
    // Unwrap the response
    let result = res.as_ref();
    let error = result.unwrap_err();
    if reqwest::Error::is_status(error) {
        println!(
            "{}",
            format!("Error! Check the file path again.").bold().red()
        );
        std::process::exit(1);
    }
    println!("URL: {}", res.unwrap().text().unwrap());
}
