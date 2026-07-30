// cargo run --package asimov-directory --example print_paths

use asimov_directory::fs::StateDirectory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asimov_dir = StateDirectory::home()?;
    println!("{}", asimov_dir); // "$HOME/.asimov" on Unix
    println!("{}", asimov_dir.configs()?); // "$HOME/.asimov/configs"
    println!("{}", asimov_dir.modules()?); // "$HOME/.asimov/modules"
    println!("{}", asimov_dir.programs()?); // "$HOME/.asimov/libexec"
    Ok(())
}
