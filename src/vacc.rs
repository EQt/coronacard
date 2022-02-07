#[derive(Debug)]
pub struct Vacc {
    name: String,
    birth: String,
    last: String,
    dose: String,
}

pub enum VaccErr {
    InvalidCode(String),
    NoGreenPass,
    NoCertificate,
}

impl std::fmt::Display for VaccErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Debug for VaccErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCode(c) => write!(f, "could not decode {c}"),
            Self::NoGreenPass => write!(f, "no greenpass found"),
            Self::NoCertificate => write!(f, "no valid certificate found"),
        }
    }
}

impl std::error::Error for VaccErr {}

impl Vacc {
    pub fn to_svg(&self, mut templ: String) -> String {
        templ = templ.replace("@birth", &self.birth);
        templ = templ.replace("@name", &self.name);
        templ = templ.replace("@dose", &self.dose);
        templ = templ.replace("@lastvacc", &self.last);
        templ
    }

    pub fn parse(code: &str) -> Result<Self, VaccErr> {
        let cert = greenpass::parse(code).map_err(|_| VaccErr::InvalidCode(code.into()))?;
        let pass = cert.passes.last().ok_or(VaccErr::NoGreenPass)?;
        let birth = &pass.date_of_birth;
        let name = format!("{}, {}", pass.surname, pass.givenname);
        match pass.entries.last().ok_or(VaccErr::NoCertificate)? {
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
