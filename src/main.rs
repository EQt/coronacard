mod qrdecode;
mod qrencode;
mod svg;
mod vacc;
use std::path::PathBuf;
use vacc::Vacc;

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

pub(crate) fn render_svg(
    args: &Args,
    vac: &Vacc,
    qr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut templ = args
        .template.as_ref()
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| Ok(include_str!("../data/template.svg").into()))?;
    vac.to_svg(&mut templ, qr);
    if args.din_a4 {
        templ = crate::svg::print_a4(&templ)?;
    }
    Ok(std::fs::write(&args.out, &templ)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let args = Args::parse();
    let img_path = args.image.as_ref();
    let code = &args
        .code
        .as_ref()
        .map(|c| Ok(c.trim_start_matches("QR-Code:").to_string()))
        .or_else(|| img_path.map(crate::qrdecode::decode_qr))
        .ok_or("need --code or --image")??;
    let vac = crate::vacc::Vacc::parse(code)?;
    let qr = crate::qrencode::gen_qr_code(code)?;
    eprint!("{vac:#?}");
    render_svg(&args, &vac, &qr)?;
    eprintln!(" => {:?}", &args.out);
    Ok(())
}
