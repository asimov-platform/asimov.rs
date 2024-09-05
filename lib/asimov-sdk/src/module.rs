// This is free and unencumbered software released into the public domain.

#[stability::unstable]
pub trait Module {
    fn new() -> Self;
    fn run(&mut self);
}
