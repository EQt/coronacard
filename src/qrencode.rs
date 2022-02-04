fn fix_svg_header(svg: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut xml = xmltree::Element::parse(svg.as_bytes())?;
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
    )?;
    Ok(String::from_utf8(out)?)
}

pub fn gen_qr_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    use qrcode::render::svg;
    use qrcode::QrCode;

    fix_svg_header(
        &QrCode::with_error_correction_level(code, qrcode::EcLevel::L)?
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build(),
    )
}
