use asimov_huggingface::ensure_file;
use std::env;

/// Example: download a file from Hugging Face with a unified progress bar.
///
/// Usage:
/// ```bash
/// cargo run -p asimov-huggingface --example download -- <repo> <filename>
/// # Example:
/// # cargo run -p asimov-huggingface --example download -- facebook/dinov2-base pytorch_model.bin
/// ```
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: download <repo> <filename>");
        std::process::exit(1);
    }

    let repo = &args[1];
    let file = &args[2];

    let path = ensure_file(repo, file)?;
    println!("\n✅ Saved to: {}", path.display());

    Ok(())
}
