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

mod AppealHi;
mod AppealLw;
mod AppealS;

mod AttackHi3;
mod AttackLw3;
mod AttackS3;

mod AttackLw4;
mod AttackS4;
mod AttackHi4;

mod AttackAirN;
mod AttackAirF;
mod AttackAirB;
mod AttackAirHi;
mod AttackAirLw;

#[skyline::main(name = "chrom_pairup")]
pub fn main() {
    AppealHi::install();
    AppealS::install();
    AppealLw::install();

    AttackS3::install();
    AttackHi3::install();
    AttackLw3::install();

    AttackS4::install();
    AttackHi4::install();
    AttackLw4::install();

    //AttackAirN::install();
    //AttackAirF::install();
    //AttackAirB::install();
    //AttackAirHi::install();
    //AttackAirLw::install();
}
