use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

use clap::Parser;
use rshyeong::{HyeongReadStack, HyeongWriteStack, Processor, StackManager};

#[derive(Debug, Parser)]
#[clap(version, about)]
struct Options {
    #[clap(short, long, default_value = "-")]
    /// Input file, stdin by default
    input: PathBuf,
    /// Output file, stdout by default
    #[clap(short, long, default_value = "-")]
    output: PathBuf,
    /// Input source code
    source: PathBuf,
}

fn main() {
    let Options {
        input,
        output,
        source,
    } = Options::parse();

    let mut source = match File::open(source) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot open source file: {}", e);
            std::process::exit(2);
        }
    };
    let mut source_string = String::new();
    if let Err(e) = source.read_to_string(&mut source_string) {
        eprintln!("Cannot read source file: {}", e);
        std::process::exit(2);
    }

    let stdin: HyeongReadStack<Box<dyn Read>> = if input.as_os_str() == "-" {
        HyeongReadStack::new(Box::new(std::io::stdin()))
    } else {
        let file = match File::open(input) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Cannot open input file: {}", e);
                std::process::exit(2);
            }
        };
        HyeongReadStack::new(Box::new(file))
    };

    let stdout: HyeongWriteStack<Box<dyn Write>> = if output.as_os_str() == "-" {
        HyeongWriteStack::new(Box::new(std::io::stdout()))
    } else {
        let file = match File::create(output) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Cannot open output file: {}", e);
                std::process::exit(2);
            }
        };
        HyeongWriteStack::new(Box::new(BufWriter::new(file)))
    };

    let stderr = HyeongWriteStack::new(std::io::stderr());

    let stacks = StackManager::from_stacks(stdin, stdout, stderr);
    let parser = rshyeong::Parser::new(&source_string);
    let processor = Processor::with_stack_manager(parser, stacks);

    let (exit_code, err) = processor.run();
    if let Err(e) = err {
        eprintln!(
            "Error during flushing: {}\nExit code was: {}",
            e,
            exit_code,
        );
        std::process::exit(3);
    }
    std::process::exit(exit_code as i32);
}
