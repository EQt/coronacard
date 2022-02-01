pub mod qrdecode;
pub mod qrencode;
pub mod svg;
pub mod vacc;

pub use qrencode::gen_qr_code;
pub use vacc::Vacc;

pub fn default_template() -> String {
    include_str!("../data/template.svg").into()
}
