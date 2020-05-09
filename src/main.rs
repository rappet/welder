#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate serde;
extern crate toml;
extern crate serde_json;

use clap::{App, load_yaml};

use std::env;

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

mod module;
mod conf;
mod commands;

fn init_envlogger(verbosity: u64) {
    // set RUST_LOG if --verbose flag is set or
    // overwrite RUST_LOG if not set to show warning log
    if verbosity > 0 || env::var_os("RUST_LOG").is_none() {
        let level = match verbosity {
            0 => "warn",
            1 => "info",
            _ => "debug"
        };
        env::set_var("RUST_LOG", format!("welder={}", level));
    }
    env_logger::init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::from(load_yaml!("cli.yml"))
        .name(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .get_matches();

    let verbosity = matches.occurrences_of("verbose");

    init_envlogger(verbosity);

    match matches.subcommand() {
        ("version", Some(matches)) => commands::version::version_command(matches),
        ("check", Some(matches)) => commands::check::check_command(matches)?,
        _ => unimplemented!()
    }

    Ok(())
}
