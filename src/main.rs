use toml;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct sim_config {
    protocol: String,
    server_name: String,
    port: u32,
    end_point: String,
}

fn main() {
    let fh = File::open(Path::new("siminfo.toml"));
    println!("Hello, world!");

}
