use toml;
use std::fs;
use std::path::Path;
use serde::Deserialize;
use reqwest;

#[derive(Debug, Deserialize)]
struct SimConfig {
    protocol: String,
    server_name: String,
    port: u32,
    end_point: String,
}

impl SimConfig {
    pub fn url(&self) -> String {
        format!("{}://{}:{}/{}", self.protocol, self.server_name, self.port, self.end_point)
    }
}

fn main() {
    let fh = fs::read_to_string(Path::new("siminfo.toml"));
    let fc = match fh {
        Ok(f) => f,
        Err(_) => std::process::exit(1),
    };
    let conf_try = toml::from_str(&fc);
    let conf: SimConfig = match conf_try {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", &conf);
    let resp = reqwest::blocking::get(conf.url());
    let resp_text = match resp {
        Ok(text) => text,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}",resp_text)
}
