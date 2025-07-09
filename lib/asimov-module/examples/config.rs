// This is free and unencumbered software released into the public domain.

use asimov_module::models::ModuleManifest;
use std::error::Error;

const YAML: &str = r#"
name: brightdata
label: Bright Data
summary: Data import powered by the Bright Data web data platform.
links:
    - https://github.com/asimov-modules/asimov-brightdata-module
    - https://crates.io/crates/asimov-brightdata-module
    - https://pypi.org/project/asimov-brightdata-module
    - https://rubygems.org/gems/asimov-brightdata-module
    - https://npmjs.com/package/asimov-brightdata-module

config:
  variables:
    - name: API_KEY
      description: "API Key to authorize requests to Bright Data"

    - name: OTHER_VAR
      default_value: "foobar"
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest: ModuleManifest = serde_yml::from_str(YAML)?;

    let api_key = manifest
        .variable("API_KEY", None)
        .or_else(|_err| std::env::var("BRIGHTDATA_API_KEY"))
        .inspect_err(|_err| eprintln!("No API_KEY found. Either configure API_KEY with `asimov module config brightdata API_KEY <your_api-key>` or set the environment variable `BRIGHTDATA_API_KEY`"))
        .unwrap_or_default();
    println!("API_KEY: `{api_key}`");

    let other_var = manifest.variable("OTHER_VAR", None).unwrap_or_default();
    println!("OTHER_VAR: `{other_var}`");

    Ok(())
}
