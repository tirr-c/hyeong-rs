use std::io::{self, Read, Write};
use super::structure::HeartTree;
use super::rational::{Rational, HyeongRational};
use super::utf8::read_codepoint;


pub trait HyeongStack {
    fn push_one(&mut self, value: HyeongRational);
    fn pop_one(&mut self) -> HyeongRational;
    fn flush(&mut self) -> io::Result<()>;
}

impl HyeongStack for Vec<HyeongRational> {
    fn push_one(&mut self, value: HyeongRational) {
        self.push(value);
    }

    fn pop_one(&mut self) -> HyeongRational {
        self.pop().into()
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}


pub struct HyeongReadStack<R> {
    inner: R,
    stack: Vec<HyeongRational>,
}

impl<R> HyeongReadStack<R> {
    pub fn new(inner: R) -> Self {
        HyeongReadStack {
            inner: inner,
            stack: vec![],
        }
    }
}

impl HyeongReadStack<io::Stdin> {
    pub fn from_stdin() -> Self {
        HyeongReadStack {
            inner: io::stdin(),
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
            return match read_codepoint(&mut self.inner) {
                Ok(c) => HyeongRational::from_u32(c),
                Err(_) => HyeongRational::NaN,
            };
        }
        self.stack.pop_one()
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}


pub struct HyeongWriteStack<W> {
    inner: W,
}

impl<W> HyeongWriteStack<W> {
    pub fn new(inner: W) -> Self {
        HyeongWriteStack {
            inner: inner,
        }
    }
}

impl HyeongWriteStack<io::Stdout> {
    pub fn from_stdout() -> Self {
        HyeongWriteStack {
            inner: io::stdout(),
        }
    }
}

impl HyeongWriteStack<io::Stderr> {
    pub fn from_stderr() -> Self {
        HyeongWriteStack {
            inner: io::stderr(),
        }
    }
}

impl<W: Write> HyeongStack for HyeongWriteStack<W> {
    fn push_one(&mut self, value: HyeongRational) {
        write!(&mut self.inner, "{}", value).unwrap();
    }

    fn pop_one(&mut self) -> HyeongRational { HyeongRational::NaN }

    fn flush(&mut self) -> io::Result<()> { self.inner.flush() }
}


pub enum StackWrapper<'a> {
    Owned(Box<HyeongStack>),
    Borrowed(&'a mut HyeongStack),
}

impl<'a> StackWrapper<'a> {
    pub fn from_owned(stack: Box<HyeongStack>) -> Self {
        StackWrapper::Owned(stack)
    }
    pub fn from_ref_mut(stack: &'a mut HyeongStack) -> Self {
        StackWrapper::Borrowed(stack)
    }
}

impl<'a, R: 'static + Read> From<HyeongReadStack<R>> for StackWrapper<'a> {
    fn from(item: HyeongReadStack<R>) -> StackWrapper<'a> {
        StackWrapper::from_owned(Box::new(item))
    }
}

impl<'a, R: Read> From<&'a mut HyeongReadStack<R>> for StackWrapper<'a> {
    fn from(item: &'a mut HyeongReadStack<R>) -> StackWrapper<'a> {
        StackWrapper::from_ref_mut(item)
    }
}

impl<'a, W: 'static + Write> From<HyeongWriteStack<W>> for StackWrapper<'a> {
    fn from(item: HyeongWriteStack<W>) -> StackWrapper<'a> {
        StackWrapper::from_owned(Box::new(item))
    }
}

impl<'a, W: Write> From<&'a mut HyeongWriteStack<W>> for StackWrapper<'a> {
    fn from(item: &'a mut HyeongWriteStack<W>) -> StackWrapper<'a> {
        StackWrapper::from_ref_mut(item)
    }
}

impl<'a> HyeongStack for StackWrapper<'a> {
    fn push_one(&mut self, value: HyeongRational) {
        match self {
            &mut StackWrapper::Owned(ref mut stack) => stack.push_one(value),
            &mut StackWrapper::Borrowed(ref mut stack) => stack.push_one(value),
        }
    }

