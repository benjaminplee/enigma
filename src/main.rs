// https://en.wikipedia.org/wiki/Enigma_rotor_details
const ROTORS: [[u8; 26]; 5] = [
    [
        4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2,
        9,
    ],
    [
        0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14,
        4,
    ],
    [
        1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16,
        14,
    ],
    [
        4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22,
        1,
    ],
    [
        21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2,
        10,
    ],
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
