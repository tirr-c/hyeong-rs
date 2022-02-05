use num::{One, Zero};
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

#[cfg(feature = "big-rational")]
pub use num::rational::BigRational as Rational;
#[cfg(not(feature = "big-rational"))]
pub use num::rational::Rational;

#[derive(Clone, Debug)]
pub enum HyeongRational {
    Rational(Rational),
    NaN,
}

impl HyeongRational {
    #[cfg(feature = "big-rational")]
    pub fn from_u32(value: u32) -> HyeongRational {
        let r = Rational::from_integer(value.into());
        HyeongRational::Rational(r)
    }
    #[cfg(not(feature = "big-rational"))]
    pub fn from_u32(value: u32) -> HyeongRational {
        let r = Rational::from_integer(value as isize);
        HyeongRational::Rational(r)
    }
    #[cfg(feature = "big-rational")]
    pub fn from_usize(value: usize) -> HyeongRational {
        let r = Rational::from_integer(value.into());
        HyeongRational::Rational(r)
    }
    #[cfg(not(feature = "big-rational"))]
    pub fn from_usize(value: usize) -> HyeongRational {
        let r = Rational::from_integer(value as isize);
        HyeongRational::Rational(r)
    }
    pub fn is_nan(&self) -> bool {
        match *self {
            HyeongRational::NaN => true,
            _ => false,
        }
    }
    pub fn rational(&self) -> &Rational {
        match *self {
            HyeongRational::NaN => panic!("the value is NaN"),
            HyeongRational::Rational(ref r) => r,
        }
    }
    pub fn into_rational(self) -> Rational {
        match self {
            HyeongRational::NaN => panic!("the value is NaN"),
            HyeongRational::Rational(r) => r,
        }
    }
    pub fn recip(&self) -> HyeongRational {
        match *self {
            HyeongRational::NaN => HyeongRational::NaN,
            HyeongRational::Rational(ref r) => {
                if r.is_zero() {
                    HyeongRational::NaN
                } else {
                    r.recip().into()
                }
            }
        }
    }
}

impl Into<Option<Rational>> for HyeongRational {
    fn into(self) -> Option<Rational> {
        match self {
            HyeongRational::NaN => None,
            HyeongRational::Rational(r) => Some(r),
        }
    }
}

impl From<Rational> for HyeongRational {
    fn from(item: Rational) -> HyeongRational {
        HyeongRational::Rational(item)
    }
}

impl From<Option<Rational>> for HyeongRational {
    fn from(item: Option<Rational>) -> HyeongRational {
        match item {
            None => HyeongRational::NaN,
            Some(r) => HyeongRational::Rational(r),
        }
    }
}

impl From<Option<HyeongRational>> for HyeongRational {
    fn from(item: Option<HyeongRational>) -> HyeongRational {
        match item {
            None => HyeongRational::NaN,
            Some(r) => r,
        }
    }
}

impl Display for HyeongRational {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use num::traits::cast::ToPrimitive;
        match *self {
            HyeongRational::Rational(ref r) => {
                let int = r.floor().to_integer();
                let zero = 0isize.into();
                if int >= zero {
                    let unicode_bound = 0x110000isize.into();
                    if int >= unicode_bound {
                        write!(f, "너무 커엇...")
                    } else {
                        let int = int.to_u32().and_then(::std::char::from_u32).unwrap();
                        write!(f, "{}", int)
                    }
                } else {
                    write!(f, "{}", -int)
                }
            }
            HyeongRational::NaN => {
                write!(f, "너무 커엇...")
            }
        }
    }
}

impl PartialEq for HyeongRational {
    fn eq(&self, other: &HyeongRational) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }
        self.rational() == other.rational()
    }
}

impl PartialOrd for HyeongRational {
    fn partial_cmp(&self, other: &HyeongRational) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        self.rational().partial_cmp(other.rational())
    }
}

impl Add for HyeongRational {
    type Output = Self;
    fn add(self, rhs: HyeongRational) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return HyeongRational::NaN;
        }
        (self.into_rational() + rhs.into_rational()).into()
    }
}

