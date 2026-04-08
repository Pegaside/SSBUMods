#![feature(
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod roy;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    roy::install();
	smashline::clone_weapon("eflame", *smash::lib::lua_const::WEAPON_KIND_EFLAME_FIREPILLAR, "roy", "prevolt", true);
}