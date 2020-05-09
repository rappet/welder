#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate toml;
extern crate serde_json;

use std::env;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod module;
mod conf;

fn init_envlogger() {
    // overwrite RUST_LOG if not set to show informational log
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "welder=info");
    }
    env_logger::init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_envlogger();

    info!("Running {} version {}", NAME, VERSION);

    let s = r#"
[vars]
name = 'Waldi Welder'

[vars.network]
ip = '192.168.1.102/24'
gateway = '192.168.1.1'

[vars.network.dns]
# Quad9 public DNS resolver
primary4 = '9.9.9.9'
secondary4 = '149.112.112.112'
primary6 = '2620:fe::fe'
secondary6 = '2620:fe::9'

[[task]]
name = 'Load foo'
module = 'var'

value = 'Hello, {{ name }}!'

requires = 'name'
yields = 'message'

[[task]]
name = 'Debug message'
module = 'debug'

value = '{{ message }}'

requires = 'message'
"#;

    let role: conf::Role = toml::from_str(s)?;

    info!("Parsed: {}", serde_json::to_string_pretty(&role)?);

    Ok(())
}
