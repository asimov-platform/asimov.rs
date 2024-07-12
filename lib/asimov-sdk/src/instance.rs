// This is free and unencumbered software released into the public domain.

#[allow(unused)]
#[derive(Debug)]
pub struct Instance {}

#[allow(unused)]
impl Instance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        //eprintln!("Dropping Instance");
    }
}
