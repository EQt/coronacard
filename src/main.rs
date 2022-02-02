use std::path::PathBuf;

/// Certificate code to SVG converter.
#[derive(clap::Parser)]
#[clap(about)]
struct Args {
    #[clap(short, long)]
    image: Option<PathBuf>,

    /// Certificate code (input)
    #[clap(short, long)]
    code: Option<String>,

    /// SVG template how to render the image.
    #[clap(short, long)]
    template: Option<PathBuf>,

    #[clap(short, long, default_value = "card.svg")]
    out: PathBuf,

    #[clap(short, long)]
    din_a4: bool,

    #[clap(short, long)]
    pdf: bool,
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
    let vac = coronacard::Vacc::parse(code)?;
    let qr = coronacard::gen_qr_code(code)?;
    let svg_templ = args
        .template
        .as_ref()
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| Ok(coronacard::default_template()))?;
    eprint!("{vac:#?}");
    let templ = vac.to_svg(svg_templ, &qr);
    std::fs::write(
        &args.out,
        if args.din_a4 {
            coronacard::svg::print_a4(&templ)?
        } else {
            templ
        },
    )?;
    eprintln!(" => {:?}", &args.out);
    if cfg!(feature = "pdf") {
        if args.pdf {
            // use svg2pdf;
        }
    } else {
        if args.pdf {
            None.ok_or("For --pdf re-compile with feature \"pdf\" enabled!")?;
        }
    }
    Ok(())
}
