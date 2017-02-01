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
    let parser = Parser::from_str(r#"
    혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.
    혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..
    하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...
    혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗.
    혀엉..... 흑. 흣"#);
    let mut processor = Processor::new(parser);
    processor.run();
}
