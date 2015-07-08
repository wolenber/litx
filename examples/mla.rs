extern crate litx;

use litx::{ Document, Strategy };

const SOURCE: &'static str = include_str!("mla.litx");
const STRATEGY: &'static str = include_str!("strategy/mla.litx");

fn main() {
    let strat = Strategy::new(STRATEGY);
    let doc = Document::new(SOURCE, strat.unwrap());
    match doc {
        Ok(d) => println!("{:#?}", d),
        Err(e) => println!("Error: {}", e)
    }
}