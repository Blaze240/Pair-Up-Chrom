#![feature(concat_idents, proc_macro_hygiene)]
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

mod AttackHi4;
mod AttackLw4;
mod AttackS4;


pub fn install() {
    AttackS4::install();
    AttackHi4::install();
    AttackLw4::install();
}
