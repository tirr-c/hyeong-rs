use std::cmp::Ordering;

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
