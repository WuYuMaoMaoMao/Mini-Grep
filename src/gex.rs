use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
pub struct Gex {}
impl Gex {
    pub fn read_regex() -> String {
        let sales_and_products = {
            let file_content = fs::read_to_string("G:/Rust/Rust-demo/MiniLog/src/log.json")
                .expect("LogRocket: error reading file");
            serde_json::from_str::<Value>(&file_content)
                .expect("LogRocket: error serializing to JSON")
        };

        let gex = serde_json::to_string_pretty(&sales_and_products).expect("error parsing json");
        return gex;
    }
    pub fn match_regex(regex: Vec<&str>, text: String) {
        for r in regex {
            let re = Regex::new(r).unwrap();
            if re.is_match(&text) {
                println!("Matched: {}", r);
            }
        }
    }
}
