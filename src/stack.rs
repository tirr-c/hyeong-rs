use std;
use std::io::{self, Read, Write};
use num::traits::cast::{ToPrimitive, FromPrimitive};
use num::rational::Ratio;
use super::rational::{Rational, HyeongRational};
use super::utf8::read_codepoint;


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


struct HyeongReadStack<R> {
    inner: R,
    stack: Vec<HyeongRational>,
}

impl<R> HyeongReadStack<R> {
    fn new(inner: R) -> Self {
        HyeongReadStack {
            inner: inner,
            stack: vec![],
        }
    }
    fn into_inner(self) -> R {
        self.inner
    }
}

impl HyeongReadStack<std::io::Stdin> {
    fn from_stdin() -> Self {
        HyeongReadStack {
            inner: std::io::stdin(),
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
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl HyeongWriteStack<std::io::Stdout> {
    pub fn from_stdout() -> Self {
        HyeongWriteStack {
            inner: std::io::stdout(),
        }
    }
}

impl HyeongWriteStack<std::io::Stderr> {
    pub fn from_stderr() -> Self {
        HyeongWriteStack {
            inner: std::io::stderr(),
        }
    }
}

impl<W: Write> HyeongWriteStack<W> {
    fn output_nan(&mut self) -> io::Result<()> {
        write!(&mut self.inner, "너무 커엇...")
    }
    fn output_unicode<I: PartialOrd + ToPrimitive + FromPrimitive>(&mut self, value: I) -> io::Result<()> {
        let zero = I::from_isize(0).unwrap();
        assert!(value >= zero);
        let unicode_bound = I::from_isize(0x110000).unwrap();
        if value >= unicode_bound {
            self.output_nan()
        } else {
            let value = value.to_u32().and_then(|c| std::char::from_u32(c)).unwrap();
            write!(&mut self.inner, "{}", value)
        }
    }
}

impl<W: Write> HyeongStack for HyeongWriteStack<W> {
    fn push_one(&mut self, value: HyeongRational) {
        match value {
            HyeongRational::NaN => self.output_nan(),
            HyeongRational::Rational(r) => {
                let int = r.floor().to_integer();
                let zero = (0 as isize).into();
                if int >= zero {
                    self.output_unicode(int)
                } else {
                    write!(&mut self.inner, "{}", -int)
                }
            },
        };
    }

    fn pop_one(&mut self) -> HyeongRational { HyeongRational::NaN }
}


enum StackWrapper<'a> {
    Owned(Box<HyeongStack>),
    Borrowed(&'a mut HyeongStack),
}

impl<'a> StackWrapper<'a> {
    fn from_owned(stack: Box<HyeongStack>) -> Self {
        StackWrapper::Owned(stack)
    }
    fn from_ref_mut(stack: &'a mut HyeongStack) -> Self {
        StackWrapper::Borrowed(stack)
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
}


use std::collections::BTreeMap;
pub struct StackManager<'a> {
    stacks: BTreeMap<usize, StackWrapper<'a>>,
    selected: usize,
}

impl<'a> StackManager<'a> {
    fn from_stacks(stdin:  StackWrapper<'a>,
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
        };
        stack.select(3);
        stack
    }

    pub fn new() -> Self {
        let stdin = HyeongReadStack::new(std::io::stdin());
        let stdout = HyeongWriteStack::new(std::io::stdout());
        let stderr = HyeongWriteStack::new(std::io::stderr());
        StackManager::from_stacks(
            StackWrapper::from_owned(Box::new(stdin)),
            StackWrapper::from_owned(Box::new(stdout)),
            StackWrapper::from_owned(Box::new(stderr)))
    }

    pub fn push(&mut self, hangul: usize, dots: usize) {
        let value = Rational::from_integer(((hangul * dots) as isize).into());
        self.stacks.get_mut(&self.selected).unwrap().push_one(value.into());
    }

    pub fn add(&mut self, count: usize, to: usize) {
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

    pub fn duplicate(&mut self, count: usize, into: usize) {
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

    fn select(&mut self, id: usize) {
        self.make_stack(id);
        self.selected = id;
    }

    fn make_stack(&mut self, id: usize) {
        if self.stacks.contains_key(&id) { return; }
        let new_stack: Box<HyeongStack> = Box::new(vec![]);
        self.stacks.insert(id, StackWrapper::from_owned(new_stack));
    }
}


#[cfg(test)]
mod tests {
    use super::super::rational::{Rational, HyeongRational};

    #[test]
    fn read_stack_pop() {
        use super::{HyeongStack, HyeongReadStack};

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
        use super::{HyeongStack, HyeongWriteStack};

        let buf = {
            let buf = vec![];
            let mut stack = HyeongWriteStack::new(buf);
            stack.push_one(HyeongRational::from_u32('흑' as u32));
            stack.push_one(HyeongRational::from_u32('.' as u32));
            stack.push_one(HyeongRational::from_u32('.' as u32));
            stack.push_one(HyeongRational::from_u32('!' as u32));
            stack.push_one(Rational::from_integer((-32 as isize).into()).into());
            stack.push_one(HyeongRational::NaN);
            stack.push_one(Rational::new((65*3+2 as isize).into(), (3 as isize).into()).into());
            stack.push_one(Rational::new((-11 as isize).into(), (7 as isize).into()).into());
            stack.into_inner()
        };
        assert_eq!(&buf[..], "흑..!32너무 커엇...A2".as_bytes());
    }

    #[test]
    fn stack_manager_push_duplicate() {
        use super::{HyeongReadStack, HyeongWriteStack, StackWrapper, StackManager};

        let test_str = "";
        let mut buf = vec![];
        let mut buf_err = vec![];
        {
            let stdin =  HyeongReadStack::new(test_str.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut buf);
            let mut stderr = HyeongWriteStack::new(&mut buf_err);
            let stdin = StackWrapper::from_owned(Box::new(stdin));
            let stdout = StackWrapper::from_ref_mut(&mut stdout);
            let stderr = StackWrapper::from_ref_mut(&mut stderr);
            let mut manager = StackManager::from_stacks(stdin, stdout, stderr);

            manager.push(5, 13);     // 혀어어어엉.............
            manager.duplicate(3, 1); // 흐으윽.
        }
        assert_eq!(&buf[..], "AAA".as_bytes());
    }

    #[test]
    fn stack_manager_add_mul() {
        use super::{HyeongReadStack, HyeongWriteStack, StackWrapper, StackManager};

        let test_str = "A";
        let mut buf = vec![];
        let mut buf_err = vec![];
        {
            let stdin =  HyeongReadStack::new(test_str.as_bytes());
            let mut stdout = HyeongWriteStack::new(&mut buf);
            let mut stderr = HyeongWriteStack::new(&mut buf_err);
            let stdin = StackWrapper::from_owned(Box::new(stdin));
            let stdout = StackWrapper::from_ref_mut(&mut stdout);
            let stderr = StackWrapper::from_ref_mut(&mut stderr);
            let mut manager = StackManager::from_stacks(stdin, stdout, stderr);

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
}
