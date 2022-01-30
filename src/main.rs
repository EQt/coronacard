fn gen_qr_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    use qrcode::render::svg;
    use qrcode::QrCode;

    let code = QrCode::new(code)?;
    Ok(code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
       .build())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = include_str!("../code");
    let cert = greenpass::parse(&code)?;
    let pass = cert.passes.last().ok_or("no greenpass found")?;
    let birth = &pass.date_of_birth;
    let name = format!("{}, {}", pass.surname, pass.givenname);
    let vac = pass.entries.last().ok_or("no vaccine entries found")?;
    match vac {
        greenpass::CertInfo::Recovery(_) => todo!(),
        greenpass::CertInfo::Test(_) => todo!(),
        greenpass::CertInfo::Vaccine(vac) => {
            let dose = format!("{}/{}", &vac.dose_number, vac.dose_total);
            dbg!(&birth, &name, &dose, &vac.date);
        }
    }
    std::fs::write("v.svg", gen_qr_code(code)?)?;
    Ok(())
}
