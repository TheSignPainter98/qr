use std::path::{Path, PathBuf};

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::Parser;
use qrcode_generator::QrCodeEcc;

fn main() {
    let Args { url, size, output } = Args::parse();

    let output_path = output.as_deref().unwrap_or(Path::new("qr.png"));
    qrcode_generator::to_png_to_file(url, QrCodeEcc::High, size as usize, output_path).unwrap();
    if output.is_none() {
        eprintln!("qr code written to {}", output_path.display());
    }
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    styles = Self::styles(),
)]
struct Args {
    /// The text to encode
    url: String,

    #[arg(long, default_value_t = 2048)]
    size: u32,

    /// Output file.
    #[arg(short)]
    output: Option<PathBuf>,
}

impl Args {
    fn styles() -> Styles {
        // Match cargo output style
        Styles::styled()
            .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
            .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
            .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
            .placeholder(AnsiColor::Cyan.on_default())
            .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
            .valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
            .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
    }
}
