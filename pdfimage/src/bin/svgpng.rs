fn png_from_qr(code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use qrcode_png::QrCode;

    let mut qr = QrCode::new(code, qrcode_png::QrCodeEcc::Low)?;
    qr.margin(5);
    let bytes = qr.generate(qrcode_png::Grayscale::default())?;
    Ok(bytes)
}

fn svg_from_png(svg: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let data = base64::encode(svg);
    let width = "51mm";
    let height = "51mm";
    let x = "0mm";
    let y = "0mm";
    let svg = include_str!("../base64.svg")
        .replace("{width}", width)
        .replace("{height}", height)
        .replace("{x}", x)
        .replace("{y}", y)
        .replace("{data}", &data);
    Ok(svg)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::env::args()
        .last()
        .map(|a| a.to_string())
        .unwrap_or("blubdiasdf asdfasdf asdf".into());
    let png = png_from_qr(&code)?;
    std::fs::write("b.png", &png)?;
    let svg = svg_from_png(&png)?;
    println!("{}", svg);
    std::fs::write("b.svg", &svg)?;
    Ok(())
}
