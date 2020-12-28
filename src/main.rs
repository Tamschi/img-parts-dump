#![doc(html_root_url = "https://docs.rs/img-parts-dump/0.0.1")]
#![warn(clippy::pedantic)]

use std::{error::Error, fs, path::PathBuf};

use img_parts::{jpeg::Jpeg, ImageEXIF, ImageICC};
use structopt::StructOpt;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(name = "FILE")]
	paths: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let opt = Opt::from_args();

	for path in opt.paths {
		println!("{:?}", &path);
		let file = fs::read(&path)?;
		let jpeg = Jpeg::from_bytes(file.into())?;

		for segment in jpeg.segments() {
			print!("{:x}, ", segment.marker());
			let mut out_path = path.as_os_str().to_os_string();
			out_path.push(format!(".{:x}", segment.marker()));
			fs::write(out_path, segment.contents())?
		}

		if let Some(exif) = jpeg.exif() {
			print!("exif, ");
			let mut exif_path = path.as_os_str().to_os_string();
			exif_path.push(".exif");
			fs::write(exif_path, exif)?
		}

		if let Some(icc_profile) = jpeg.icc_profile() {
			print!("icc_profile, ");
			let mut icc_profile_path = path.as_os_str().to_os_string();
			icc_profile_path.push(".icc_profile");
			fs::write(icc_profile_path, icc_profile)?
		}

		println!()
	}

	Ok(())
}
