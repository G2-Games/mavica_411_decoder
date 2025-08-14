use std::{env, fs::File, io::BufReader};

use image::DynamicImage;
use mavica_411::decode_411;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.contains(&"--help".to_string()) || arguments.contains(&"-h".to_string()) {
        eprintln!("{}", env!("CARGO_PKG_DESCRIPTION"));
        eprintln!("\nUsage: {} [411 FILE]", env!("CARGO_CRATE_NAME"));
        eprintln!("\nArguments:");
        eprintln!("  -h, --help     Print this help");
        eprintln!("  -V, --version  Print version information");
        std::process::exit(0);
    } else if arguments.contains(&"--version".to_string()) || arguments.contains(&"-V".to_string()) {
        eprintln!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    for file in arguments {
        let mut input_image = BufReader::new(File::open(&file).unwrap());
        let buffer = decode_411(&mut input_image).unwrap();

        let out_image = image::RgbImage::from_raw(mavica_411::WIDTH, mavica_411::HEIGHT, buffer.to_vec()).unwrap();

        DynamicImage::from(out_image).save(file + ".png").unwrap();
    }
}