impl AddAssign for HyeongRational {
    fn add_assign(&mut self, rhs: HyeongRational) {
        let result = self.clone() + rhs;
        self.clone_from(&result);
    }
}

impl Mul for HyeongRational {
    type Output = Self;
    fn mul(self, rhs: HyeongRational) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return HyeongRational::NaN;
        }
        (self.into_rational() * rhs.into_rational()).into()
    }
}

impl MulAssign for HyeongRational {
    fn mul_assign(&mut self, rhs: HyeongRational) {
        let result = self.clone() * rhs;
        self.clone_from(&result);
    }
}

impl Neg for HyeongRational {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            HyeongRational::NaN => HyeongRational::NaN,
            HyeongRational::Rational(r) => (-r).into(),
        }
    }
}

impl Zero for HyeongRational {
    fn zero() -> Self {
        HyeongRational::Rational(Rational::zero())
    }
    fn is_zero(&self) -> bool {
        if let HyeongRational::Rational(ref r) = *self {
            r.is_zero()
        } else {
            false
        }
    }
}

impl One for HyeongRational {
    fn one() -> Self {
        HyeongRational::Rational(Rational::one())
    }
}

#[cfg(test)]
mod tests {
    use super::{HyeongRational, Rational};

    #[test]
    fn partial_eq() {
        let three = HyeongRational::from_u32(3);
        let five = HyeongRational::from_u32(5);
        let another_three = HyeongRational::from_u32(1 + 2);
        let nan = HyeongRational::NaN;
        let another_nan = HyeongRational::NaN;

        assert_eq!(three, another_three);
        assert_ne!(three, five);
        assert_ne!(five, nan);
        assert_ne!(nan, another_nan); // NaN != NaN
    }
    #[test]
    fn partial_ord() {
        use std::cmp::Ordering;

        let three = HyeongRational::from_u32(3);
        let five = HyeongRational::from_u32(5);
        let another_three = HyeongRational::from_u32(1 + 2);
        let nan = HyeongRational::NaN;
        let another_nan = HyeongRational::NaN;

        assert_eq!(three.partial_cmp(&five), Some(Ordering::Less));
        assert_eq!(three.partial_cmp(&another_three), Some(Ordering::Equal));
        assert_eq!(five.partial_cmp(&nan), None);
        assert_eq!(nan.partial_cmp(&another_nan), None);
    }
    #[test]
    fn operators() {
        let mut half: HyeongRational = Rational::new(1isize.into(), 2isize.into()).into();
        let one_third: HyeongRational = Rational::new(1isize.into(), 3isize.into()).into();
        let five_sixth: HyeongRational = Rational::new(5isize.into(), 6isize.into()).into();
        let one_sixth: HyeongRational = Rational::new(1isize.into(), 6isize.into()).into();
        let minus_one_sixth: HyeongRational = Rational::new((-1isize).into(), 6isize.into()).into();
        let nan = HyeongRational::NaN;

        assert_eq!(half.clone() + one_third.clone(), five_sixth.clone());
        assert_eq!(half.clone() * one_third.clone(), one_sixth.clone());
        assert_eq!(one_third.clone() + (-half.clone()), minus_one_sixth.clone());
        assert!((five_sixth.clone() + nan.clone()).is_nan());
        assert!((nan.clone() * one_third.clone()).is_nan());
        assert!((nan.clone() * nan.clone()).is_nan());

        half += one_third;
        half *= one_sixth;
        half += minus_one_sixth;
        let mut answer: HyeongRational = Rational::new((-1isize).into(), 36isize.into()).into();
        assert_eq!(half, answer);
        half += nan.clone();
        assert!(half.is_nan());
        answer *= nan.clone();
        assert!(answer.is_nan());
    }
    #[test]
    fn recip() {
        let half: HyeongRational = Rational::new(1isize.into(), 2isize.into()).into();
        let two = HyeongRational::from_u32(2);
        let zero = HyeongRational::from_u32(0);
        let nan = HyeongRational::NaN;

        assert_eq!(half.recip(), two);
        assert_eq!(two.recip(), half);
        assert!(zero.recip().is_nan());
        assert!(nan.recip().is_nan());
    }
}
