mod Landing_AirB;
mod Landing_AirF;
mod Landing_AirHi;
mod Landing_AirLw;
mod Landing_AirN;

mod Landing_Light;
mod Landing_Heavy;

pub fn install() {
    Landing_AirN::install();
    Landing_AirF::install();
    Landing_AirB::install();
    Landing_AirHi::install();
    Landing_AirLw::install();

    Landing_Light::install();
    Landing_Heavy::install();
}
