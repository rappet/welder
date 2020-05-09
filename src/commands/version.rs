pub fn version_command(_matches: &clap::ArgMatches) {
    println!("Running {} version {}", crate::NAME, crate::VERSION);
}
