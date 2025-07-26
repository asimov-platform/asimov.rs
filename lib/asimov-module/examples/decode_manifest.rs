// This is free and unencumbered software released into the public domain.

use asimov_module::ModuleManifest;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = serde_yml::from_reader::<_, ModuleManifest>(std::io::stdin())?;
    print!("Debug print:\n\n{manifest:?}\n\n");
    print!("Re-encoded:\n\n");
    serde_yml::to_writer(std::io::stdout(), &manifest)?;
    Ok(())
}
