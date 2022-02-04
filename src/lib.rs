pub mod qrdecode;
pub mod qrencode;
pub mod svg;
pub mod vacc;

pub use qrdecode::qr_from_img;
pub use qrencode::gen_qr_code;
pub use vacc::Vacc;

pub fn default_template() -> String {
    include_str!("../data/template.svg").into()
}

pub fn svg_with_templ(
    code: &str,
    din_a4: bool,
    templ: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let vac = Vacc::parse(code)?;
    let _qrsvg = gen_qr_code(code)?;
    let templ = vac.to_svg(templ);
    Ok(if din_a4 {
        svg::print_a4(&templ)?
    } else {
        templ
    })
}

pub fn svg_from_code(code: &str, din_a4: bool) -> Result<String, Box<dyn std::error::Error>> {
    svg_with_templ(code, din_a4, default_template())
}
