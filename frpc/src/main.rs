use std::env;

mod args;

fn main() {
    if let Some((command, matches)) = args::init() {
        dbg!(command, matches);
    };
}
