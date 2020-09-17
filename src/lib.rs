#![feature(const_fn)]
extern crate const_fn_assert;
use const_fn_assert::*;


/// #[constraint(self.0%2 != 0)]
/// type OddU32 = u32;

pub struct OddU32(pub u32);

impl OddU32 {
    fn constrained(self) -> Self {
        assert!(self.0%2 != 0);
        self
    }
    const fn const_constrained(self) -> Self {
        cfn_assert!(self.0%2 != 0);
        self
    }
    // should be put in a trait
    pub const fn const_from(x: u32) -> Self {
        Self(x).const_constrained()
    }
}
impl From<u32> for OddU32 {
    fn from(x: u32) -> Self {
        Self(x).constrained()
    }
}
impl Into<u32> for OddU32 {
    fn into(self) -> u32 {
        self.0
    }
}
