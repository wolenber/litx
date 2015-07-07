extern crate litx;

const SOURCE: &'static str = include_str!("simple.litx");

fn main() {
    let lexer = litx::Lexer::new(SOURCE);
    let program = litx::parser::parse(lexer).unwrap();
    println!("{:?}", program);
}