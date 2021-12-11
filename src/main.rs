extern crate env_logger;
use env_logger::Env;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::App;

use std::fs;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();

    let level = match matches.occurrences_of("verbose") {
        0 => "OFF",
        1 => "INFO",
        2 => "DEBUG",
        3 | _ => "TRACE",
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(level)).init();

    info!("Using input file: {}", input_file);

    let rotors = [
        enigma::Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'R'),
        enigma::Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", 'F'),
        enigma::Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", 'W'),
        enigma::Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", 'K'),
        enigma::Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", 'A'),
    ];

    let reflectors = [
        enigma::Reflector::new("A", "EJMZALYXVBWFCRQUONTSPIKHGD"),
        enigma::Reflector::new("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        enigma::Reflector::new("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
    ];

    let mut machine = enigma::State::new(
        &rotors[0],
        &rotors[1],
        &rotors[2],
        ['A', 'A', 'A'],
        enigma::NO_PLUGS,
        &reflectors[0],
    );

    let text = fs::read_to_string(input_file).expect("Something went wrong reading the input file");

    debug!("Starting State: {:?}", machine);

    let output = machine.encode(&text);

    println!("{}", output);
}
