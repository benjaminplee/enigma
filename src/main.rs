use enigma::*;

fn main() {
    let rotors = [
        Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'A'),
        Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", 'A'),
        Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", 'A'),
        Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", 'A'),
        Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", 'A'),
    ];

    println!("Output: {:#?}", rotors[0]);
}
