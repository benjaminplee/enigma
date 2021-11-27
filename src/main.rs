// use enigma::*;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
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

    let text = String::from("HELLO WORLD!");

    trace!("Starting State: {:#?}", machine);

    let output = machine.encode(&text);

    info!("Encoded: {} -> {}", text, output);

    println!("{}", output);
}
