use std::{env, fs::File, io::{BufReader, Read, Write}};

use image::DynamicImage;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 48;
const LENGTH: usize = (WIDTH as usize * HEIGHT as usize) * 3;
const INPUT_LENGTH: usize = ((WIDTH as usize * HEIGHT as usize) as f32 * 1.5) as usize;

fn main() {
    let arguments = env::args().skip(1);

    for file in arguments {
        let mut input_image = BufReader::new(File::open(&file).unwrap());
        let buffer = decode_411(&mut input_image).unwrap();

        let out_image = image::RgbImage::from_raw(WIDTH, HEIGHT, buffer.to_vec()).unwrap();

        DynamicImage::from(out_image).save(file + ".png").unwrap();
    }
}

fn decode_411(mut reader: impl Read) -> Result<[u8; LENGTH], ()> {
    let mut output_buffer = [0u8; LENGTH];
    let mut output_writer = output_buffer.as_mut_slice();

    let mut input_buf = [0u8; 6];
    while reader.read_exact(&mut input_buf).is_ok() {
        // Grab the Cr and Cb values
        let cb = input_buf[4];
        let cr = input_buf[5];

        // Loop over the luma values, transforming them as in Rec. 601
        for y in &input_buf[0..4] {
            let (r, g, b) = {
                #[cfg(not(feature = "fixed"))]
                {
                    rec601_float(*y, cr, cb)
                }

                #[cfg(feature = "fixed")]
                {
                    rec601_fixed(*y, cr, cb)
                }
            };

            output_writer.write_all(&[r, g, b]).unwrap();
        }
    }

    Ok(output_buffer)
}

fn rec601_float(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let y = y as f32;
    let cb = cb as f32 - 128.0;
    let cr = cr as f32 - 128.0;

    let r = y + 1.40200 * cr;
    let g = y - 0.34414 * cb - 0.71414 * cr;
    let b = y + 1.77200 * cb;

    (r as u8, g as u8, b as u8)
}

fn rec601_fixed(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let y = y as i32 * 100_000;
    let cb = cb as i32 - 128;
    let cr = cr as i32 - 128;

    let r = (y + 140200 * cr) / 100_000;
    let g = (y - 034414 * cb - 071414 * cr) / 100_000;
    let b = (y + 177200 * cb) / 100_000;

    let r = r.clamp(0, 255) as u8;
    let g = g.clamp(0, 255) as u8;
    let b = b.clamp(0, 255) as u8;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    /// Test the decoding of a directory of .411 images
    #[test]
    fn test_decode() {
        for image in std::fs::read_dir("./test_images")
            .unwrap()
            .filter_map(|f| f.ok())
            .filter(|f|
                f.file_name()
                    .into_string()
                    .is_ok_and(|c| !c.contains(".bin"))
            )
        {
            let input_file = BufReader::new(File::open(image.path()).unwrap());
            let decoded_image = decode_411(input_file).unwrap();

            let mut compare_path = image.path();
            compare_path.set_extension("bin");
            let compare_image = fs::read(compare_path).unwrap();

            assert_eq!(*compare_image, decoded_image)
        }
    }

    /// Ensure Rec. 601 floating point decoding produces the right value
    #[test]
    fn test_rec601_float() {
        assert_eq!(rec601_float(120, 129, 51), (12,174,121));
    }

    /// Ensure Rec. 601 fixed point decoding produces the right value
    #[test]
    fn test_rec601_fixed() {
        assert_eq!(rec601_fixed(120, 129, 51), (12,174,121));
    }

    /// Ensure Rec. 601 floating point and fixed decoding both produce the same
    /// value
    #[test]
    fn test_rec601_fixed_float_equal() {
        assert_eq!(rec601_fixed(120, 129, 51), rec601_float(120, 129, 51));
    }
}
