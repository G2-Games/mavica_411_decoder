use std::{fs::File, io::{BufReader, Read}};

use image::DynamicImage;

fn main() {
    let mut input_image = BufReader::new(File::open("MVC-659F.411").unwrap());

    let mut output_buffer = vec![];

    let mut input_buf = [0u8; 6];
    while input_image.read_exact(&mut input_buf).is_ok() {
        let cb = input_buf[4].saturating_sub(128) as f32;
        let cr = input_buf[5].saturating_sub(128) as f32;

        let luma_values = &input_buf[0..4];

        for y in luma_values {
            let y = *y as f32;

            let mut r = (y + 1.40200 * cr) as i32;
            let mut g = (y - 0.34414 * cb - 0.71414 * cr) as i32;
            let mut b = (y + 1.77200 * cb) as i32;

            if r < 0 { r = 0 }
            if g < 0 { g = 0 }
            if b < 0 { b = 0 }

            if r > 255 { r = 255 }
            if g > 255 { g = 255 }
            if b > 255 { b = 255 }

            output_buffer.extend_from_slice([r as u8, g as u8, b as u8].as_slice());
        }
    }

    dbg!(output_buffer.len());

    let out_image = image::RgbImage::from_raw(64, 48, output_buffer).unwrap();
    DynamicImage::from(out_image).save("test.png");
}
