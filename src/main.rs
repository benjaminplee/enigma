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

use std::collections::HashMap;

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
        ("encode-io", Some(_)) => {
            command_encode_io();
        }
        ("encode-dir", Some(sub_m)) => {
            command_encode_dir(
                sub_m.value_of("source").unwrap(),
                sub_m.value_of("destination").unwrap(),
            );
        }
        ("stats-io", Some(_)) => {
            stats_io();
        }
        ("decode-io", Some(_)) => {
            command_decode_io();
        }
        _ => unreachable!("Unknown subcommand"),
    }
}

fn command_encode_io() {
    info!("Running ENCODE-IO subcommand");

    let machine = enigma::State::new_random();

    info!("Encoding with {}", machine.show());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.expect("Error reading from STDIN: {}");
        trace!("Read from STDIN: {}", text);
        let output = machine.encode(&text);
        println!("{}", output);
    }
}

fn command_decode_io() {
    info!("Running DECODE-IO subcommand");

    let states = enigma::StateSet::new();

    debug!(
        "Running through {} states for first pass",
        enigma::StateSet::MAX_STATES
    );

    for state in states {
        trace!("Trying: {}", state.show());
    }

    // let machine = enigma::State::new_random();
    //
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let text = line.expect("Error reading from STDIN: {}");
    //     trace!("Read from STDIN: {}", text);
    //     let output = machine.encode(&text);
    //     println!("{}", output);
    // }
}

fn command_encode_dir(source: &str, dest: &str) {
    info!(
        "Running ENCODE-DIR subcommand for source: {} and dest: {}",
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

fn stats_io() {
    info!("Running STATS-IO subcommand");

    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin
        .read_to_string(&mut buffer)
        .expect("Error reading from STDIN");

    println!("STDIN STATS");
    println!("  Bytes read: {}", buffer.len());

    let (char_count, num_chars) = gen_stats(buffer);

    println!(
        "  Character Counts ({} unique ascii alpha present):",
        num_chars
    );
    for c in (b'A'..=b'Z').map(char::from) {
        let count = char_count.get(&c).ok_or(0).expect("Character error");
        let percent = 100.0 * *count as f64 / num_chars as f64;

        println!(
            "    {} {:12} {:5.1} {:-^4$}",
            c,
            count,
            percent,
            "",
            percent as usize * 2
        );
    }
}

fn gen_stats(buffer: String) -> (HashMap<char, u32>, u32) {
    let mut char_count = HashMap::new();
    let mut num_chars = 0;

    for character in buffer.chars().filter(|c| c.is_ascii() && c.is_alphabetic()) {
        let count = char_count
            .entry(character.to_ascii_uppercase())
            .or_insert(0);
        *count += 1;
        num_chars += 1;
    }

    (char_count, num_chars)
}
