// This is free and unencumbered software released into the public domain.

use asimov_module::models::ModuleManifest;
use std::error::Error;

const YAML: &str = r#"
name: example
label: Example
summary: Example module
links:
    - https://github.com/asimov-platform/asimov.rs/tree/master/lib/asimov-module/examples/config.rs

config:
  variables:
    # name is the only mandatory field

    - name: api_key
      description: "api key to authorize requests"

    - name: other_var
      default_value: "foobar"

    - name: var_from_env
      environment: VAR_FROM_ENV
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest: ModuleManifest = serde_yml::from_str(YAML)?;

    let api_key = manifest.variable("api_key", None).unwrap_or_default();
    if api_key.is_empty() {
        println!("api_key: `{api_key}`");
        println!("(consider `mkdir -p ~/.asimov/configs/default/example/ && echo -n \"<api-key-value>\" >> ~/.asimov/configs/default/example/api_key` or for a non-example module `asimov module config <example> api_key <api-key-value>`)");
    } else {
        println!("api_key: `{api_key}`");
    }

    let other_var = manifest.variable("other_var", None).unwrap_or_default();
    println!("\nother_var: `{other_var}`");

    let env_var = manifest.variable("var_from_env", None).unwrap_or_default();
    println!("\nvar_from_env: `{env_var}`");

    println!("setting env var `VAR_FROM_ENV` manually...");
    unsafe { std::env::set_var("VAR_FROM_ENV", "hello!") }
    let env_var = manifest.variable("var_from_env", None).unwrap_or_default();
    println!("var_from_env: `{env_var}`");

    // alternative way:
    // let vars = manifest.read_variables(None)?;
    // let api_key = vars.get("api_key");
    // println!("api_key: `{api_key:?}`");

    Ok(())
}
