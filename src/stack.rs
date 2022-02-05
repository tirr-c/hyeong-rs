use std::io::{self, Read, Write};
use super::structure::HeartTree;
use super::rational::{Rational, HyeongRational};
use super::utf8::read_codepoint;
use num::{Zero, One};


pub trait HyeongStack {
    fn push_one(&mut self, value: HyeongRational);
    fn pop_one(&mut self) -> HyeongRational;
}

impl HyeongStack for Vec<HyeongRational> {
    fn push_one(&mut self, value: HyeongRational) {
        self.push(value);
    }

    fn pop_one(&mut self) -> HyeongRational {
        self.pop().into()
    }
}


pub struct HyeongReadStack<R> {
    inner: R,
    stack: Vec<HyeongRational>,
}

impl<R> HyeongReadStack<R> {
    pub fn new(inner: R) -> Self {
        HyeongReadStack {
            inner,
            stack: vec![],
        }
    }
}

impl<R: Read> HyeongStack for HyeongReadStack<R> {
    fn push_one(&mut self, value: HyeongRational) {
        self.stack.push_one(value);
    }

    fn pop_one(&mut self) -> HyeongRational {
        if self.stack.is_empty() {
            if let Ok(c) = read_codepoint(&mut self.inner) {
                HyeongRational::from_u32(c)
            } else {
                HyeongRational::NaN
            }
        } else {
            self.stack.pop_one()
        }
    }
}


pub struct HyeongWriteStack<W> {
    inner: W,
}

impl<W> HyeongWriteStack<W> {
    pub fn new(inner: W) -> Self {
        HyeongWriteStack {
            inner,
        }
    }
}

impl<W: Write> HyeongWriteStack<W> {
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<W: Write> HyeongStack for HyeongWriteStack<W> {
    fn push_one(&mut self, value: HyeongRational) {
        write!(&mut self.inner, "{}", value).unwrap();
    }

    fn pop_one(&mut self) -> HyeongRational { HyeongRational::NaN }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HeartResult {
    Heart(usize),
    Return,
    Nil,
}

use std::collections::BTreeMap;
pub struct StackManager<I, O, E> {
    stdin:  HyeongReadStack<I>,
    stdout: HyeongWriteStack<O>,
    stderr: HyeongWriteStack<E>,
    stacks: BTreeMap<usize, Vec<HyeongRational>>,
    selected: usize,
    exit_code: Option<isize>
}

impl<I: Read, O: Write, E: Write> StackManager<I, O, E> {
    pub fn from_stacks(stdin:  HyeongReadStack<I>,
                       stdout: HyeongWriteStack<O>,
                       stderr: HyeongWriteStack<E>
                      ) -> Self {
        let mut stacks = BTreeMap::new();
        stacks.insert(3, vec![]);
        StackManager {
            stdin,
            stdout,
            stderr,
            stacks,
            selected: 3,
            exit_code: None,
        }
    }

    fn check_exit(&mut self) -> bool {
        if self.selected == 1 {
            self.exit_code = Some(0);
            true
        } else if self.selected == 2 {
            self.exit_code = Some(1);
            true
        } else {
            false
        }
    }

    fn selected_stack_mut(&mut self) -> &mut dyn HyeongStack {
        let id = self.selected;
        self.stack_mut(id)
    }

    fn stack_mut(&mut self, id: usize) -> &mut dyn HyeongStack {
        self.make_stack(id);
        match id {
            0 => &mut self.stdin,
            1 => &mut self.stdout,
            2 => &mut self.stderr,
            i => self.stacks.get_mut(&i).unwrap(),
        }
    }

    pub fn exit_code(&self) -> Option<isize> {
        self.exit_code
    }

    pub fn push(&mut self, hangul: usize, dots: usize) {
        let value = Rational::from_integer((hangul * dots) as isize);
        self.selected_stack_mut().push_one(value.into());
    }

    pub fn add(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut sum = HyeongRational::zero();
            let stack_from = self.selected_stack_mut();
            for _ in 0..count {
                sum += stack_from.pop_one();
            }
            sum
        };
        self.stack_mut(to).push_one(sum);
    }

    pub fn mul(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut sum = HyeongRational::one();
            let stack_from = self.selected_stack_mut();
            for _ in 0..count {
                sum *= stack_from.pop_one();
            }
            sum
        };
        self.stack_mut(to).push_one(sum);
    }

