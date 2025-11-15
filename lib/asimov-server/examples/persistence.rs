use std::io::Write;

use asimov_server::persistence;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = persistence::get();
    println!("{state:?}");

    print!("Select new provider: ");
    std::io::stdout().flush()?;

    let mut provider = String::new();
    std::io::stdin().read_line(&mut provider)?;
    provider = provider.trim().to_string();

    persistence::set(|x| {
        x.provider = provider;
    })?;

    Ok(())
}
