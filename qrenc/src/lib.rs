pub fn png_from_qr(code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use qrcode_png::QrCode;

    let mut qr = QrCode::new(code, qrcode_png::QrCodeEcc::Low)?;
    qr.margin(5);
    let bytes = qr.generate(qrcode_png::Grayscale::default())?;
    Ok(bytes)
}

pub fn png_base64_from_qr(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let png = png_from_qr(code)?;
    Ok(base64::encode(png))
}

pub fn svg_from_png(png: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let data = base64::encode(png);
    let width = "51mm";
    let height = "51mm";
    let x = "0mm";
    let y = "0mm";
    let svg = include_str!("./base64.svg")
        .replace("{width}", width)
        .replace("{height}", height)
        .replace("{x}", x)
        .replace("{y}", y)
        .replace("{data}", &data);
    Ok(svg)
}

pub fn svg_from_qr(qr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let png = png_from_qr(qr)?;
    let svg = svg_from_png(&png)?;
    Ok(svg)
}
