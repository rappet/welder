#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn init_envlogger() {
    // overwrite RUST_LOG if not set to show informational log
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "welder=info");
    }
    env_logger::init();
}

fn main() {
    init_envlogger();

    info!("Running {} version {}", NAME, VERSION);
}
