// This is free and unencumbered software released into the public domain.

use asimov_module::models::Manifest;
use std::{error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let _n = std::io::stdin().read_to_string(&mut input)?;
    let manifest: Manifest = serde_yml::from_str(&input)?;
    println!("Debug print:\n\n{manifest:?}");
    let output = serde_yml::to_string(&manifest)?;
    println!("\nRe-encoded:\n\n{output}");
    Ok(())
}
