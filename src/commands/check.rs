use std::error::Error;

use std::path::Path;

use crate::conf::Role;

pub fn check_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = Path::new(matches.value_of("config").unwrap());
    let role = Role::open(path)?;
    println!("Parsed {} tasks.", role.tasks.len());
    println!("{} looks like a valid role.", path.display());

    Ok(())
}
