#[macro_use]
extern crate log;

const MAX_WIRES: usize = 26;

#[derive(Debug)]
pub struct Rotor {
    name: String,
    wiring: [usize; 26],
    inv_wiring: [usize; 26],
    turnover_post: usize,
}

impl Rotor {
    pub fn new(name: &str, encoding: &str, turnover_pos: char) -> Rotor {
        let wiring = gen_wiring(encoding);
        Rotor {
            name: String::from(name),
            wiring,
            inv_wiring: inv(wiring),
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

fn inv(wiring: [usize; 26]) -> [usize; 26] {
    let mut iwiring: [usize; 26] = [9; 26];

    for i in 0..26 {
        iwiring[wiring[i]] = i;
    }

    return iwiring;
}

fn wire(c: char) -> usize {
    return ((c as u8) - b'A') as usize;
}

fn unwire(i: usize) -> char {
    return (i as u8 + b'A') as char;
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
pub struct State<'a> {
    rotors: [&'a Rotor; 3],
    setting: [usize; 3],
    offsets: [usize; 3],
    plug_board: [usize; 26],
    reflector: &'a Reflector,
}

pub const NO_PLUGS: [(char, char); 10] = [
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
    ('A', 'A'),
];

impl<'a> State<'a> {
    pub fn new(
        r1: &'a Rotor,
        r2: &'a Rotor,
        r3: &'a Rotor,
        initial: [char; 3],
        plugs: [(char, char); 10],
        reflector: &'a Reflector,
    ) -> State<'a> {
        let initial_settings = [wire(initial[0]), wire(initial[1]), wire(initial[2])];
        State {
            rotors: [r1, r2, r3],
            setting: initial_settings,
            offsets: [wire(initial[0]), wire(initial[1]), wire(initial[2])],
            plug_board: gen_board(plugs),
            reflector,
        }
    }

    fn increment(&mut self) {
        if self.offsets[1] == self.rotors[1].turnover_post {
            self.offsets[1] = (self.offsets[1] + 1) % MAX_WIRES;
            self.offsets[0] = (self.offsets[0] + 1) % MAX_WIRES;
        }

        if self.offsets[2] == self.rotors[2].turnover_post {
            self.offsets[1] = (self.offsets[1] + 1) % MAX_WIRES;
        }

        self.offsets[2] = (self.offsets[2] + 1) % MAX_WIRES;
    }

    pub fn encrypt(_input: char) -> char {
        return 'A';
    }

    pub fn encode(&'a mut self, text: &String) -> String {
        let mut output = String::new();

        let left = self.rotors[0];
        let center = self.rotors[1];
        let right = self.rotors[2];
        let plug_board = self.plug_board;
        let reflector = self.reflector.wiring;

        for c in text
            .chars()
            .filter(|c| c.is_ascii() && c.is_alphabetic())
            .map(|c| c.to_ascii_uppercase())
        {
            let mut shift: usize;

            // Input
            let input1 = ((c as u8) - b'A') as usize;

            // (1) Shift Rotors
            self.increment();
            let left_offset = self.offsets[0];
            let center_offset = self.offsets[1];
            let right_offset = self.offsets[2];

            // (2) Plug Board
            let input2 = plug_board[input1];
            trace!("Plug = {} -> {}", input1, input2);

            // (3) First Rotor
            shift = (input2 + right_offset) % MAX_WIRES;
            let input3 = right.wiring[shift];
            trace!("R-Rotor = {} -> {} -> {}", input2, shift, input3);

            // (4) Second Rotor
            shift = (input3 + center_offset) % MAX_WIRES;
            let input4 = center.wiring[shift];
            trace!("C-Rotor = {} -> {} -> {}", input3, shift, input4);

            // (5) Third Rotor
            shift = (input4 + left_offset) % MAX_WIRES;
            let input5 = left.wiring[shift];
            trace!("L-Rotor = {} -> {} -> {}", input4, shift, input5);

            // (6) Reflector
            let input6 = reflector[input5];
            trace!("Reflector = {} -> {}", input5, input6);

            // (7) Third Rotor Inverse
            shift = (input6 + left_offset) % MAX_WIRES;
            let input7 = left.inv_wiring[shift];
            trace!("L-Rotor = {} -> {} -> {}", input6, shift, input7);

            // (8) Second Rotor Inverse
            shift = (input7 + center_offset) % MAX_WIRES;
            let input8 = center.inv_wiring[shift];
            trace!("C-Rotor = {} -> {} -> {}", input7, shift, input8);

            // (9) First Rotor Inverse
            shift = (input8 + right_offset) % MAX_WIRES;
            let input9 = right.inv_wiring[shift];
            trace!("R-Rotor = {} -> {} -> {}", input8, shift, input9);

            // (10) Plug Board
            let input10 = plug_board[input9];
            trace!("Plug = {} -> {}", input9, input10);

            // Output
            let cout = unwire(input10);
            output.push(cout);

            debug!(
                "ENCRYPTED {} -> {} :: {} {} {} :: {} {} {}",
                c,
                cout,
                unwire(left_offset % 26),
                unwire(center_offset % 26),
                unwire(right_offset % 26),
                left_offset,
                center_offset,
                right_offset
            );
        }

        return output;
    }
}

fn gen_board(plugs: [(char, char); 10]) -> [usize; 26] {
    let mut board = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
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
