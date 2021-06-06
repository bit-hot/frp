use std::env;

mod args;
mod conf;

fn main() {
    if let Some((command, matches)) = args::init() {
        dbg!(command, matches);
    };
    let base_conf = conf::base::load_base();
    dbg!(base_conf);
}
