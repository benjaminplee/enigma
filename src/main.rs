use enigma::*;

fn main() {
    let M3_ROTORS = [
        Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'A'),
        Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", 'A'),
        Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", 'A'),
        Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", 'A'),
        Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", 'A'),
    ];

    println!("Output: {:?}", M3_ROTORS[0]);
}
