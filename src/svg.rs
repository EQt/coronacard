pub fn print_a4(card: &str) -> Result<String, Box<dyn std::error::Error>> {
    let inner = xmltree::Element::parse(card.as_bytes())?;
    let mut xml = xmltree::Element::parse(&include_bytes!("../data/print.svg")[..])?;
    xml.children.iter_mut().for_each(|tag| {
        if let xmltree::XMLNode::Element(img) = tag {
            if &img.name == "image" {
                img.attributes.remove("href");
                img.name = "svg".into();
                img.children = inner.children.clone();
            }
        }
    });
    let mut out = Vec::new();
    xml.write(&mut out)?;
    Ok(String::from_utf8(out)?)
}

#[cfg(feature = "pdf")]
pub fn to_pdf(svg: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let fontdb = {
        let mut db = fontdb::Database::new();
        db.load_font_data(
            include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial_Bold.ttf")[..].into(),
        );
        db.load_font_data(
            include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial.ttf")[..].into(),
        );
        assert_eq!(db.len(), 2);
        db
    };
    let options = usvg::Options {
        fontdb,
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(svg, &options.to_ref())?;
    let pdf = svg2pdf::convert_tree(&tree, svg2pdf::Options::default());
    Ok(pdf)
}

#[cfg(not(feature = "pdf"))]
pub fn to_pdf(_svg: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    None.ok_or("For --pdf re-compile with feature \"pdf\" enabled!")?;
    Ok(vec![])
}

pub fn replace_rect(templ: String, qr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let xqr = xmltree::Element::parse(qr.as_bytes())?;
    let mut xml = xmltree::Element::parse(templ.as_bytes())?;
    xml.children.iter_mut().for_each(|tag| {
        if let xmltree::XMLNode::Element(rect) = tag {
            if &rect.name == "rect" {
                if Some("@qr") == rect.attributes.get("id").map(|i| &**i) {
                    rect.attributes.remove("id");
                    rect.name = "svg".into();
                    rect.attributes.remove("fill");
                    if let Some(vb) = xqr.attributes.get("viewBox") {
                        rect.attributes.insert("viewBox".into(), vb.into());
                    }
                    rect.children = xqr.children.clone();
                }
            }
        }
    });
    let mut out = Vec::new();
    xml.write(&mut out)?;
    Ok(String::from_utf8(out)?)
}
