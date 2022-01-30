
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = include_str!("../code");
    let cert = greenpass::parse(&code)?;
    // println!("{cert:?}");
    // let created = cert.created;
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
    Ok(())
}
