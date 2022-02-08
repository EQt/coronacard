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

pub fn default_a4_template() -> String {
    include_str!("../data/print.svg").into()
}

pub enum Error {
    Vacc(vacc::VaccErr),
    GenQr(qrencode::QrEncErr),
    GenSvg(Box<dyn std::error::Error>),
    GenPdf(Box<dyn std::error::Error>),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vacc(arg0) => f.debug_tuple("Vacc").field(arg0).finish(),
            Self::GenQr(arg0) => f.debug_tuple("GenQr").field(arg0).finish(),
            Self::GenSvg(arg0) => f.debug_tuple("GenSvg").field(arg0).finish(),
            Self::GenPdf(arg0) => f.debug_tuple("GenPdf").field(arg0).finish(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Vacc(e) => Some(e),
            Error::GenQr(e) => Some(e),
            Error::GenSvg(e) => Some(e.as_ref()),
            Error::GenPdf(e) => Some(e.as_ref()),
        }
    }
}

pub fn svg_with_templ(code: &str, pdf: bool, templ: String) -> Result<Vec<u8>, Error> {
    let vac = Vacc::parse(code).map_err(|e| Error::Vacc(e))?;
    let qrsvg = gen_qr_code(code).map_err(|e| Error::GenQr(e))?;
    let templ = vac.to_svg(templ);
    let templ: String = svg::replace_rect(templ, &qrsvg).map_err(|e| Error::GenSvg(e))?;
    Ok(if pdf {
        svg::to_pdf(&templ).map_err(|e| Error::GenPdf(e))?
    } else {
        templ.into_bytes()
    })
}

pub fn svg_from_code(code: &str, din_a4: bool) -> Result<Vec<u8>, Error> {
    svg_with_templ(code, din_a4, default_template())
}
