use asimov_huggingface::ensure_file;
use std::env;

/// Usage:
/// cargo run -p asimov-huggingface --example download -- <repo> <filename>
/// Example:
/// cargo run -p asimov-huggingface --example download -- facebook/dinov2-base pytorch_model.bin
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let repo = args.next().expect("usage: download <repo> <filename>");
    let file = args.next().expect("usage: download <repo> <filename>");

    let path = ensure_file(&repo, &file)?;
    println!("\nSaved to: {}", path.display());
    Ok(())
}
