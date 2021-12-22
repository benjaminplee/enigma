extern crate env_logger;
use env_logger::Env;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

mod commands;
use crate::commands::*;

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
        ("rand-io", Some(_)) => {
            command_rand_io();
        }
        ("rand-dir", Some(sub_m)) => {
            command_rand_dir(
                sub_m.value_of("source").unwrap(),
                sub_m.value_of("destination").unwrap(),
            );
        }
        ("stats-io", Some(_)) => {
            command_stats_io();
        }
        ("search-io", Some(_)) => {
            command_search_io();
        }
        ("encode-io", Some(sub_m)) => {
            command_encode_io(
                sub_m.value_of("left_rotor").unwrap(),
                sub_m.value_of("center_rotor").unwrap(),
                sub_m.value_of("right_rotor").unwrap(),
                sub_m.value_of("left_rotor_start").unwrap(),
                sub_m.value_of("center_rotor_start").unwrap(),
                sub_m.value_of("right_rotor_start").unwrap(),
                sub_m.value_of("reflector").unwrap(),
            );
        }
        _ => unreachable!("Unknown subcommand"),
    }
}
