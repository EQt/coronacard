fn png_from_qr(_: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, 8, 11); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::One);
        let mut writer = encoder.write_header()?;
        let data: Vec<_> = [255; 11].iter().enumerate().map(|(i, p)| p >> i.min(7)).collect();
        writer.write_image_data(&data).unwrap(); // Save
    }
    Ok(buf)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = png_from_qr("pngwriter.rs")?;
    std::fs::write("image.png", buf)?;
    Ok(())
}
