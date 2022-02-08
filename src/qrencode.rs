/// Set svg attributes.
/// - `x`
/// - `y`
/// - `width`
/// - `height`
fn fix_svg_header(svg: &str) -> Result<String, QrEncErr> {
    let mut xml =
        xmltree::Element::parse(svg.as_bytes()).map_err(|e| QrEncErr::QrSvgParse(Box::new(e)))?;
    xml.attributes.insert("x".into(), "1mm".into());
    xml.attributes.insert("y".into(), "1mm".into());
    xml.attributes.insert("width".into(), "51mm".into());
    xml.attributes.insert("height".into(), "51mm".into());
    let mut out = Vec::new();
    xml.write_with_config(
        &mut out,
        xmltree::EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(false),
    )
    .map_err(|_| QrEncErr::XmlWriteErr)?;
    String::from_utf8(out).map_err(|_| QrEncErr::Utf8Err)
}

pub enum QrEncErr {
    Encode(String, Box<dyn std::error::Error>),
    QrSvgParse(Box<dyn std::error::Error>),
    XmlWriteErr,
    Utf8Err,
}

impl std::fmt::Debug for QrEncErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::InternalSVG => write!(f, "InternalSVG"),
            Self::Encode(c, e) => write!(f, "could not encode {c}: {e:?}"),
            Self::QrSvgParse(e) => write!(f, "generated qr code not valid xml (svg): {e:?}"),
            Self::Utf8Err => write!(f, "qr wirte: xml is not valid utf8"),
            Self::XmlWriteErr => write!(f, "could not write xml"),
        }
    }
}

impl std::fmt::Display for QrEncErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for QrEncErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QrEncErr::QrSvgParse(e) => Some(e.as_ref()),
            QrEncErr::Encode(_, e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

/// Generate a QR code as SVG.
pub fn gen_qr_code(code: &str) -> Result<String, QrEncErr> {
    use qrcode::render::svg;
    use qrcode::QrCode;

    let svg = QrCode::with_error_correction_level(code, qrcode::EcLevel::L)
        .map_err(|e| QrEncErr::Encode(code.into(), e.into()))?
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    fix_svg_header(&svg)
}
