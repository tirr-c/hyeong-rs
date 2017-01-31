use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg};
use num::Zero;

#[cfg(feature = "big-rational")]
pub use num::rational::BigRational as Rational;
#[cfg(not(feature = "big-rational"))]
pub use num::rational::Rational as Rational;


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
    pub fn is_nan(&self) -> bool {
        match self {
            &HyeongRational::NaN => true,
            _ => false,
        }
    }
    pub fn rational(&self) -> &Rational {
        match self {
            &HyeongRational::NaN => panic!("the value is NaN"),
            &HyeongRational::Rational(ref r) => r,
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
            &HyeongRational::NaN => HyeongRational::NaN,
            &HyeongRational::Rational(ref r) => {
                if r.is_zero() { HyeongRational::NaN }
                else { r.recip().into() }
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

impl PartialEq for HyeongRational {
    fn eq(&self, other: &HyeongRational) -> bool {
        if self.is_nan() || other.is_nan() { return false; }
        self.rational() == other.rational()
    }
}

impl PartialOrd for HyeongRational {
    fn partial_cmp(&self, other: &HyeongRational) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() { return None; }
        self.rational().partial_cmp(other.rational())
    }
}

impl Add for HyeongRational {
    type Output = Self;
    fn add(self, rhs: HyeongRational) -> Self::Output {
        if self.is_nan() || rhs.is_nan() { return HyeongRational::NaN; }
        (self.into_rational() + rhs.into_rational()).into()
    }
}

impl Mul for HyeongRational {
    type Output = Self;
    fn mul(self, rhs: HyeongRational) -> Self::Output {
        if self.is_nan() || rhs.is_nan() { return HyeongRational::NaN; }
        (self.into_rational() * rhs.into_rational()).into()
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


#[cfg(test)]
mod tests {
    use super::{Rational, HyeongRational};

    #[test]
    fn partial_eq() {
        let three = HyeongRational::from_u32(3);
        let five = HyeongRational::from_u32(5);
        let another_three = HyeongRational::from_u32(1+2);
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
        let another_three = HyeongRational::from_u32(1+2);
        let nan = HyeongRational::NaN;
        let another_nan = HyeongRational::NaN;

        assert_eq!(three.partial_cmp(&five), Some(Ordering::Less));
        assert_eq!(three.partial_cmp(&another_three), Some(Ordering::Equal));
        assert_eq!(five.partial_cmp(&nan), None);
        assert_eq!(nan.partial_cmp(&another_nan), None);
    }
    #[test]
    fn operators() {
        let half: HyeongRational = Rational::new((1 as isize).into(), (2 as isize).into()).into();
        let one_third: HyeongRational = Rational::new((1 as isize).into(), (3 as isize).into()).into();
        let five_sixth: HyeongRational = Rational::new((5 as isize).into(), (6 as isize).into()).into();
        let one_sixth: HyeongRational = Rational::new((1 as isize).into(), (6 as isize).into()).into();
        let minus_one_sixth: HyeongRational = Rational::new((-1 as isize).into(), (6 as isize).into()).into();
        let nan = HyeongRational::NaN;

        assert_eq!(half.clone() + one_third.clone(), five_sixth.clone());
        assert_eq!(half.clone() * one_third.clone(), one_sixth.clone());
        assert_eq!(one_third.clone() + (-half.clone()), minus_one_sixth.clone());
        assert!((five_sixth.clone() + nan.clone()).is_nan());
        assert!((nan.clone() * one_third.clone()).is_nan());
        assert!((nan.clone() * nan.clone()).is_nan());
    }
}
