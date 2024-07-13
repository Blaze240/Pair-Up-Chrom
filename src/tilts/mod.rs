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

mod AttackHi3;
mod AttackLw3;
mod AttackS3;


pub fn install() {
    AttackS3::install();
    AttackHi3::install();
    AttackLw3::install();
}
