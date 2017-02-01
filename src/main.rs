extern crate num;
#[macro_use]
extern crate clap;

mod structure;
mod parser;
mod utf8;
mod rational;
mod stack;
mod processor;

use std::fs::File;
use std::io::Read;
use self::parser::Parser;
use self::stack::{HyeongReadStack, HyeongWriteStack, StackManager};
use self::processor::Processor;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();


    let source_file = matches.value_of("SOURCE").unwrap();
    let mut source = match File::open(source_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Cannot open source file {}: {}", source_file, e);
            std::process::exit(2);
        }
    };
    let mut source_string = String::new();
    if let Err(e) = source.read_to_string(&mut source_string) {
        println!("Cannot read source file {}: {}", source_file, e);
        std::process::exit(2);
    }

    let input_file = matches.value_of("input").unwrap_or("-");
    let stdin = if input_file == "-" {
        HyeongReadStack::from_stdin().into()
    } else {
        let file = match File::open(input_file) {
            Ok(f) => f,
            Err(e) => {
                println!("Cannot open input file {}: {}", input_file, e);
                std::process::exit(2);
            }
        };
        HyeongReadStack::new(file).into()
    };

    let output_file = matches.value_of("output").unwrap_or("-");
    let stdout = if output_file == "-" {
        HyeongWriteStack::from_stdout().into()
    } else {
        let file = match File::create(output_file) {
            Ok(f) => f,
            Err(e) => {
                println!("Cannot open output file {}: {}", output_file, e);
                std::process::exit(2);
            }
        };
        HyeongWriteStack::new(file).into()
    };

    let stderr = HyeongWriteStack::from_stderr().into();


    let stacks = StackManager::from_stacks(stdin, stdout, stderr);
    let parser = Parser::from_str(&source_string);
    let processor = Processor::with_stack_manager(parser, stacks);

    let exit_code = processor.run();
    std::process::exit(exit_code as i32);
}
