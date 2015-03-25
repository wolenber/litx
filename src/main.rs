#![cfg(not(test))]

extern crate litx;

use litx::Document;
use litx::Strategy;

use std::path::Path;

fn main() {
    let strats = vec![
        "strategy/mla.litx",
    ];
    let docs = vec![
        "example_docs/mla.litx",
        "example_docs/simple.litx",
    ];
    for filepath in &strats {
        print!("\n\n\n");
        println!("File: {}", filepath);
        let strat = match Strategy::new_from_file(Path::new(filepath)) {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); continue; }
        };
        println!("Success: {:?}", strat);
    }
    for filepath in &docs {
        print!("\n\n\n");
        println!("File: {}", filepath);
        let doc = match Document::new_from_file(Path::new(filepath)) {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); continue; }
        };
        println!("Success: {:?}", doc);
    }
}
