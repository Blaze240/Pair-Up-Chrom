mod AttackHi3;
mod AttackLw3;
mod AttackS3;
mod AttackDash;


pub fn install() {
    AttackS3::install();
    AttackHi3::install();
    AttackLw3::install();
    AttackDash::install();
}
