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

use enigma::machine::*;
use enigma::factory::*;

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
        ("rand-io", Some(_)) => {
            command_rand_io();
        }
        ("rand-dir", Some(sub_m)) => {
            command_rand_dir(
                sub_m.value_of("source").unwrap(),
                sub_m.value_of("destination").unwrap(),
            );
        }
        ("stats-io", Some(_)) => {
            stats_io();
        }
        ("search-io", Some(_)) => {
            command_search_io();
        }
        ("encode-io", Some(sub_m)) => {
            command_encode_io(
                sub_m.value_of("left_rotor").unwrap(),
                sub_m.value_of("center_rotor").unwrap(),
                sub_m.value_of("right_rotor").unwrap(),
                sub_m.value_of("left_rotor_start").unwrap(),
                sub_m.value_of("center_rotor_start").unwrap(),
                sub_m.value_of("right_rotor_start").unwrap(),
                sub_m.value_of("reflector").unwrap(),
            );
        }
        _ => unreachable!("Unknown subcommand"),
    }
}

fn command_encode_io(
    left_rotor: &str,
    center_rotor: &str,
    right_rotor: &str,
    left_rotor_start: &str,
    center_rotor_start: &str,
    right_rotor_start: &str,
    reflector: &str,
) {
    info!("Running ENCODE-IO subcommand");
    debug!(
        " Config = Rotors: {} {} {}  Start: {} {} {} Reflector: {}",
        left_rotor,
        center_rotor,
        right_rotor,
        left_rotor_start,
        center_rotor_start,
        right_rotor_start,
        reflector
    );

    encode_io(State::new(
        (
            rotor_by_name(left_rotor),
            rotor_by_name(center_rotor),
            rotor_by_name(right_rotor),
        ),
        [
            left_rotor_start.chars().next().unwrap(),
            center_rotor_start.chars().next().unwrap(),
            right_rotor_start.chars().next().unwrap(),
        ],
        NO_PLUGS,
        reflector_by_name(reflector),
    ));
}

fn command_rand_io() {
    info!("Running RAND-IO subcommand");

    encode_io(State::new_random());
}

fn encode_io(machine: State) {
    info!("Encoding with {}", machine.show());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.expect("Error reading from STDIN: {}");
        trace!("Read from STDIN: {}", text);
        let output = machine.encode(&text);
        println!("{}", output);
    }
}

fn command_search_io() {
    info!("Running SEARCH-IO subcommand");

    let states = StateSet::new();
    let mut best_state_by_freq = State::new_random();
    let mut best_freq = 2600.0;

    let mut count = 0;

    debug!(
        "Running through {} states for first pass",
        StateSet::MAX_STATES
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

    println!(
        "Best guess is below using state: {}",
        best_state_by_freq.show()
    );
    println!("------------------------------------------------------");
    println!("{}", best_state_by_freq.encode(&buffer));
    println!("------------------------------------------------------");
}

fn command_rand_dir(source: &str, dest: &str) {
    info!(
        "Running RAND-DIR subcommand for source: {} and dest: {}",
        source, dest
    );

    let machine = State::new_random();
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

            debug!(
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
    for c in ALPHABET {
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
