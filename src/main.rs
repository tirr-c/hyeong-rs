#[macro_use]
extern crate clap;
extern crate rshyeong;

use rshyeong::{HyeongReadStack, HyeongWriteStack, Parser, Processor, StackManager};
use std::fs::File;
use std::io::{BufWriter, Read, Write};

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
    let stdin: HyeongReadStack<Box<dyn Read>> = if input_file == "-" {
        HyeongReadStack::new(Box::new(std::io::stdin()))
    } else {
        let file = match File::open(input_file) {
            Ok(f) => f,
            Err(e) => {
                println!("Cannot open input file {}: {}", input_file, e);
                std::process::exit(2);
            }
        };
        HyeongReadStack::new(Box::new(file))
    };

    let output_file = matches.value_of("output").unwrap_or("-");
    let stdout: HyeongWriteStack<Box<dyn Write>> = if output_file == "-" {
        HyeongWriteStack::new(Box::new(std::io::stdout()))
    } else {
        let file = match File::create(output_file) {
            Ok(f) => f,
            Err(e) => {
                println!("Cannot open output file {}: {}", output_file, e);
                std::process::exit(2);
            }
        };
        HyeongWriteStack::new(Box::new(BufWriter::new(file)))
    };

    let stderr = HyeongWriteStack::new(std::io::stderr());

    let stacks = StackManager::from_stacks(stdin, stdout, stderr);
    let parser = Parser::new(&source_string);
    let processor = Processor::with_stack_manager(parser, stacks);

    let (exit_code, err) = processor.run();
    if let Err(e) = err {
        if writeln!(
            std::io::stderr(),
            "Error during flushing: {}\nExit code was: {}",
            e,
            exit_code
        )
        .is_err()
        {
            std::process::exit(3);
        }
        std::process::exit(3);
    }
    std::process::exit(exit_code as i32);
}
