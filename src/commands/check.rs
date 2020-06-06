use std::error::Error;

use std::path::Path;

use crate::conf::Role;

pub fn check_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = Path::new(matches.value_of("config").unwrap());
    let role = Role::open(path)?;

    for (module, tasks) in &role.tasks {
        for (name, task) in tasks {
            info!(
                "Found task: {}.{} {}",
                module, name,
                task.description.clone().unwrap_or("<with no description>".into())
            );
        }
    }

    info!("Role content: {}", serde_json::to_string_pretty(&role)?);
    println!("Parsed {} tasks.", role.tasks.len());
    println!("{} looks like a valid role.", path.display());

    Ok(())
}
