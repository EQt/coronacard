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
        db.load_font_data(arial::regular_ttf());
        db.load_font_data(arial::bold_ttf());
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

fn replace_rect_elem(e: &mut xmltree::Element, xqr: &xmltree::Element) {
    if &e.name == "rect" {
        if Some("@qr") == e.attributes.get("id").map(|i| &**i) {
            e.attributes.remove("id");
            e.name = "svg".into();
            e.attributes.remove("fill");
            if let Some(vb) = xqr.attributes.get("viewBox") {
                e.attributes.insert("viewBox".into(), vb.into());
            }
            e.children = xqr.children.clone();
        }
    } else {
        replace_rect_rec(e, xqr);
    }
}

fn replace_rect_rec(xml: &mut xmltree::Element, xqr: &xmltree::Element) {
    xml.children.iter_mut().for_each(|xml| {
        if let Some(e) = xml.as_mut_element() {
            replace_rect_elem(e, xqr);
        }
    });
}

pub fn replace_rect(templ: String, qr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let xqr = xmltree::Element::parse(qr.as_bytes())?;
    let mut xml = xmltree::Element::parse(templ.as_bytes())?;
    replace_rect_rec(&mut xml, &xqr);
    let mut out = Vec::new();
    xml.write(&mut out)?;
    Ok(String::from_utf8(out)?)
}
