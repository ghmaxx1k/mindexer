// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

use std::fs;
use glob::glob;
use std::path::Path;
use std::collections::HashMap;

#[warn(for_loops_over_fallibles)]
fn main() {
    
    let config_path = "/etc/mindexer/config.txt";
    let log_path = "/etc/mindexer/log.txt";
    let mut config_base: Vec<String> = vec![];
    let mut config_variables: HashMap<String, String> = HashMap::new();
    
    if !Path::new(config_path).is_file() {
        println!("[e] config file not found");
        return();
    }
    if !Path::new(log_path).is_file() {
        println!("[e] log file not found");
        return();
    }

    for line in fs::read_to_string(config_path) {
        let (key, value) = line.split_once('=').unwrap();
        if key == "base" {
            config_base.push(value.to_string());
        }
        if key == "logging" {
            config_variables.insert(
                key.to_string(),
                value.to_string(),
            );
        }
    }

    println!("{:?}, {:?}", config_base, config_variables);

}