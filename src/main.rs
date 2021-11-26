// https://en.wikipedia.org/wiki/Enigma_rotor_details
const M3_ROTORS: [&str; 5] = [
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
    "AJDKSIRUXBLHWTMCQGZNPYFVOE",
    "BDFHJLCPRTXVZNYEIWGAKMUSQO",
    "ESOVPZJAYQUIRHXLNFTGKDCMWB",
    "VZBRGITYUPSDNHLXAWMJQOFECK",
];

struct State {
    rotors: [u8; 3],
    initial: [u8; 3],
    offset: [u8; 3],
    plugs: [u8; 26],
}

impl State {
    fn new(rotors: [u8; 3], initial: [u8; 3]) -> State {
        State {
            rotors,
            initial,
            offset: [0, 0, 0],
            plugs: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                33, 24, 25,
            ],
        }
    }
}

fn enigma(mut state: State, text: String) -> String {
    let mut output = String::new();

    for c in text
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
    {
        // Input
        let input = (c as u8) - b'A';

        // (1) Shift Rotors

        // (2) Plug Board

        // (3) First Rotor

        // (4) Second Rotor

        // (5) Third Rotor

        // (6) Reflector

        // (7) Third Rotor Inverse

        // (8) Second Rotor Inverse

        // (9) First Rotor Inverse

        // (10) Plug Board

        // Output
        output.push((input + b'A') as char);
    }

    output
}

fn main() {
    let state = State::new([0, 3, 4], [1, 1, 1]);
    let input = String::from("HELLO! I AM HERE.");
    let output = enigma(state, input);

    println!("Output: {:#?}", output);
}
