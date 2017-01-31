extern crate num;

mod structure;
mod parser;
mod utf8;
mod rational;
mod stack;
use self::parser::Parser;

fn main() {
    let parser = Parser::from_str("하흐아읏...하아앙....흑..?!♥.혀엉...");
    for op in parser {
        println!("{:?}", op);
    }
}
