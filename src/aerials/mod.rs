mod AttackAirB;
mod AttackAirF;
mod AttackAirHi;
mod AttackAirLw;
mod AttackAirN;

pub fn install() {
    AttackAirN::install();
    AttackAirF::install();
    AttackAirB::install();
    AttackAirHi::install();
    AttackAirLw::install();
}
