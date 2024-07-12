// This is free and unencumbered software released into the public domain.

pub trait Module {
    fn new() -> Self;
    fn run(&mut self);
}
