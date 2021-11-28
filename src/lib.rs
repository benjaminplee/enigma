#[macro_use]
extern crate log;

const MAX_WIRES: usize = 26;
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

#[derive(Debug)]
pub struct Rotor {
    name: String,
    wiring: [usize; MAX_WIRES],
    inv_wiring: [usize; MAX_WIRES],
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

    fn push(&self, input: usize, offset: usize) -> usize {
        let shift = (MAX_WIRES + input + offset) % MAX_WIRES;
        let output = (MAX_WIRES + self.wiring[shift] - offset) % MAX_WIRES;
        trace!("PUSH {} :: {} -> {} -> {}", self.name, input, shift, output);
        return output;
    }

    fn pull(&self, input: usize, offset: usize) -> usize {
        let shift = (MAX_WIRES + input + offset) % MAX_WIRES;
        let output = (MAX_WIRES + self.inv_wiring[shift] - offset) % MAX_WIRES;
        trace!("PULL {} :: {} -> {} -> {}", self.name, input, shift, output);
        return output;
    }
}

#[derive(Debug)]
pub struct Reflector {
    name: String,
    wiring: [usize; MAX_WIRES],
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
    left_rotor: &'a Rotor,
    center_rotor: &'a Rotor,
    right_rotor: &'a Rotor,
    setting: [usize; 3],
    offsets: [usize; 3],
    plug_board: [usize; MAX_WIRES],
    reflector: &'a Reflector,
}

impl<'a> State<'a> {
    pub fn new(
        left_rotor: &'a Rotor,
        center_rotor: &'a Rotor,
        right_rotor: &'a Rotor,
        initial: [char; 3],
        plugs: [(char, char); 10],
        reflector: &'a Reflector,
    ) -> State<'a> {
        let initial_settings = [wire(initial[0]), wire(initial[1]), wire(initial[2])];
        State {
            left_rotor,
            center_rotor,
            right_rotor,
            setting: initial_settings,
            offsets: [wire(initial[0]), wire(initial[1]), wire(initial[2])],
            plug_board: gen_board(plugs),
            reflector,
        }
    }

    fn increment(&mut self) {
        if self.offsets[1] == self.right_rotor.turnover_post {
            self.offsets[1] = (self.offsets[1] + 1) % MAX_WIRES;
            self.offsets[0] = (self.offsets[0] + 1) % MAX_WIRES;
        }

        if self.offsets[2] == self.center_rotor.turnover_post {
            self.offsets[1] = (self.offsets[1] + 1) % MAX_WIRES;
        }

        self.offsets[2] = (self.offsets[2] + 1) % MAX_WIRES;
    }

    pub fn encrypt(_input: char) -> char {
        return 'A';
    }

    pub fn encode(&'a mut self, text: &String) -> String {
        let mut output = String::new();

        let left = self.left_rotor;
        let center = self.center_rotor;
        let right = self.right_rotor;
        let plug_board = self.plug_board;
        let reflector = self.reflector.wiring;

        for c in text
            .chars()
            .filter(|c| c.is_ascii() && c.is_alphabetic())
            .map(|c| c.to_ascii_uppercase())
        {
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
            let input3 = right.push(input2, right_offset);

            // (4) Second Rotor
            let input4 = center.push(input3, center_offset);

            // (5) Third Rotor
            let input5 = left.push(input4, left_offset);

            // (6) Reflector
            let input6 = reflector[input5];
            trace!("Reflector = {} -> {}", input5, input6);

            // (7) Third Rotor Inverse
            let input7 = left.pull(input6, left_offset);

            // (8) Second Rotor Inverse
            let input8 = center.pull(input7, center_offset);

            // (9) First Rotor Inverse
            let input9 = right.pull(input8, right_offset);

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
                unwire(left_offset % MAX_WIRES),
                unwire(center_offset % MAX_WIRES),
                unwire(right_offset % MAX_WIRES),
                left_offset,
                center_offset,
                right_offset
            );
        }

        return output;
    }
}

fn gen_board(plugs: [(char, char); 10]) -> [usize; MAX_WIRES] {
    let mut board: [usize; MAX_WIRES] = [0; MAX_WIRES];

    for i in 0..MAX_WIRES {
        board[i] = i;
    }

    for (p1, p2) in plugs {
        let w1 = wire(p1);
        let w2 = wire(p2);

        board[w1] = w2;
        board[w2] = w1;
    }

    return board;
}

fn gen_wiring(encoding: &str) -> [usize; MAX_WIRES] {
    let mut wiring: [usize; MAX_WIRES] = [0; MAX_WIRES];
    let bytes = encoding.as_bytes();

    for i in 0..MAX_WIRES {
        wiring[i] = wire(bytes[i] as char)
    }

    return wiring;
}

fn inv(wiring: [usize; MAX_WIRES]) -> [usize; MAX_WIRES] {
    let mut iwiring: [usize; MAX_WIRES] = [0; MAX_WIRES];

    for i in 0..MAX_WIRES {
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
