use serde_derive::Deserialize;
pub mod tools {
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