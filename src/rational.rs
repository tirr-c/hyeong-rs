use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use num_traits::{One, ToPrimitive, Zero};

#[cfg(feature = "big-rational")]
use num_rational::BigRational as Rational;
#[cfg(not(feature = "big-rational"))]
use num_rational::Rational64 as Rational;

#[derive(Clone, Debug)]
pub enum HyeongRational {
    Rational(Rational),
    NaN,
}

impl HyeongRational {
    #[cfg(feature = "big-rational")]
    pub fn new_i64(numer: i64, denom: i64) -> HyeongRational {
        let r = Rational::new(numer.into(), denom.into());
        HyeongRational::Rational(r)
    }
    #[cfg(not(feature = "big-rational"))]
    pub fn new_i64(numer: i64, denom: i64) -> HyeongRational {
        let r = Rational::new(numer, denom);
        HyeongRational::Rational(r)
    }
    #[cfg(feature = "big-rational")]
    pub fn from_i64(value: i64) -> HyeongRational {
        let r = Rational::from_integer(value.into());
        HyeongRational::Rational(r)
    }
    #[cfg(not(feature = "big-rational"))]
    pub fn from_i64(value: i64) -> HyeongRational {
        let r = Rational::from_integer(value);
        HyeongRational::Rational(r)
    }
    #[cfg(feature = "big-rational")]
    pub fn from_u64(value: u64) -> HyeongRational {
        let r = Rational::from_integer(value.into());
        HyeongRational::Rational(r)
    }
    #[cfg(not(feature = "big-rational"))]
    pub fn from_u64(value: u64) -> HyeongRational {
        let r = Rational::from_integer(value as i64);
        HyeongRational::Rational(r)
    }
    pub fn is_nan(&self) -> bool {
        matches!(self, HyeongRational::NaN)
    }
    pub fn rational(&self) -> &Rational {
        match self {
            HyeongRational::NaN => panic!("the value is NaN"),
            HyeongRational::Rational(r) => r,
        }
    }
    pub fn into_rational(self) -> Rational {
        match self {
            HyeongRational::NaN => panic!("the value is NaN"),
            HyeongRational::Rational(r) => r,
        }
    }
    pub fn recip(&self) -> HyeongRational {
        match self {
            HyeongRational::NaN => HyeongRational::NaN,
            HyeongRational::Rational(r) => {
                if r.is_zero() {
                    HyeongRational::NaN
                } else {
                    r.recip().into()
                }
            }
        }
    }
}

impl From<HyeongRational> for Option<Rational> {
    fn from(v: HyeongRational) -> Option<Rational> {
        match v {
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
        match self {
            HyeongRational::Rational(r) => {
                let int = r.floor().to_integer();
                let zero = Zero::zero();
                if int >= zero {
                    let unicode_bound = num_traits::FromPrimitive::from_i64(0x110000).unwrap();
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
        if let HyeongRational::Rational(r) = self {
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
    use super::HyeongRational;

    #[test]
    fn partial_eq() {
        let three = HyeongRational::from_u64(3);
        let five = HyeongRational::from_u64(5);
        let another_three = HyeongRational::from_u64(1 + 2);
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

        let three = HyeongRational::from_u64(3);
        let five = HyeongRational::from_u64(5);
        let another_three = HyeongRational::from_u64(1 + 2);
        let nan = HyeongRational::NaN;
        let another_nan = HyeongRational::NaN;

        assert_eq!(three.partial_cmp(&five), Some(Ordering::Less));
        assert_eq!(three.partial_cmp(&another_three), Some(Ordering::Equal));
        assert_eq!(five.partial_cmp(&nan), None);
        assert_eq!(nan.partial_cmp(&another_nan), None);
    }
    #[test]
    fn operators() {
        let mut half = HyeongRational::new_i64(1, 2);
        let one_third = HyeongRational::new_i64(1, 3);
        let five_sixth = HyeongRational::new_i64(5, 6);
        let one_sixth = HyeongRational::new_i64(1, 6);
        let minus_one_sixth = HyeongRational::new_i64(-1, 6);
        let nan = HyeongRational::NaN;

        assert_eq!(half.clone() + one_third.clone(), five_sixth);
        assert_eq!(half.clone() * one_third.clone(), one_sixth);
        assert_eq!(one_third.clone() + (-half.clone()), minus_one_sixth);
        assert!((five_sixth + nan.clone()).is_nan());
        assert!((nan.clone() * one_third.clone()).is_nan());
        assert!((nan.clone() * nan.clone()).is_nan());

        half += one_third;
        half *= one_sixth;
        half += minus_one_sixth;
        let mut answer = HyeongRational::new_i64(-1, 36);
        assert_eq!(half, answer);
        half += nan.clone();
        assert!(half.is_nan());
        answer *= nan;
        assert!(answer.is_nan());
    }
    #[test]
    fn recip() {
        let half = HyeongRational::new_i64(1, 2);
        let two = HyeongRational::from_u64(2);
        let zero = HyeongRational::from_u64(0);
        let nan = HyeongRational::NaN;

        assert_eq!(half.recip(), two);
        assert_eq!(two.recip(), half);
        assert!(zero.recip().is_nan());
        assert!(nan.recip().is_nan());
    }
}
