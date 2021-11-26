#[derive(Debug)]
pub struct Rotor {
    name: String,
    wiring: [u8; 26],
    turnover_post: u8,
}

impl Rotor {
    pub fn new(name: &str, encoding: &str, turnover_pos: char) -> Rotor {
        Rotor {
            name: String::from(name),
            wiring: gen_wiring(encoding),
            turnover_post: wire(turnover_pos),
        }
    }
}

fn gen_wiring(encoding: &str) -> [u8; 26] {
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ]
}

fn wire(c: char) -> u8 {
    1
}

#[derive(Debug)]
pub struct State {
    rotors: [Rotor; 3],
    setting: [u8; 3],
    offsets: [u8; 3],
    plug_board: [u8; 26],
}

impl State {
    pub fn new(
        r1: Rotor,
        r2: Rotor,
        r3: Rotor,
        initial: [char; 3],
        plugs: [(char, char); 10],
    ) -> State {
        State {
            rotors: [r1, r2, r3],
            setting: [wire(initial[0]), wire(initial[1]), wire(initial[2])],
            offsets: [0, 0, 0],
            plug_board: gen_board(plugs),
        }
    }

    fn increment(&self) {}
}

fn gen_board(plugs: [(char, char); 10]) -> [u8; 26] {
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ]
}

pub fn machine(mut state: State, text: String) -> String {
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
