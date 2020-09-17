#![feature(const_fn)]
extern crate const_fn_assert;
extern crate constraint;
use constraint::*;

#[test]
fn odd_integer_valid() {
    OddU32::from(1);
}
#[test]
#[should_panic]
fn odd_integer_invalid() {
    OddU32::from(2);
}
#[test]
const fn const_odd_integer_valid() {
    OddU32::const_from(1);
}
#[test]
#[should_panic]
const fn const_odd_integer_invalid() {
    OddU32::const_from(2);
}