    fn pop_one(&mut self) -> HyeongRational {
        match self {
            &mut StackWrapper::Owned(ref mut stack) => stack.pop_one(),
            &mut StackWrapper::Borrowed(ref mut stack) => stack.pop_one(),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut StackWrapper::Owned(ref mut stack) => stack.flush(),
            &mut StackWrapper::Borrowed(ref mut stack) => stack.flush(),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HeartResult {
    Heart(usize),
    Return,
    Nil,
}

use std::collections::BTreeMap;
pub struct StackManager<'a> {
    stacks: BTreeMap<usize, StackWrapper<'a>>,
    selected: usize,
    exit_code: Option<isize>
}

impl<'a> StackManager<'a> {
    pub fn from_stacks(stdin:  StackWrapper<'a>,
                       stdout: StackWrapper<'a>,
                       stderr: StackWrapper<'a>
                      ) -> Self {
        let mut stacks = BTreeMap::new();
        stacks.insert(0, stdin);
        stacks.insert(1, stdout);
        stacks.insert(2, stderr);

        let mut stack = StackManager {
            stacks: stacks,
            selected: 0,
            exit_code: None,
        };
        stack.select(3);
        stack
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

    pub fn exit_code(&self) -> Option<isize> {
        self.exit_code
    }

    pub fn push(&mut self, hangul: usize, dots: usize) {
        let value = Rational::from_integer(((hangul * dots) as isize).into());
        self.stacks.get_mut(&self.selected).unwrap().push_one(value.into());
    }

    pub fn add(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut sum = HyeongRational::from_u32(0);
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            for _ in 0..count {
                sum += stack_from.pop_one();
            }
            sum
        };
        self.make_stack(to);
        let stack_to = self.stacks.get_mut(&to).unwrap();
        stack_to.push_one(sum);
    }

    pub fn mul(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut sum = HyeongRational::from_u32(1);
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            for _ in 0..count {
                sum *= stack_from.pop_one();
            }
            sum
        };
        self.make_stack(to);
        let stack_to = self.stacks.get_mut(&to).unwrap();
        stack_to.push_one(sum);
    }

    pub fn neg(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut temp = vec![];
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            for _ in 0..count {
                temp.push(-stack_from.pop_one());
            }
            let sum = temp.iter().fold(HyeongRational::from_u32(0), |a, b| a + b.clone());
            let mut temp = temp.into_iter();
            while let Some(r) = temp.next_back() {
                stack_from.push_one(r);
            }
            sum
        };
        self.make_stack(to);
        let stack_to = self.stacks.get_mut(&to).unwrap();
        stack_to.push_one(sum);
    }

    pub fn recip(&mut self, count: usize, to: usize) {
        if self.check_exit() { return; }
        let sum = {
            let mut temp = vec![];
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            for _ in 0..count {
                temp.push(stack_from.pop_one().recip());
            }
            let sum = temp.iter().fold(HyeongRational::from_u32(1), |a, b| a * b.clone());
            let mut temp = temp.into_iter();
            while let Some(r) = temp.next_back() {
                stack_from.push_one(r);
            }
            sum
        };
        self.make_stack(to);
        let stack_to = self.stacks.get_mut(&to).unwrap();
        stack_to.push_one(sum);
    }

    pub fn duplicate(&mut self, count: usize, into: usize) {
        if self.check_exit() { return; }
        let value = {
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            let value = stack_from.pop_one();
            stack_from.push_one(value.clone());
            value
        };
        self.select(into);
        let stack_to = self.stacks.get_mut(&self.selected).unwrap();
        for _ in 0..count {
            stack_to.push_one(value.clone());
        }
    }

    pub fn process_hearts(&mut self, heart: &HeartTree, target: usize) -> HeartResult {
        match heart {
            &HeartTree::Heart(id) => HeartResult::Heart(id),
            &HeartTree::Return => HeartResult::Return,
            &HeartTree::Nil => HeartResult::Nil,
            &HeartTree::LessThan(ref l, ref r) => {
                if self.stack_less_than(target) {
                    self.process_hearts(l, target)
                } else {
                    self.process_hearts(r, target)
                }
            },
            &HeartTree::Equals(ref l, ref r) => {
                if self.stack_equals(target) {
                    self.process_hearts(l, target)
                } else {
                    self.process_hearts(r, target)
                }
            },
        }
    }

