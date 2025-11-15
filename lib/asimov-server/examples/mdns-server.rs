// This is free and unencumbered software released into the public domain.

use asimov_server::mdns;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    mdns::register()?;

    std::thread::sleep(std::time::Duration::MAX);
    Ok(())
}
