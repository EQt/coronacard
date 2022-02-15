fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::env::args()
        .last()
        .map(|a| a.to_string())
        .unwrap_or("png -> svg".into());
    let png = pdfimage::png_from_qr(&code)?;
    std::fs::write("qr.png", &png)?;
    let svg = pdfimage::svg_from_png(&png)?;
    println!("{}", svg);
    std::fs::write("qr.svg", &svg)?;
    Ok(())
}
