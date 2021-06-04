use std::env;
mod args;

fn main() {
    let (command, matched) = args::init().unwrap();
    dbg!(command, matched);
}
