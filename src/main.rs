fn gen_qr_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    use qrcode::render::svg;
    use qrcode::QrCode;

    let code = QrCode::new(code)?;
    let qr = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    {
        let tag_end = qr.find('>').ok_or("could not find end tag")?;
        let svg_header = &qr[..tag_end];
        if svg_header.contains("x=") {
            None.ok_or("x attribute in svg header")?;
        }
        if svg_header.contains("y=") {
            None.ok_or("y attribute in svg header")?;
        }
    }
    let qr = qr
        .strip_prefix(r#"<?xml version="1.0" standalone="yes"?>"#)
        .ok_or("expected prefix not found")?;
    let qr = regex::Regex::new(r#"height="\d+""#)?
        .replace(qr, r#"height="51mm" y="1mm""#);
    let qr = regex::Regex::new(r#"width="\d+""#)?
        .replace(&qr, r#"width="51mm" x="1mm""#);
    Ok(qr.to_string())
}

/// Certificate code to SVG converter.
#[derive(clap::Parser)]
#[clap(author, about)]
struct Args {
    /// Certificate code (input)
    #[clap(short, long, default_value = include_str!("../code"))]
    code: String,

    /// SVG template how to render the image.
    #[clap(short, long, default_value = "template.svg")]
    template: String,

    #[clap(short, long, default_value = "v.svg")]
    out: String,

    #[clap(short, long)]
    no_show: bool,
}

#[derive(Debug)]
struct Vacc {
    name: String,
    birth: String,
    lastvacc: String,
    dose: String,
}

impl Vacc {
    fn to_svg(&self, templ: &mut String) {
        *templ = templ.replace("@birth", &self.birth);
        *templ = templ.replace("@name", &self.name);
        *templ = templ.replace("@dose", &self.dose);
        *templ = templ.replace("@lastvacc", &self.lastvacc);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let args = Args::parse();
    let cert = greenpass::parse(&args.code)?;
    let pass = cert.passes.last().ok_or("no greenpass found")?;
    let birth = &pass.date_of_birth;
    let name = format!("{}, {}", pass.surname, pass.givenname);
    let vac = pass.entries.last().ok_or("no vaccine entries found")?;

    let vac = match vac {
        greenpass::CertInfo::Recovery(_) => todo!(),
        greenpass::CertInfo::Test(_) => todo!(),
        greenpass::CertInfo::Vaccine(vac) => Vacc {
            name,
            dose: format!("{}/{}", &vac.dose_number, vac.dose_total),
            lastvacc: format!("{}", &vac.date),
            birth: birth.to_owned(),
        },
    };
    dbg!(&vac);
    {
        let mut templ = std::fs::read_to_string(&args.template)?;
        vac.to_svg(&mut templ);
        let card = gen_qr_code(&args.code)?;
        templ = templ.replace("@inner", &card);
        std::fs::write(&args.out, &templ)?;
    }
    Ok(())
}
