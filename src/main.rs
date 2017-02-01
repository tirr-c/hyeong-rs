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
    let parser = Parser::from_str(r#"형 흑.... 흑..... 흑... 형. 흣.♥ 흣.... 하앙..... 흑.... 하앙... 흑..... 하앙... 흑... 하앙... 혀어어엉........ 항.♥"#);
    let mut processor = Processor::new(parser);
    processor.run();
}
