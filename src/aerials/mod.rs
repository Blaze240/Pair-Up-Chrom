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

mod AttackAirB;
mod AttackAirF;
mod AttackAirHi;
mod AttackAirLw;
mod AttackAirN;

pub fn install() {
    //AttackAirN::install();
    //AttackAirF::install();
    //AttackAirB::install();
    AttackAirHi::install();
    //AttackAirLw::install();
}
