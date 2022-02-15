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

pub(crate) fn fix_href(xml: &mut xmltree::Element) {
    if let Some(img) = xml.get_mut_child("image") {
        if let Some(href) = img
            .attributes
            .remove("href") {
                img.attributes.insert("xlink:href".into(), href);
            }
        dbg!(img.attributes.keys());
    }
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
            fix_href(e)
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

pub fn replace_rect_with_str(
    templ: String,
    qr: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let xqr = xmltree::Element::parse(qr.as_bytes())?;
    replace_rect_with_xml(templ, xqr)
}

pub fn replace_rect_with_xml(
    templ: String,
    xqr: xmltree::Element,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut xml = xmltree::Element::parse(templ.as_bytes())?;
    replace_rect_rec(&mut xml, &xqr);
    let mut out = Vec::new();
    xml.write_with_config(
        &mut out,
        xmltree::EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(true),
    )?;
    Ok(String::from_utf8(out)?)
}