    fn stack_less_than(&mut self, target: usize) -> bool {
        let target = HyeongRational::from_usize(target);
        let value = {
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            stack_from.pop_one()
        };
        value < target
    }

    fn stack_equals(&mut self, target: usize) -> bool {
        let target = HyeongRational::from_usize(target);
        let value = {
            let stack_from = self.stacks.get_mut(&self.selected).unwrap();
            stack_from.pop_one()
        };
        value == target
    }

    fn select(&mut self, id: usize) {
        self.make_stack(id);
        self.selected = id;
    }

    fn make_stack(&mut self, id: usize) {
        if self.stacks.contains_key(&id) { return; }
        let new_stack: Box<HyeongStack> = Box::new(vec![]);
        self.stacks.insert(id, StackWrapper::from_owned(new_stack));
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stacks.get_mut(&1).unwrap().flush()?;
        self.stacks.get_mut(&2).unwrap().flush()
    }
}


#[cfg(test)]
mod tests {
    use super::super::rational::{Rational, HyeongRational};
    use super::{HyeongStack, HyeongReadStack, HyeongWriteStack, StackManager};

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
            stack.push_one(Rational::from_integer((-32 as isize).into()).into());
            stack.push_one(HyeongRational::NaN);
            stack.push_one(Rational::new((65*3+2isize).into(), 3isize.into()).into());
            stack.push_one(Rational::new((-11isize).into(), 7isize.into()).into());
        };
        assert_eq!(&buf[..], "흑..!32너무 커엇...A2".as_bytes());
    }

    #[test]
    fn stack_manager_push_duplicate() {
        let test_str = "";
        let mut buf = vec![];
        let mut buf_err = vec![];
        {
            let stdin =  HyeongReadStack::new(test_str.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut buf);
            let mut stderr = HyeongWriteStack::new(&mut buf_err);
            let mut manager = StackManager::from_stacks(stdin.into(), (&mut stdout).into(), (&mut stderr).into());

            manager.push(5, 13);     // 혀어어어엉.............
            manager.duplicate(3, 1); // 흐으윽.
        }
        assert_eq!(&buf[..], "AAA".as_bytes());
    }

    #[test]
    fn stack_manager_add_mul() {
        let test_str = "A";
        let mut buf = vec![];
        let mut buf_err = vec![];
        {
            let stdin =  HyeongReadStack::new(test_str.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut buf);
            let mut stderr = HyeongWriteStack::new(&mut buf_err);
            let mut manager = StackManager::from_stacks(stdin.into(), (&mut stdout).into(), (&mut stderr).into());

            manager.duplicate(1, 0); // 흑
            manager.add(1, 2);       // 항..
            manager.add(1, 1);       // 항.
            manager.push(1, 1);      // 형.
            manager.push(1, 7);      // 형.......
            manager.push(2, 2);      // 혀엉..
            manager.push(1, 13);     // 형.............
            manager.push(3, 9);      // 혀어엉.........
            manager.mul(4, 0);       // 하아아아아앗
            manager.add(2, 2);       // 하앙..
        }
        assert_eq!(&buf[..], "A".as_bytes());
        assert_eq!(&buf_err[..], "너무 커엇...\u{2665}".as_bytes());
    }

    #[test]
    fn stack_manager_mul_recip() {
        let test_str = "밯망희";
        let mut buf = vec![];
        let mut buf_err = vec![];
        {
            let stdin =  HyeongReadStack::new(test_str.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut buf);
            let mut stderr = HyeongWriteStack::new(&mut buf_err);
            let mut manager = StackManager::from_stacks(stdin.into(), (&mut stdout).into(), (&mut stderr).into());

            manager.push(4, 2);
            manager.push(2, 3);
            manager.recip(1, 4);
            manager.mul(2, 3);
            manager.neg(1, 2);
            manager.push(1, 0);
            manager.duplicate(1, 0);
            manager.neg(5, 2);
            manager.add(1, 4);
            manager.add(1, 1);
            manager.add(1, 1);
            manager.add(1, 1);
        }
        assert_eq!(&buf[..], "481754758155148".as_bytes());
        assert_eq!(&buf_err[..], "2너무 커엇...".as_bytes());
    }
}
