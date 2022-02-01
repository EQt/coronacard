pub mod qrdecode;
pub mod qrencode;
pub mod svg;
pub mod vacc;

pub use qrencode::gen_qr_code;
pub use vacc::Vacc;

pub fn default_template() -> String {
    include_str!("../data/template.svg").into()
}


pub fn qr_detect(buf: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    qrdecode::decode_qr_image(&image::load_from_memory(buf)?)
}
