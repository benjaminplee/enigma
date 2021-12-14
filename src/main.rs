extern crate env_logger;
use env_logger::Env;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

use std::fs;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .get_matches();

    let level = match matches.occurrences_of("verbose") {
        0 => "OFF",
        1 => "INFO",
        2 => "DEBUG",
        3 | _ => "TRACE",
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(level)).init();

    match matches.subcommand() {
        ("io", Some(_)) => {
            command_io();
        }
        ("dir", Some(sub_m)) => {
            command_dir(
                sub_m.value_of("source").unwrap(),
                sub_m.value_of("destination").unwrap(),
            );
        }
        _ => unreachable!("Unknown subcommand"),
    }
}

fn command_io() {
    info!("Running IO subcommand");
}

fn command_dir(source: &str, dest: &str) {
    info!(
        "Running DIR subcommand for source: {} and dest: {}",
        source, dest
    );

    // let rotors = enigma::State::all_rotors();
    // let reflectors = enigma::State::all_reflectors();
    //
    // let mut machine = enigma::State::new(
    //     &rotors[0],
    //     &rotors[1],
    //     &rotors[2],
    //     ['A', 'A', 'A'],
    //     enigma::NO_PLUGS,
    //     &reflectors[0],
    // );
    //
    // let text = fs::read_to_string(source).expect("Something went wrong reading the input file");
    //
    // debug!("Starting State: {:?}", machine);
    //
    // let output = machine.encode(&text);
    //
    // println!("{}", output);
}
