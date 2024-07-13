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

mod taunts;
mod tilts;
mod smash_attacks;
mod aerials;


#[skyline::main(name = "chrom_pairup")]
pub fn main() {

    taunts::install();
    tilts::install();
    smash_attacks::install();
    aerials::install();
   
}
