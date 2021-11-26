use enigma::*;

fn main() {
    let rotors = [
        Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'R'),
        Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", 'F'),
        Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", 'W'),
        Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", 'K'),
        Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", 'A'),
    ];

    let reflectors = [
        Reflector::new("A", "EJMZALYXVBWFCRQUONTSPIKHGD"),
        Reflector::new("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        Reflector::new("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
    ];

    println!("Rotors: {:#?}", rotors);
    println!("Reflectors: {:#?}", reflectors);
}
