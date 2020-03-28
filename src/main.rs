use lazy_static::*;
use std::sync::RwLock;

lazy_static! {
    pub static ref RW: RwLock<bool> = RwLock::new(true);
}

fn main() {
    println!("Hello, world!");

    RW.read();
}