    pub fn neg(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut temp = vec![];
            let stack_from = self.selected_stack_mut();
            for _ in 0..count {
                temp.push(-stack_from.pop_one());
            }
            {
                let mut it = temp.iter();
                while let Some(r) = it.next_back() {
                    stack_from.push_one(r.clone());
                }
            }
            temp.into_iter().fold(HyeongRational::zero(), |a, b| a + b)
        };
        self.stack_mut(to).push_one(sum);
    }

    pub fn recip(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut temp = vec![];
            let stack_from = self.selected_stack_mut();
            for _ in 0..count {
                temp.push(stack_from.pop_one().recip());
            }
            {
                let mut it = temp.iter();
                while let Some(r) = it.next_back() {
                    stack_from.push_one(r.clone());
                }
            }
            temp.into_iter().fold(HyeongRational::one(), |a, b| a * b)
        };
        self.stack_mut(to).push_one(sum);
    }

    pub fn dup(&mut self, count: usize, into: usize) {
        if self.check_exit() { return; }
        let value = {
            let stack_from = self.selected_stack_mut();
            let value = stack_from.pop_one();
            stack_from.push_one(value.clone());
            value
        };
        self.selected = into;
        let stack_to = self.selected_stack_mut();
        for _ in 0..count {
            stack_to.push_one(value.clone());
        }
    }

    pub fn process_hearts(&mut self, heart: &HeartTree, target: usize) -> HeartResult {
        match *heart {
            HeartTree::Heart(id) => HeartResult::Heart(id),
            HeartTree::Return => HeartResult::Return,
            HeartTree::Nil => HeartResult::Nil,
            HeartTree::LessThan(ref l, ref r) => {
                let into = if self.stack_less_than(target) { l } else { r };
                self.process_hearts(into, target)
            },
            HeartTree::Equals(ref l, ref r) => {
                let into = if self.stack_equals(target) { l } else { r };
                self.process_hearts(into, target)
            },
        }
    }

    fn stack_less_than(&mut self, target: usize) -> bool {
        let target = HyeongRational::from_usize(target);
        let value = self.selected_stack_mut().pop_one();
        value < target
    }

    fn stack_equals(&mut self, target: usize) -> bool {
        let target = HyeongRational::from_usize(target);
        let value = self.selected_stack_mut().pop_one();
        value == target
    }

    fn make_stack(&mut self, id: usize) {
        match id {
            0 | 1 | 2 => {},
            i => { self.stacks.entry(i).or_insert_with(Vec::new); }
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush().and_then(|_| self.stderr.flush())
    }
}


#[cfg(test)]
mod tests {
    mod rw {
        use super::super::super::rational::{Rational, HyeongRational};
        use super::super::{HyeongStack, HyeongReadStack, HyeongWriteStack};

        #[test]
        fn read_stack_pop() {
            let test_str = "하앗...💕";
            let mut stack = HyeongReadStack::new(test_str.as_bytes());
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('하' as u32));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('앗' as u32));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('.' as u32));
            stack.push_one(HyeongRational::from_u32(14));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32(14));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('.' as u32));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('.' as u32));
            assert_eq!(stack.pop_one(), HyeongRational::from_u32('💕' as u32));
            assert!(stack.pop_one().is_nan());
        }

        #[test]
        fn write_stack_push() {
            let mut buf = vec![];
            {
                let mut stack = HyeongWriteStack::new(&mut buf);
                stack.push_one(HyeongRational::from_u32('흑' as u32));
                stack.push_one(HyeongRational::from_u32('.' as u32));
                stack.push_one(HyeongRational::from_u32('.' as u32));
                stack.push_one(HyeongRational::from_u32('!' as u32));
                stack.push_one(Rational::from_integer(-32isize).into());
                stack.push_one(HyeongRational::NaN);
                stack.push_one(Rational::new(65*3+2isize, 3isize).into());
                stack.push_one(Rational::new(-11isize, 7isize).into());
            };
            assert_eq!(&buf[..], "흑..!32너무 커엇...A2".as_bytes());
        }
    }

    mod manager {
        use super::super::{HyeongReadStack, HyeongWriteStack, StackManager};

        macro_rules! extract_arg {
            ($target:ident, [ $t:ident $v:expr ] $($rest:tt)*) => {
                if stringify!($target) == stringify!($t) {
                    (true, $v)
                } else {
                    extract_arg!($target, $($rest)*)
                }
            };
            ($target:ident, ) => ((false, ""));
        }
        macro_rules! make_test {
            ($m:ident $test:block $(, $t:ident $v:expr)*) => {{
                let (_, input) = extract_arg!(input, $([ $t $v ])*);
                let (test_output, expected_output) = extract_arg!(output, $([ $t $v ])*);
                let (test_error, expected_error) = extract_arg!(error, $([ $t $v ])*);
                let mut output = vec![];
                let mut error = vec![];
                {
                    let stdin = HyeongReadStack::new(input.as_bytes());
                    let stdout = HyeongWriteStack::new(&mut output);
                    let stderr = HyeongWriteStack::new(&mut error);
                    let mut $m = StackManager::from_stacks(stdin, stdout, stderr);
                    $test
                }
                if test_output { assert_eq!(&output[..], expected_output.as_bytes()); }
                if test_error  { assert_eq!( &error[..],  expected_error.as_bytes()); }
            }};
        }

        #[test]
        fn stack_manager_push_dup() {
            make_test!(manager {
                manager.push(5, 13);
                manager.dup(3, 1);
            }, output "AAA");
        }

        #[test]
        fn stack_manager_add_mul() {
            make_test!(manager {
                manager.dup(1, 0); // 흑
                manager.add(1, 2);       // 항..
                manager.add(1, 1);       // 항.
                manager.push(1, 1);      // 형.
                manager.push(1, 7);      // 형.......
                manager.push(2, 2);      // 혀엉..
                manager.push(1, 13);     // 형.............
                manager.push(3, 9);      // 혀어엉.........
                manager.mul(4, 0);       // 하아아아아앗
                manager.add(2, 2);       // 하앙..
            }, input "A", output "A", error "너무 커엇...\u{2665}");
        }

        #[test]
        fn stack_manager_mul_recip() {
            make_test!(manager {
                manager.push(4, 2);
                manager.push(2, 3);
                manager.recip(1, 4);
                manager.mul(2, 3);
                manager.neg(1, 2);
                manager.push(1, 0);
                manager.dup(1, 0);
                manager.neg(5, 2);
                manager.add(1, 4);
                manager.add(1, 1);
                manager.add(1, 1);
                manager.add(1, 1);
            }, input "밯망희", output "481754758155148", error "2너무 커엇...");
        }
    }
}
