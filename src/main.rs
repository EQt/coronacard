mod print;
mod qrdecode;
mod qrencode;
mod vacc;

/// Certificate code to SVG converter.
#[derive(clap::Parser)]
#[clap(about)]
struct Args {
    /// Certificate code (input)
    #[clap(short, long)]
    code: Option<String>,

    #[clap(short, long)]
    image: Option<String>,

    /// SVG template how to render the image.
    #[clap(short, long)]
    template: Option<String>,

    #[clap(short, long, default_value = "card.svg")]
    out: String,

    #[clap(short, long)]
    din_a4: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    let args = Args::parse();
    let img_path = args.image;
    let code = &args
        .code
        .as_ref()
        .map(|c| Ok(c.trim_start_matches("QR-Code:").to_string()))
        .or(img_path.map(|img| crate::qrdecode::decode_qr(img)))
        .ok_or("need --code or --image")??;
    let vac = crate::vacc::Vacc::parse(code)?;
    eprint!("{vac:#?}");
    {
        let mut templ = if let Some(t) = &args.template {
            std::fs::read_to_string(t)?
        } else {
            include_str!("../data/template.svg").to_string()
        };
        vac.to_svg(&mut templ, &crate::qrencode::gen_qr_code(code)?);
        if args.din_a4 {
            templ = crate::print::print_a4(&templ)?;
        }
        std::fs::write(&args.out, &templ)?;
        eprintln!(" => {}", &args.out);
    }
    Ok(())
}
