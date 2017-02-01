extern crate num;

mod structure;
mod parser;
mod utf8;
mod rational;
mod stack;
mod processor;
use self::parser::Parser;
use self::processor::Processor;

fn main() {
    let parser = Parser::from_str(r#"형 형...... 하앙...♥!♡ 항. 형 형...... 하앙...♥!♡"#);
    let mut processor = Processor::new(parser);
    processor.run();
}
