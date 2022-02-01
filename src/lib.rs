pub mod qrdecode;
pub mod qrencode;
pub mod svg;
pub mod vacc;

pub use qrencode::gen_qr_code;
pub use vacc::Vacc;

pub fn load_template<P>(path: Option<P>) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    Ok(path
        .as_ref()
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| Ok(include_str!("../data/template.svg").into()))?)
}

