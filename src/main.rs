use std::{env, fs::File, io::{BufReader, Read, Write}, time::Instant};

use image::DynamicImage;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 48;
const LENGTH: usize = (WIDTH as usize * HEIGHT as usize) * 3;
const INPUT_LENGTH: usize = ((WIDTH as usize * HEIGHT as usize) as f32 * 1.5) as usize;

fn main() {
    let arguments = env::args().skip(1);

    for file in arguments {
        let buffer = decode_411(&file).unwrap();

        let out_image = image::RgbImage::from_raw(WIDTH, HEIGHT, buffer.to_vec()).unwrap();

        DynamicImage::from(out_image).save(file + ".png").unwrap();
    }
}

fn decode_411(filename: &str) -> Result<[u8; LENGTH], ()> {
    let mut input_image = BufReader::new(File::open(filename).unwrap());

    let mut output_buffer = [0u8; LENGTH];
    let mut output_writer = output_buffer.as_mut_slice();

    let mut input_buf = [0u8; 6];
    while input_image.read_exact(&mut input_buf).is_ok() {
        // Grab the Cr and Cb values
        let cb = (input_buf[4] as i32 - 128) as f32;
        let cr = (input_buf[5] as i32 - 128) as f32;

        // Loop over the luma values, transforming them as in Rec. 601
        //
        // It might be better to do this in fixed-point, but this is plenty fast
        // on modern systems.
        for y in &input_buf[0..4] {
            let y = *y as f32;

            let r = ((y + 1.40200 * cr) * 0.8) as u8;
            let g = (y - 0.34414 * cb - 0.71414 * cr) as u8;
            let b = (y + 1.77200 * cb) as u8;

            output_writer.write_all(&[r, g, b]).unwrap();
        }
    }

    Ok(output_buffer)
}
