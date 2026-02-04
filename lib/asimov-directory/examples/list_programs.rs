// cargo run --package asimov-directory --example list_programs

use asimov_directory::fs::StateDirectory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _programs = StateDirectory::home()?.programs()?;
    // TODO
    Ok(())
}
