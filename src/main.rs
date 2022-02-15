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
        .or_else(|| img_path.map(coronacard::qrdecode::qr_from_path))
        .ok_or("need --code or --image")??;
    let vac = coronacard::Vacc::parse(code)?;
    let qr = coronacard::gen_qr_code(code)?;
    let svg_templ = args
        .template
        .as_ref()
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| Ok(coronacard::default_template()))?;
    eprint!("{vac:#?}");
    let templ = vac.to_svg(svg_templ);
    let templ: String = coronacard::svg::replace_rect_with_str(templ, &qr)?;
    std::fs::write(&args.out, &templ)?;
    eprintln!(" => {:?}", &args.out);
    if args.pdf {
        let basename: &str = args
            .out
            .as_path()
            .to_str()
            .ok_or("Path is not valid unicode")?
            .trim_end_matches(".svg");
        let pdf_out = format!("{basename}.pdf");
        eprintln!("  => {:?}", &pdf_out);
        let pdf = coronacard::svg::to_pdf(&templ)?;
        std::fs::write(pdf_out, pdf)?;
    }
    Ok(())
}
