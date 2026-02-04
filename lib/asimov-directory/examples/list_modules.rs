// cargo run --package asimov-directory --example list_modules

use asimov_directory::{ModuleNameIterator, fs::StateDirectory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modules_dir = StateDirectory::home()?.modules()?;
    let mut module_names = modules_dir.iter_installed().await?;
    while let Some(module_name) = module_names.next().await {
        println!("{}", module_name);
    }
    Ok(())
}
