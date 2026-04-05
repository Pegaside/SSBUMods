#![feature(
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod terryknee;

#[skyline::main(name = "smashline_test")]
pub fn main() {
	terryknee::install();
}