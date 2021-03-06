#[macro_use]
extern crate log;

pub mod machine {
    extern crate rand;
    use rand::prelude::*;
    use rand::Rng;

    pub const MAX_ROTORS: usize = 5;
    pub const MAX_REFLECTORS: usize = 3;
    pub const MAX_WIRES: usize = 26;
    pub const ALPHABET: [char; MAX_WIRES] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
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

    #[derive(Debug, Copy, Clone)]
    pub struct Rotor {
        name: &'static str,
        wiring: [usize; MAX_WIRES],
        inv_wiring: [usize; MAX_WIRES],
        turnover_post: usize,
    }

    impl Rotor {
        pub fn new(name: &'static str, encoding: &'static str, turnover_pos: char) -> Rotor {
            let wiring = gen_wiring(encoding);
            Rotor {
                name,
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

    #[derive(Debug, Copy, Clone)]
    pub struct Reflector {
        name: &'static str,
        wiring: [usize; MAX_WIRES],
    }

    impl Reflector {
        pub fn new(name: &'static str, encoding: &'static str) -> Reflector {
            Reflector {
                name,
                wiring: gen_wiring(encoding),
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct State {
        left_rotor: Rotor,
        center_rotor: Rotor,
        right_rotor: Rotor,
        setting: [usize; 3],
        offsets: [usize; 3],
        plug_board: [usize; MAX_WIRES],
        reflector: Reflector,
    }

    impl State {
        pub fn new(
            rotors: (Rotor, Rotor, Rotor),
            initial: [char; 3],
            plugs: [(char, char); 10],
            reflector: Reflector,
        ) -> State {
            let (left_rotor, center_rotor, right_rotor) = rotors;
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

        pub fn new_random() -> State {
            let (left_rotor, center_rotor, right_rotor) = random_rotors();
            let reflector = random_reflector();
            let initial = random_settings();
            let plugs = random_plugs();
            let initial_settings = [wire(initial[0]), wire(initial[1]), wire(initial[2])];

            State {
                left_rotor,
                center_rotor,
                right_rotor,
                setting: initial_settings,
                offsets: initial_settings,
                plug_board: gen_board(plugs),
                reflector,
            }
        }

        pub fn show(&self) -> String {
            format!(
                "State(Rotors={:>3}|{:>3}|{:>3} Settings={}|{}|{} Reflector={} Plugs={:?})",
                self.left_rotor.name,
                self.center_rotor.name,
                self.right_rotor.name,
                ALPHABET[self.setting[0]],
                ALPHABET[self.setting[1]],
                ALPHABET[self.setting[2]],
                self.reflector.name,
                self.plug_board,
            )
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

        pub fn encode(mut self, text: &String) -> String {
            let mut output = String::new();

            let left = self.left_rotor;
            let center = self.center_rotor;
            let right = self.right_rotor;
            let plug_board = self.plug_board;
            let reflector = self.reflector.wiring;

            for c in text.chars().map(|c| c.to_ascii_uppercase()) {
                if c.is_ascii() && c.is_alphabetic() {
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
                } else {
                    output.push(c);
                    debug!(
                        "IGNORED [{}]",
                        if c.is_control() {
                            String::from("CONTROL")
                        } else {
                            c.to_string()
                        }
                    );
                }
            }

            return output;
        }
    }

    pub fn all_rotors() -> [Rotor; 5] {
        [
            Rotor::new("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'R'),
            Rotor::new("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE", 'F'),
            Rotor::new("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO", 'W'),
            Rotor::new("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB", 'K'),
            Rotor::new("V", "VZBRGITYUPSDNHLXAWMJQOFECK", 'A'),
        ]
    }

    pub fn rotor_by_name(name: &str) -> Rotor {
        for r in all_rotors() {
            if r.name == name {
                return r;
            }
        }

        panic!("Unable to find rotor by name: {}", name);
    }

    fn random_rotors() -> (Rotor, Rotor, Rotor) {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (0..MAX_ROTORS).collect();
        nums.shuffle(&mut rng);

        let rotors = all_rotors();
        return (rotors[nums[0]], rotors[nums[1]], rotors[nums[2]]);
    }

    pub fn all_reflectors() -> [Reflector; 3] {
        [
            Reflector::new("A", "EJMZALYXVBWFCRQUONTSPIKHGD"),
            Reflector::new("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            Reflector::new("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        ]
    }

    fn random_reflector() -> Reflector {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..MAX_REFLECTORS);

        all_reflectors()[index]
    }

    pub fn reflector_by_name(name: &str) -> Reflector {
        for r in all_reflectors() {
            if r.name == name {
                return r;
            }
        }

        panic!("Unable to find reflector by name: {}", name);
    }

    fn random_settings() -> [char; 3] {
        let mut rng = rand::thread_rng();

        [
            ALPHABET[rng.gen_range(0..MAX_WIRES)],
            ALPHABET[rng.gen_range(0..MAX_WIRES)],
            ALPHABET[rng.gen_range(0..MAX_WIRES)],
        ]
    }

    fn random_plugs() -> [(char, char); 10] {
        NO_PLUGS
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
}

pub mod factory {
    use crate::machine::*;

    pub struct StateSet {
        pub count: usize,
        rotors: [Rotor; MAX_ROTORS],
        selected_rotors: (usize, usize, usize),
        rotor_indexes: (usize, usize, usize),
        rotor_options: (
            [usize; StateSet::MAX_LEFT_ROTOR],
            [usize; StateSet::MAX_CENTER_ROTOR],
            [usize; StateSet::MAX_RIGHT_ROTOR],
        ),
        reflectors: [Reflector; MAX_REFLECTORS],
        selected_reflector: usize,
        selected_settings: (usize, usize, usize),
    }

    impl StateSet {
        pub const MAX_LEFT_ROTOR: usize = MAX_ROTORS;
        pub const MAX_CENTER_ROTOR: usize = MAX_ROTORS - 1;
        pub const MAX_RIGHT_ROTOR: usize = MAX_ROTORS - 2;
        pub const MAX_STATES: usize = MAX_WIRES
            * MAX_WIRES
            * MAX_WIRES
            * MAX_REFLECTORS
            * StateSet::MAX_RIGHT_ROTOR
            * StateSet::MAX_CENTER_ROTOR
            * StateSet::MAX_LEFT_ROTOR;

        pub fn new() -> StateSet {
            StateSet {
                count: 0,
                rotors: all_rotors(),
                selected_rotors: (0, 1, 2),
                rotor_indexes: (0, 0, 0),
                rotor_options: ([0, 1, 2, 3, 4], [1, 2, 3, 4], [2, 3, 4]),
                reflectors: all_reflectors(),
                selected_reflector: 0,
                selected_settings: (0, 0, 0),
            }
        }

        fn shift(&mut self) {
            self.count += 1;

            let (mut left_set, mut center_set, mut right_set) = self.selected_settings;

            right_set += 1;
            if right_set == MAX_WIRES {
                center_set += 1;
                if center_set == MAX_WIRES {
                    left_set += 1;
                    if left_set == MAX_WIRES {
                        let mut reflector = self.selected_reflector;

                        reflector += 1;
                        if reflector == MAX_REFLECTORS {
                            let (mut left_index, mut center_index, mut right_index) =
                                self.rotor_indexes;
                            let (left_opts, mut center_opts, mut right_opts) = self.rotor_options;

                            right_index += 1;
                            if right_index == StateSet::MAX_RIGHT_ROTOR {
                                center_index += 1;

                                if center_index == StateSet::MAX_CENTER_ROTOR {
                                    left_index += 1;
                                    left_index %= StateSet::MAX_LEFT_ROTOR;

                                    // create new right rotor options
                                    let mut i = 0;
                                    for l in 0..StateSet::MAX_LEFT_ROTOR {
                                        if l != left_index {
                                            center_opts[i] = left_opts[l];
                                            i += 1;
                                        }
                                    }
                                }

                                center_index %= StateSet::MAX_CENTER_ROTOR;

                                // create new right rotor options
                                let mut i = 0;
                                for c in 0..StateSet::MAX_CENTER_ROTOR {
                                    if c != center_index {
                                        right_opts[i] = center_opts[c];
                                        i += 1;
                                    }
                                }
                            }

                            right_index %= StateSet::MAX_RIGHT_ROTOR;

                            let right_rotor = right_opts[right_index]; // Use next right rotor
                            let center_rotor = center_opts[center_index]; // Use next center rotor
                            let left_rotor = left_opts[left_index]; // Use next center rotor

                            self.rotor_options = (left_opts, center_opts, right_opts);
                            self.rotor_indexes = (left_index, center_index, right_index);
                            self.selected_rotors = (left_rotor, center_rotor, right_rotor);
                        }

                        self.selected_reflector = reflector % MAX_REFLECTORS;
                    }
                }
            }

            self.selected_settings = (
                left_set % MAX_WIRES,
                center_set % MAX_WIRES,
                right_set % MAX_WIRES,
            );
        }

        fn pick_rotors(&self) -> (Rotor, Rotor, Rotor) {
            let (left, center, right) = self.selected_rotors;

            (self.rotors[left], self.rotors[center], self.rotors[right])
        }

        fn pick_reflector(&self) -> Reflector {
            self.reflectors[self.selected_reflector]
        }

        fn pick_initial_settings(&self) -> [char; 3] {
            let (left_set, center_set, right_set) = self.selected_settings;

            [
                ALPHABET[left_set],
                ALPHABET[center_set],
                ALPHABET[right_set],
            ]
        }

        fn done(&self) -> bool {
            self.count >= StateSet::MAX_STATES
        }
    }

    impl Iterator for StateSet {
        type Item = State;

        fn next(&mut self) -> Option<Self::Item> {
            if self.done() {
                None
            } else {
                let next_state = Some(State::new(
                    self.pick_rotors(),
                    self.pick_initial_settings(),
                    NO_PLUGS,
                    self.pick_reflector(),
                ));

                self.shift();

                return next_state;
            }
        }
    }
}
