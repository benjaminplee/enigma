extern crate env_logger;
use env_logger::Env;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

use std::fs;
use std::path::Path;

use std::io;
use std::io::prelude::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .get_matches();

    let level = match matches.occurrences_of("verbose") {
        0 => "OFF",
        1 => "INFO",
        2 => "DEBUG",
        3 | _ => "TRACE",
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(level)).init();

    match matches.subcommand() {
        ("io", Some(_)) => {
            command_io();
        }
        ("dir", Some(sub_m)) => {
            command_dir(
                sub_m.value_of("source").unwrap(),
                sub_m.value_of("destination").unwrap(),
            );
        }
        _ => unreachable!("Unknown subcommand"),
    }
}

fn command_io() {
    info!("Running IO subcommand");

    let machine = enigma::State::new_random();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.expect("Error reading from STDIN: {}");
        trace!("Read from STDIN: {}", text);
        let output = machine.encode(&text);
        println!("{}", output);
    }
}

fn command_dir(source: &str, dest: &str) {
    info!(
        "Running DIR subcommand for source: {} and dest: {}",
        source, dest
    );

    let machine = enigma::State::new_random();

    let source_path = Path::new(source);
    let dest_path = Path::new(dest);

    debug!("Ensuring dest directory and parents exist");
    fs::create_dir_all(dest_path).expect("Unable to create parent directory");

    for entry in fs::read_dir(source_path).expect("Unable to list source directory") {
        let in_path = entry.expect("unable to get entry in source").path();

        if in_path.is_dir() {
            trace!("Skipping {} as directory", in_path.display());
        } else {
            let out_path = dest_path.join(in_path.file_name().expect("Unable to find file name"));

            info!(
                "Processing file: {} to {}",
                in_path.display(),
                out_path.display()
            );

            let text =
                fs::read_to_string(in_path).expect("Something went wrong reading the source file");

            let output = machine.encode(&text);

            fs::write(out_path, output).expect("Unable to write dest file");
        }
    }
}
