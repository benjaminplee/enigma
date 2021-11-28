// use enigma::*;
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

    info!("Using input file: {}", input_file);

    pretty_env_logger::init();

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

    // println!("Rotors: {:#?}", rotors);
    // println!("Reflectors: {:#?}", reflectors);

    let mut machine = enigma::State::new(
        &rotors[0],
        &rotors[1],
        &rotors[2],
        ['A', 'A', 'A'],
        enigma::NO_PLUGS,
        &reflectors[0],
    );

    // let text = String::from("HELLO WORLD! THE QUICK BROWN FOX JUMPED OVER THE LAZY DOG.");

    let text = fs::read_to_string(input_file).expect("Something went wrong reading the input file");

    debug!("Starting State: {:?}", machine);

    let output = machine.encode(&text);

    info!("Encoded: {} -> {}", text, output);

    println!("{}", output);
}
