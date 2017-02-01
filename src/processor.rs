use std::collections::HashMap;
use super::structure::{Instruction, OperationType};
use super::stack::{StackManager, HeartResult};

pub struct Processor<'a, P> {
    inner: P,
    instructions: Vec<Instruction>,
    position: usize,
    stacks: StackManager<'a>,
    last_jump: Option<usize>,
    labels: HashMap<(usize, usize), usize>,
}

impl<'a, P> Processor<'a, P> {
    pub fn new(inner: P) -> Self {
        Processor {
            inner: inner,
            instructions: vec![],
            position: 0,
            stacks: StackManager::new(),
            last_jump: None,
            labels: HashMap::new(),
        }
    }
    pub fn with_stack_manager(inner: P, stacks: StackManager<'a>) -> Self {
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

impl<'a, P: Iterator<Item = Instruction>> Processor<'a, P> {
    pub fn run(&mut self) -> isize {
        loop {
            match self.advance() {
                Some(x) => { return x; },
                None => { },
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
            },
            OperationType::Add => {
                self.stacks.add(instr.hangul_count(), instr.dots());
            },
            OperationType::Multiply => {
                self.stacks.mul(instr.hangul_count(), instr.dots());
            },
            OperationType::Negate => {
                self.stacks.neg(instr.hangul_count(), instr.dots());
            },
            OperationType::Reciprocate => {
                self.stacks.recip(instr.hangul_count(), instr.dots());
            },
            OperationType::Duplicate => {
                self.stacks.duplicate(instr.hangul_count(), instr.dots());
            },
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
                } else { self.position += 1; }
            },
            HeartResult::Return => {
                if let Some(next) = self.last_jump {
                    self.position = next;
                } else { self.position += 1; }
            },
            _ => { self.position += 1; }
        }

        self.stacks.exit_code()
    }
}


#[cfg(test)]
mod tests {
    use super::super::parser::Parser;
    use super::super::stack::{HyeongReadStack, HyeongWriteStack, StackManager};
    use super::Processor;

    #[test]
    fn hello_world() {
        let source = include_str!("../snippets/hello-world.hyeong");
        let input = "";
        let expected = include_bytes!("../snippets/hello-world.stdout");
        let mut output = vec![];
        let mut error = vec![];

        let exit_code = {
            let stdin  = HyeongReadStack::new(input.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut output);
            let mut stderr = HyeongWriteStack::new(&mut error);
            let parser = Parser::from_str(source);
            let stacks = StackManager::from_stacks(stdin.into(), (&mut stdout).into(), (&mut stderr).into());
            let mut processor = Processor::with_stack_manager(parser, stacks);

            processor.run()
        };
        assert_eq!(exit_code, 0);
        assert_eq!(&output[..], expected);
        assert_eq!(error.len(), 0);
    }
}
