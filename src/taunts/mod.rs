mod AppealHi;
mod AppealLw;
mod AppealS;


pub fn install() {
    AppealHi::install();
    AppealS::install();
    AppealLw::install();
}
