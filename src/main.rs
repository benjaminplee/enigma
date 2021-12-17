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

    let machine = enigma::machine::State::new_random();
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

    let states = enigma::machine::StateSet::new();
    let mut best_state_by_freq = enigma::machine::State::new_random();
    let mut best_freq = 2600.0;

    let mut count = 0;

    debug!(
        "Running through {} states for first pass",
        enigma::machine::StateSet::MAX_STATES
    );

    let mut stdin = io::stdin();
    let mut buffer = String::new();
    let bytes = stdin
        .read_to_string(&mut buffer)
        .expect("Problem reading from STDIN");

    debug!("Read {} bytes from STDIN", bytes);

    for state in states {
        debug!("Trying: {}", state.show());

        let output = state.encode(&buffer);

        let (_, _, freq_delta) = gen_stats(&output);

        if freq_delta < best_freq {
            best_freq = freq_delta;
            best_state_by_freq = state;

            debug!("New best state found {} < {}", freq_delta, best_freq);
        }

        if count % 10 == 0 {
            info!(
                "{:>10} Best state by frequency ({}) so far: {}",
                count,
                best_freq,
                best_state_by_freq.show()
            );
        }

        if output.len() < 60 {
            trace!("  OUTPUT: [{}]", output);
        } else {
            trace!("  OUTPUT SAMPLE: [{}]", (&output[..60]));
        }

        count += 1;
    }
}

fn command_encode_dir(source: &str, dest: &str) {
    info!(
        "Running ENCODE-DIR subcommand for source: {} and dest: {}",
        source, dest
    );

    let machine = enigma::machine::State::new_random();
    info!("Encoding with {}", machine.show());

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

    let (char_count, num_chars, freq_delta) = gen_stats(&buffer);

    println!(
        "  Character Counts ({} unique ascii alpha present):",
        num_chars
    );
    for c in enigma::machine::ALPHABET {
        let count = char_count.get(&c).ok_or(0).expect("Character error");
        let percent = 100.0 * *count as f64 / num_chars as f64;

        println!(
            "    {} {:12} {:>7.3} {:-^4$}",
            c,
            count,
            percent,
            "",
            percent as usize * 2
        );
    }

    println!("  Character frequency delta: {}", freq_delta);
}

const NORMAL_PERCENT: [f64; 26] = [
    7.856, 1.671, 2.306, 4.915, 12.038, 2.052, 2.249, 6.435, 6.399, 0.238, 1.014, 4.034, 2.423,
    6.794, 7.887, 1.661, 0.060, 5.299, 5.946, 9.770, 3.041, 0.805, 2.656, 0.140, 2.160, 0.051,
];

fn gen_stats(buffer: &String) -> (HashMap<char, u32>, u32, f64) {
    let mut char_count = HashMap::new();
    let mut num_chars = 0;

    for character in buffer.chars().filter(|c| c.is_ascii() && c.is_alphabetic()) {
        let count = char_count
            .entry(character.to_ascii_uppercase())
            .or_insert(0);
        *count += 1;
        num_chars += 1;
    }

    let mut freq_delta: f64 = 0.0;

    for (k, v) in &char_count {
        let expected = NORMAL_PERCENT[((*k as u8) - b'A') as usize];
        let actual = 100.0 * *v as f64 / num_chars as f64;
        freq_delta += (expected - actual).abs();
    }

    (char_count, num_chars, freq_delta)
}
