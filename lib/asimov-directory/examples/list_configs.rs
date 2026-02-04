// cargo run --package asimov-directory --example list_configs

use asimov_directory::fs::StateDirectory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _configs_dir = StateDirectory::home()?.configs()?;
    // TODO
    Ok(())
}
