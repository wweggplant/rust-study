extern crate regex;
pub mod compiler;
pub mod tokenizer;
pub mod parser;
pub mod transformer;

fn main() {
    println!("{:#?}", compiler::compiler("(add 2 (sub 4 3))"));
}
