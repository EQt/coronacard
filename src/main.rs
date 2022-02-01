use coronacard::vacc::Vacc;
use std::path::PathBuf;

/// Certificate code to SVG converter.
#[derive(clap::Parser)]
#[clap(about)]
struct Args {
    /// Certificate code (input)
    #[clap(short, long)]
    code: Option<String>,

    #[clap(short, long)]
    image: Option<PathBuf>,

    /// SVG template how to render the image.
    #[clap(short, long)]
    template: Option<PathBuf>,

    #[clap(short, long, default_value = "card.svg")]
    out: PathBuf,

    #[clap(short, long)]
    din_a4: bool,
}

pub fn load_template<P>(path: Option<P>) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    Ok(path
        .as_ref()
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| Ok(coronacard::default_template()))?)
}

pub(crate) fn render_svg(
    args: &Args,
    vac: &Vacc,
    qr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let templ = vac.to_svg(load_template(args.template.as_ref())?, qr);
    Ok(std::fs::write(
        &args.out,
        if args.din_a4 {
            coronacard::svg::print_a4(&templ)?
        } else {
            templ
        },
    )?)
}

pub(crate) fn code_to_svg(code: &str, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let vac = coronacard::Vacc::parse(code)?;
    let qr = coronacard::gen_qr_code(code)?;
    eprint!("{vac:#?}");
    render_svg(args, &vac, &qr)?;
    eprintln!(" => {:?}", &args.out);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let args = Args::parse();
    let img_path = args.image.as_ref();
    let code = &args
        .code
        .as_ref()
        .map(|c| Ok(c.trim_start_matches("QR-Code:").to_string()))
        .or_else(|| img_path.map(coronacard::qrdecode::decode_qr))
        .ok_or("need --code or --image")??;
    code_to_svg(code, &args)
}
