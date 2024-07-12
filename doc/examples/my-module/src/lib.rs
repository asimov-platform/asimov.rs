// This is free and unencumbered software released into the public domain.

pub use asimov_sdk::Module;

pub struct MyModule;

impl Module for MyModule {
    fn new() -> Self {
        MyModule {}
    }

    fn run(&mut self) {
        println!("Hello from MyModule!");
    }
}
