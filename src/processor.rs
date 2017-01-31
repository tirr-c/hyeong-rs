use std::collections::HashMap;
use super::structure::{Instruction, OperationType};
use super::stack::{StackManager, HeartResult};

struct Processor<'a, P> {
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
}

impl<'a, P: Iterator<Item = Instruction>> Processor<'a, P> {
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
                }
            },
            HeartResult::Return => {
                if let Some(next) = self.last_jump {
                    self.last_jump = Some(self.position);
                    self.position = next;
                }
            },
            _ => { }
        }
        None
    }
}
