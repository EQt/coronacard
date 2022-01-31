fn fix_svg_header(qr: String) -> Result<String, Box<dyn std::error::Error>> {
    {
        let tag_end = qr.find('>').ok_or("could not find end tag")?;
        let svg_header = &qr[..tag_end];
        if svg_header.contains("x=") {
            None.ok_or("x attribute in svg header")?;
        }
        if svg_header.contains("y=") {
            None.ok_or("y attribute in svg header")?;
        }
    }
    let qr = qr
        .strip_prefix(r#"<?xml version="1.0" standalone="yes"?>"#)
        .ok_or("expected prefix not found")?;
    Ok(qr
        .replacen(r#"height="255""#, r#"height="51mm" y="1mm""#, 1)
        .replacen(r#"width="255""#, r#"width="51mm" x="1mm""#, 1))
}

pub(crate) fn gen_qr_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    use qrcode::render::svg;
    use qrcode::QrCode;

    fix_svg_header(
        QrCode::new(code)?
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build(),
    )
}
