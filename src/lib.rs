#[derive(Debug)]
pub struct Rotor {
    name: String,
    wiring: [usize; 26],
    turnover_post: usize,
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

fn gen_wiring(encoding: &str) -> [usize; 26] {
    let mut wiring: [usize; 26] = [0; 26];
    let bytes = encoding.as_bytes();

    for i in 0..26 {
        wiring[i] = wire(bytes[i] as char)
    }

    return wiring;
}

fn wire(c: char) -> usize {
    return ((c as u8) - b'A') as usize;
}

#[derive(Debug)]
pub struct Reflector {
    name: String,
    wiring: [usize; 26],
}

impl Reflector {
    pub fn new(name: &str, encoding: &str) -> Reflector {
        Reflector {
            name: String::from(name),
            wiring: gen_wiring(encoding),
        }
    }
}

#[derive(Debug)]
pub struct State {
    rotors: [Rotor; 3],
    setting: [usize; 3],
    offsets: [usize; 3],
    plug_board: [usize; 26],
    reflector: Reflector,
}

impl State {
    pub fn new(
        r1: Rotor,
        r2: Rotor,
        r3: Rotor,
        initial: [char; 3],
        plugs: [(char, char); 10],
        reflector: Reflector,
    ) -> State {
        let initial_settings = [wire(initial[0]), wire(initial[1]), wire(initial[2])];
        State {
            rotors: [r1, r2, r3],
            setting: initial_settings,
            offsets: [wire(initial[0]), wire(initial[1]), wire(initial[2])],
            plug_board: gen_board(plugs),
            reflector,
        }
    }

    fn increment(&self) {}
}

fn gen_board(plugs: [(char, char); 10]) -> [usize; 26] {
    let mut board = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 181, 19, 20, 21, 22, 23, 24,
        25,
    ];

    for (p1, p2) in plugs {
        let w1 = wire(p1);
        let w2 = wire(p2);

        board[w1] = w2;
        board[w2] = w1;
    }

    return board;
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
