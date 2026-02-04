use asimov_directory::{ModuleNameIterator as _, fs::StateDirectory};
use core::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let modules_dir = StateDirectory::home()?.modules()?;
    let mut module_names = modules_dir.iter_installed().await?;
    while let Some(module_name) = module_names.next().await {
        println!("{}", module_name);
    }
    Ok(())
}
