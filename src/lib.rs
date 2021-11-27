#[macro_use]
extern crate log;

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
            self.offsets[1] = self.offsets[1] + 1;
            self.offsets[0] = self.offsets[0] + 1;
        }

        if self.offsets[2] == self.rotors[2].turnover_post {
            self.offsets[1] = self.offsets[1] + 1;
        }

        self.offsets[2] = self.offsets[2] + 1;
    }

    pub fn encrypt(_input: char) -> char {
        return 'A';
    }

    pub fn encode(&'a mut self, text: &String) -> String {
        let mut output = String::new();

        let left = self.rotors[0].wiring;
        let center = self.rotors[1].wiring;
        let right = self.rotors[2].wiring;
        let plug_board = self.plug_board;
        let reflector = self.reflector.wiring;

        info!("Starting encoding of: {}", text);
        info!("R-Rotor: {:#?}", right);

        for c in text
            .chars()
            .filter(|c| c.is_ascii() && c.is_alphabetic())
            .map(|c| c.to_ascii_uppercase())
        {
            let mut shift = 0;

            // Input
            let input1 = ((c as u8) - b'A') as usize;
            debug!("Input = {}({})", input1, c);

            // (1) Shift Rotors
            self.increment();

            // (2) Plug Board
            let input2 = plug_board[input1];
            debug!("Plug = {} -> {}", input1, input2);

            // (3) First Rotor
            shift = (input2 + self.offsets[2]) % 26;
            let input3 = right[shift];
            debug!("R-Rotor = {} -> {} -> {}", input2, shift, input3);

            // (4) Second Rotor
            let input4 = center[(input3 + self.offsets[1]) % 26];
            debug!("C-Rotor = {} -> {}", input3, input4);

            // (5) Third Rotor
            let input5 = left[(input4 + self.offsets[0]) % 26];
            debug!("L-Rotor = {} -> {}", input4, input5);

            // (6) Reflector
            let input6 = reflector[input5];
            debug!("Reflector = {} -> {}", input5, input6);

            // (7) Third Rotor Inverse
            let input7 = left[(input6 + self.offsets[0]) % 26];
            debug!("L-Rotor = {} -> {}", input6, input7);

            // (8) Second Rotor Inverse
            let input8 = center[(input7 + self.offsets[1]) % 26];
            debug!("C-Rotor = {} -> {}", input7, input8);

            // (9) First Rotor Inverse
            let input9 = right[(input8 + self.offsets[2]) % 26];
            debug!("R-Rotor = {} -> {}", input8, input9);

            // (10) Plug Board
            let input10 = plug_board[input9];
            debug!("Plug = {} -> {}", input9, input10);

            // Output
            let cout = (input10 as u8 + b'A') as char;
            debug!("Output = {}({})", input10, cout);
            output.push(cout);

            info!("ENCRYPTED {} -> {}", c, cout);
        }

        return output;
    }
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
