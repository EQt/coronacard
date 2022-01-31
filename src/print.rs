pub(crate) fn print_a4(card: &str) -> Result<String, Box<dyn std::error::Error>> {
    let inner = xmltree::Element::parse(card.as_bytes())?;
    let print_svg = include_str!("../data/print.svg");

    let mut xml = xmltree::Element::parse(print_svg.as_bytes())?;
    xml.children.iter_mut().for_each(|tag| match tag {
        xmltree::XMLNode::Element(img) => {
            if &img.name == "image" {
                img.attributes.remove("href");
                img.name = "svg".into();
                img.children = inner.children.clone();
            }
        }
        _ => (),
    });
    let mut out = Vec::new();
    xml.write(&mut out)?;
    Ok(String::from_utf8(out)?)
}
