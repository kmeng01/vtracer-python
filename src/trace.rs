use image::{ImageBuffer, Rgba, RgbaImage};
use std::vec::Vec;
use visioncortex::ColorImage;
use vtracer::{convert_image_to_svg, convert_image_to_svg_in_mem, Config, Preset};

pub fn vtrace_fpath(input_fpath: &str, output_fpath: &str) -> Result<(), String> {
    let args = Config::from_preset(Preset::Bw, &input_fpath, &output_fpath);
    println!(
        "Running vtracer for input file {} -> output at {}",
        &input_fpath, &output_fpath
    );
    convert_image_to_svg(args)
}

pub fn vtrace_image_array(img_ar: Vec<Vec<Vec<u8>>>) -> Result<String, String> {
    let img = array_to_image(img_ar);
    let config = Config::from_preset(Preset::Photo, "foo", "bar");

    Ok(convert_image_to_svg_in_mem(config, img)?)
}

fn array_to_image(img_ar: Vec<Vec<Vec<u8>>>) -> ColorImage {
    let (w, h) = (img_ar[0].len() as u32, img_ar.len() as u32);

    let mut imgbuf: RgbaImage = ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let pxl = &img_ar[y as usize][x as usize];
        *pixel = Rgba([pxl[0], pxl[1], pxl[2], 255]);
    }

    ColorImage {
        pixels: imgbuf.as_raw().to_vec(),
        width: w as usize,
        height: h as usize,
    }
}
