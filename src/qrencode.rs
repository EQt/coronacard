/// Set svg attributes.
/// - `x`
/// - `y`
/// - `width`
/// - `height`
fn fix_svg_header(svg: &str) -> Result<String, QrEncErr> {
    assert!(
        svg.contains("xlink:href"),
        "fix_svg_header: svg={}",
        &svg[..300]
    );
    let mut xml =
        xmltree::Element::parse(svg.as_bytes()).map_err(|e| QrEncErr::QrSvgParse(Box::new(e)))?;
    xml.attributes.insert("x".into(), "0mm".into());
    xml.attributes.insert("y".into(), "0mm".into());
    xml.attributes.insert("width".into(), "53mm".into());
    xml.attributes.insert("height".into(), "53mm".into());
    crate::svg::fix_href(&mut xml);
    let mut out = Vec::new();
    xml.write_with_config(
        &mut out,
        xmltree::EmitterConfig::new()
            .write_document_declaration(true)
            .perform_indent(true),
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
pub fn gen_qr_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let svg = qrenc::svg_from_qr(code)?;
    let svg = fix_svg_header(&svg)?;
    Ok(svg)
}
