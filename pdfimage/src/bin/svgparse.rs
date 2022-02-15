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
                        dbg!(decoded);
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
