use clap::Parser;
use image::io::Reader;
use std::path::PathBuf;
use zxing_cpp::*;

#[derive(Parser)]
struct Cli {
	filename: PathBuf,
	formats: Option<String>,
	fast: bool,
}

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	let formats = barcode_formats_from_string(cli.formats.unwrap_or_default())?;
	let opts = ReaderOptions::new()
		.formats(formats)
		.try_harder(!cli.fast)
		.try_invert(!cli.fast)
		.try_rotate(!cli.fast);
	let image = Reader::open(&cli.filename)?.decode()?.into_luma8();
	let iv = ImageView::from(&image);

	let results = read_barcodes(&iv, &opts)?;

	if results.is_empty() {
		println!("No barcode found.");
	} else {
		for result in results {
			println!("Text: {}", result.text());
			println!("Bytes: {:?}", result.bytes());
			println!("Format: {}", result.format());
			println!("Content: {}", result.content_type());
			println!("Identifier: {}", result.symbology_identifier());
			println!("EC Level: {}", result.ec_level());
			println!("Error: {}", result.error_message());
			println!("Orientation: {}", result.orientation());
			println!();
		}
	}

	Ok(())
}
