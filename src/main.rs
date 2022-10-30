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

impl sim_config {
    pub fn url(&self) -> String {
        format!("{}://{}:{}/{}", self.protocol, self.server_name, self.port, self.end_point)
    }
}

fn main() {
    let fh = File::open(Path::new("siminfo.toml"));
    let fc = match fh {
        Ok(f) => f,
        Err(_) => std::process::exit(1),
    };
    println!("{:?}", &fc);
}
