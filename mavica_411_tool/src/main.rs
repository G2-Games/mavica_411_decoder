mod bitmap;

use std::{env, error::Error, fs::File, io::BufReader};

use bitmap::write_bitmap;
use mavica_411::decode_411;

fn main() -> Result<(), Box<dyn Error>> {
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

    for filename in arguments {
        let file = File::open(&filename)?;

        let mut input_image = BufReader::new(file);
        let buffer = decode_411(&mut input_image).unwrap();

        write_bitmap(&(filename + ".BMP"), &buffer);
    }

    Ok(())
}

