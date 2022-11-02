use toml;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use simple_os_sim_monitor::{tools,config};



fn main() {
    let fh = fs::read_to_string(Path::new("siminfo.toml"));
    let fc = match fh {
        Ok(f) => f,
        Err(_) => std::process::exit(1),
    };
    let conf_try = toml::from_str(&fc);
    let conf: config::SimConfig = match conf_try {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    println!("\n\n");
    loop {
        let sim_stats = match tools::get_stats(&conf) {
            Some(s) => s,
            None => continue
        };
        print!(
"\x1b[3A\r{}'s stats:\t\tVersion:{}
\x1b[KAgents      Other Stats
\x1b[KRoot\tChild\tPrims\tFPS\tThreads\t\"Unack Bytes\"
\x1b[K{}\t{}\t{}\t{}\t{}\t{}",
            tools::strip_quotes(sim_stats["RegionName"].to_string()),
            tools::strip_quotes(sim_stats["Version"].to_string()),
            tools::strip_quotes(sim_stats["RootAg"].to_string()),
            tools::get_i32(sim_stats["ChldAg"].to_string()),
            tools::get_i32(sim_stats["Prims"].to_string()),
            tools::get_i32(sim_stats["PhyFPS"].to_string()),
            tools::get_i32(sim_stats["System Thread Count"].to_string()),
            tools::get_i32(sim_stats["UnackB"].to_string()),
        );
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
    }
}
