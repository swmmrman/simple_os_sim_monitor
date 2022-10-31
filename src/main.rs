use toml;
use std::fs;
use std::io::Write;
use std::path::Path;
use serde::Deserialize;
use reqwest;
use std::thread::sleep;
use std::time::Duration;
use serde_json::Value;

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

fn get_i32(input: String) -> i32 {
    let length = input.len();
    let clean = &input[1..length-1];
    let val = clean.parse::<i32>();
    match val {
        Ok(v) => v,
        Err(_) => -1,
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
    println!();
    loop {
        let resp = reqwest::blocking::get(conf.url());
        let resp_text = match resp {
            Ok(text) => text,
            Err(e) => {
                println!("{}", e); 
                continue;
            }
        };
        let resp_ct = match resp_text.text() {
            Ok(r) => r,
            Err(e) => { 
                println!("{}", e);
                continue;
            }
        };
        let sim_stats: Value = match serde_json::from_str(&resp_ct) {
            Ok(stats) => stats,
            Err(e) => {
                println!("Error deserailizing: {}", e);
                continue
            }
        };
        let fps = get_i32(sim_stats["PhyFPS"].to_string());
        print!("\x1b[1A\rPhysics FPS of {}: {}\nAgents: {}",
            sim_stats["RegionName"],
            fps,
            sim_stats["RootAg"],
        );
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
    }
}
