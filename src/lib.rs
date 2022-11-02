//use toml;
use serde_derive::Deserialize;

pub mod tools {
    use serde_json::Value;
    use reqwest;
    pub fn get_i32(input: String) -> i32 {
        let clean = strip_quotes(input);
        let val = clean.parse::<i32>();
        match val {
            Ok(v) => v,
            Err(_) => -255,
        }
    }

    pub fn strip_quotes(input: String) -> String {
        let length = input.len();
        input[1..length-1].to_string()
    }
    pub fn get_stats(conf: &crate::config::SimConfig ) -> Option<Value>{
        let resp = reqwest::blocking::get(conf.url());
        let resp_text = match resp {
            Ok(text) => Some(text),
            Err(e) => {
                println!("{}", e); 
                None
            }
        };
        let resp_ct = match resp_text.unwrap().text() {
            Ok(r) => Some(r),
            Err(e) => { 
                println!("{}", e);
                None
            }
        };
        let sim_stats: Option<Value> = match serde_json::from_str(&resp_ct.unwrap()) {
            Ok(stats) => Some(stats),
            Err(e) => {
                println!("Error deserailizing: {}", e);
                None
            }
        };
        sim_stats
    }
}
pub mod config {
    #[derive(Debug, crate::Deserialize)]
    pub struct SimConfig {
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
}