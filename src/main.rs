// use enigma::*;

fn main() {
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

    println!("Starting State: {:#?}", machine);

    println!("Encoded: {} -> {}", text, machine.encode(&text));
}
