#[derive(Debug)]
pub struct Vacc {
    name: String,
    birth: String,
    last: String,
    dose: String,
}

impl Vacc {
    pub fn to_svg(&self, mut templ: String, inner: &str) -> String {
        templ = templ.replace("@birth", &self.birth);
        templ = templ.replace("@name", &self.name);
        templ = templ.replace("@dose", &self.dose);
        templ = templ.replace("@lastvacc", &self.last);
        templ = templ.replace("<text>@inner</text>", inner);
        templ
    }

    pub fn parse(code: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cert = greenpass::parse(code)?;
        let pass = cert.passes.last().ok_or("no greenpass found")?;
        let birth = &pass.date_of_birth;
        let name = format!("{}, {}", pass.surname, pass.givenname);
        match pass.entries.last().ok_or("no vaccine entries found")? {
            greenpass::CertInfo::Recovery(_) => todo!(),
            greenpass::CertInfo::Test(_) => todo!(),
            greenpass::CertInfo::Vaccine(vac) => Ok(Self {
                name,
                dose: format!("{}/{}", &vac.dose_number, vac.dose_total),
                last: format!("{}", &vac.date),
                birth: birth.to_owned(),
            }),
        }
    }
}
