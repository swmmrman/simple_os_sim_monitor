
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