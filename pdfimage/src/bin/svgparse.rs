fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "png -> svg";
    let png = pdfimage::png_from_qr(&code)?;
    let svg = pdfimage::svg_from_png(&png)?;
    let opts = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg, &opts.to_ref())?;
    for node in tree.root().children() {
        match *node.borrow() {
            usvg::NodeKind::Image(ref i) => {
                eprintln!("Found image {i:#?}");
                match &i.kind {
                    usvg::ImageKind::PNG(buf) => {
                        let buf = buf.as_ref();
                        assert_eq!(buf, &png);
                        // std::fs::write("image.png", p.as_ref())?;
                        let cursor = std::io::Cursor::new(buf);
                        let decoded =
                            image::io::Reader::with_format(cursor, image::ImageFormat::Png)
                                .decode()?;
                        // eprintln!("img = {decoded:?}");
                        let color = decoded.color();
                        dbg!(color.has_color());
                        dbg!(color.bits_per_pixel());
                        dbg!(color.channel_count());
                        let bits = color.bits_per_pixel();
                        let channels = color.channel_count() as u16;
                        dbg!(bits / channels > 8);
                        let img_rgb = decoded.to_rgb8();
                        let pixel = img_rgb.pixels().collect::<Vec<_>>();
                        dbg!(&pixel[251..255]);
                        let image_bytes: Vec<u8> = pixel.iter().flat_map(|&image::Rgb(c)| c).cloned().collect();
                        dbg!(&image_bytes[251*3 .. 255 * 3]);
                    }
                    k => eprintln!("ignoring image type {k:?}"),
                }
            }
            ref s => {
                eprintln!("ignoring {s:#?}");
            }
        }
    }
    Ok(())
}
