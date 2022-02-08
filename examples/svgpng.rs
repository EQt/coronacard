fn png_from_qr(code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use qrcode_png::QrCode;

    let mut qr = QrCode::new(code, qrcode_png::QrCodeEcc::Low)?;
    qr.margin(5);
    let bytes = qr.generate(qrcode_png::Grayscale::default())?;
    Ok(bytes)
}

fn svg_from_png(svg: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let enc = base64::encode(svg);
    Ok(enc)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png = png_from_qr("BlubdiWupp")?;
    std::fs::write("b.png", &png)?;
    println!("{}", svg_from_png(&png)?);
    Ok(())
}
