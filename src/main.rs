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
    let clean = strip_quotes(input);
    let val = clean.parse::<i32>();
    match val {
        Ok(v) => v,
        Err(_) => -255,
    }
}

fn strip_quotes(input: String) -> String {
    let length = input.len();
    input[1..length-1].to_string()
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
    println!("\n\n");
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
        let r_name = strip_quotes(sim_stats["RegionName"].to_string());
        let agents = strip_quotes(sim_stats["RootAg"].to_string());
        print!(
            "\x1b[3A\r{}'s stats:\t\tVersion:{}\nAgents\t\tOther Stats\nRoot\tChild\tPrims\tFPS\tThreads\tUnack Bytes\n{}\t{}\t{}\t{}\t{}\t{}",
            r_name,
            strip_quotes(sim_stats["Version"].to_string()),
            agents,
            get_i32(sim_stats["ChldAg"].to_string()),
            get_i32(sim_stats["Prims"].to_string()),
            fps,
            get_i32(sim_stats["System Thread Count"].to_string()),
            get_i32(sim_stats["UnackB"].to_string()),
        );
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
    }
}
