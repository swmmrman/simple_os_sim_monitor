use toml;
use std::fs;
use std::io::Write;
use std::path::Path;
use serde::Deserialize;
use reqwest;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct SimConfig {
    protocol: String,
    server_name: String,
    port: u32,
    end_point: String,
}

#[derive(Debug, Deserialize)]
struct SimStats {
    Dilatn:                         String,
    SimFPS:                         String,
    PhyFPS:                         String,
    AgntUp:                         String,
    RootAg:                         String,
    ChldAg:                         String,
    NPCAg:                          String,
    Prims:                          String,
    AtvPrm:                         String,
    AtvScr:                         String,
    ScrLPS:                         String,
    ScrEPS:                         String,
    PktsIn:                         String,
    PktOut:                         String,
    PendDl:                         String,
    PendUl:                         String,
    UnackB:                         String,
    TotlFt:                         String,
    NetFt:                          String,
    PhysFt:                         String,
    OthrFt:                         String,
    AgntFt:                         String,
    ImgsFt:                         String,
    FrameDilatn:                    String,
    #[serde(rename = "Logging in Users")]
    Logging_in_Users:               String,
    GeoPrims:                       String,
    #[serde(rename = "Mesh Objects")]
    Mesh_Objects:                   String,
    #[serde(rename = "Script Engine Thread Count")]
    Script_Engine_Thread_Count:     String,
    #[serde(rename = "Util Thread Count")]
    Util_Thread_Count:              String,
    #[serde(rename = "System Thread Count")]
    System_Thread_Count:            String,
    #[serde(rename = "System Thread Active")]
    System_Thread_Active:           String,
    ProcMem:                        String,
    Memory:                         String,
    Uptime:                         String,
    Version:                        String,
    RegionName:                     String,
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
        let sim_stats: SimStats = match serde_json::from_str(&resp_ct) {
            Ok(stats) => stats,
            Err(e) => {
                println!("Error deserailizing: {}", e);
                continue
            }
        };
        print!("\x1b[1A\rPhysics FPS of {}: {}\nAgents: {}",
            sim_stats.RegionName,
            sim_stats.PhyFPS.parse::<u32>().unwrap(),
            sim_stats.RootAg,
        );
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
    }
}
