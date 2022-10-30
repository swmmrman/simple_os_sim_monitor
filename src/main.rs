use toml;
use std::fs::File;
use std::path::Path;

fn main() {
    let fh = File::open(Path::new("siminfo.toml"));
    println!("Hello, world!");

}
