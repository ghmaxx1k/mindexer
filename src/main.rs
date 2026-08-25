// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

use std::fs;
use glob::glob;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    
    let config_path = "/etc/mindexer/config.txt";
    let log_path = "/etc/mindexer/log.txt";
    let mut base: Vec<String> = vec![];
    let config_variables: Vec<String> = vec![];
    
    if !Path::new(config_path).is_file() {
        println!("[e] config file not found");
        return();
    }
    if !Path::new(log_path).is_file() {
        println!("[e] log file not found");
        return();
    }

    for line in fs::read_to_string(config_path) {
        println!("{}", line);
    }

}