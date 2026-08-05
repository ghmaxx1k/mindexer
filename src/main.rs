// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

use std::fs;
use std::path::Path;

#[allow(non_snake_case)]
fn main() {    
    
    let CONFIG_PATH = "/etc/mindexer/config";
    let mut RUNTIME_VARIABLES = vec![""];
    let mut MINDEXER_BASE_NAME = vec![""];

    if Path::new(CONFIG_PATH).is_file() {
        let CONFIG = fs::read_to_string(CONFIG_PATH).expect("failed to read file");
        for line in CONFIG.lines() {
            let (KEY, VALUE) = line.split_once('=').unwrap();
            if VALUE == "true" {
                RUNTIME_VARIABLES.push(KEY);
            }
            if KEY == "mindexerbasename" {
                MINDEXER_BASE_NAME.push(VALUE);
            }
        }
        // indexer
    } else {
        println!("[ error ] config file not found, halting");
        return();
    }

}