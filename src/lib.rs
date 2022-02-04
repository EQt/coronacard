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
    pdf: bool,
    templ: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let vac = Vacc::parse(code)?;
    let qrsvg = gen_qr_code(code)?;
    let templ = vac.to_svg(templ);
    let templ: String = svg::replace_rect(templ, &qrsvg)?;
    Ok(if pdf {
        svg::to_pdf(&templ)?
    } else {
        templ.into_bytes()
    })
}

pub fn svg_from_code(code: &str, din_a4: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    svg_with_templ(code, din_a4, default_template())
}
