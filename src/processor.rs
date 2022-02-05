use super::stack::{HeartResult, StackManager};
use super::structure::{Instruction, OperationType};
use std::collections::HashMap;
use std::io::{self, Read, Write};

pub struct Processor<P, I: Read, O: Write, E: Write> {
    inner: P,
    instructions: Vec<Instruction>,
    position: usize,
    stacks: StackManager<I, O, E>,
    last_jump: Option<usize>,
    labels: HashMap<(usize, usize), usize>,
}

impl<P, I: Read, O: Write, E: Write> Processor<P, I, O, E> {
    pub fn with_stack_manager(inner: P, stacks: StackManager<I, O, E>) -> Self {
        Processor {
            inner: inner,
            instructions: vec![],
            position: 0,
            stacks: stacks,
            last_jump: None,
            labels: HashMap::new(),
        }
    }
}

impl<P, I: Read, O: Write, E: Write> Drop for Processor<P, I, O, E> {
    fn drop(&mut self) {
        self.stacks.flush().unwrap();
    }
}

impl<P: Iterator<Item = Instruction>, I: Read, O: Write, E: Write> Processor<P, I, O, E> {
    pub fn run(mut self) -> (isize, io::Result<()>) {
        loop {
            if let Some(x) = self.advance() {
                return (x, self.stacks.flush());
            }
        }
    }

    pub fn advance(&mut self) -> Option<isize> {
        if self.instructions.len() <= self.position {
            match self.inner.next() {
                None => self.position = 0,
                Some(instr) => self.instructions.push(instr),
            }
        }
        let instr = &self.instructions[self.position];

        match instr.operation_type() {
            OperationType::Push => {
                self.stacks.push(instr.hangul_count(), instr.dots());
            }
            OperationType::Add => {
                self.stacks.add(instr.hangul_count(), instr.dots());
            }
            OperationType::Multiply => {
                self.stacks.mul(instr.hangul_count(), instr.dots());
            }
            OperationType::Negate => {
                self.stacks.neg(instr.hangul_count(), instr.dots());
            }
            OperationType::Reciprocate => {
                self.stacks.recip(instr.hangul_count(), instr.dots());
            }
            OperationType::Duplicate => {
                self.stacks.dup(instr.hangul_count(), instr.dots());
            }
        }

        let param = instr.hangul_times_dots();
        let heart = instr.heart_tree();
        let result = self.stacks.process_hearts(heart, param);
        match result {
            HeartResult::Heart(id) => {
                let label = (param, id);
                let next = *(self.labels.entry(label).or_insert(self.position));
                if next != self.position {
                    self.last_jump = Some(self.position);
                    self.position = next;
                } else {
                    self.position += 1;
                }
            }
            HeartResult::Return => {
                if let Some(next) = self.last_jump {
                    self.position = next;
                } else {
                    self.position += 1;
                }
            }
            _ => {
                self.position += 1;
            }
        }

        self.stacks.exit_code()
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::Parser;
    use super::super::stack::{HyeongReadStack, HyeongWriteStack, StackManager};
    use super::Processor;

    macro_rules! test_path {
        ($name:expr, $ext:expr) => {
            concat!("../snippets/", $name, ".", $ext)
        };
    }
    macro_rules! make_input {
        ($name:expr, ) => (b"");
        ($name:expr, input $($r:ident)*) => (
            include_bytes!(test_path!($name, "stdin"))
            );
        ($name:expr, $i:ident $($next:ident)*) => (make_input!($name, $($next)*));
    }
    macro_rules! make_output {
        ($name:expr, ) => ((false, b""));
        ($name:expr, output $($r:ident)*) => (
            (true, include_bytes!(test_path!($name, "stdout")))
            );
        ($name:expr, $i:ident $($next:ident)*) => (make_output!($name, $($next)*));
    }
    macro_rules! make_error {
        ($name:expr, ) => ((false, b""));
        ($name:expr, error $($r:ident)*) => (
            (true, include_bytes!(test_path!($name, "stderr")))
            );
        ($name:expr, $i:ident $($next:ident)*) => (make_error!($name, $($next)*));
    }
    macro_rules! make_exitcode {
        ($name:expr, ) => ((false, 0));
        ($name:expr, exitcode $($r:ident)*) => (
            (true, include!(test_path!($name, "exitcode")))
            );
        ($name:expr, $i:ident $($next:ident)*) => (make_exitcode!($name, $($next)*));
    }

    macro_rules! test {
        ($name:expr $(, $t:ident)*) => {
            let source = include_str!(test_path!($name, "hyeong"));
            let input = make_input!($name, $($t)*);
            let (test_output, expected_output) = make_output!($name, $($t)*);
            let (test_error, expected_error) = make_error!($name, $($t)*);
            let (test_exitcode, expected_exitcode) = make_exitcode!($name, $($t)*);
            let mut output = vec![];
            let mut error = vec![];

            let (exit_code, err) = {
                let stdin  = HyeongReadStack::new(&input[..]);
                let stdout = HyeongWriteStack::new(&mut output);
                let stderr = HyeongWriteStack::new(&mut error);
                let parser = Parser::new(source);
                let stacks = StackManager::from_stacks(stdin, stdout, stderr);
                let processor = Processor::with_stack_manager(parser, stacks);
                processor.run()
            };
            err.unwrap();

            if test_exitcode { assert_eq!(exit_code, expected_exitcode); }
            if test_output { assert_eq!(&output[..], expected_output); }
            if test_error { assert_eq!(&error[..], expected_error); }
        };
    }

    #[test]
    fn hello_world() {
        test!("hello-world", output, exitcode);
    }

    #[test]
    fn fibonacci() {
        test!("fibonacci", output, exitcode);
    }

    #[test]
    fn stderr() {
        test!("stderr", error, exitcode);
    }
}
