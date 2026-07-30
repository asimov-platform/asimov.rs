// cargo run --package asimov-directory --example show_config

use asimov_directory::fs::StateDirectory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configs = StateDirectory::home()?.configs()?;
    let profile = configs.default_profile()?;
    println!("{:?}", profile); // TODO
    Ok(())
}
