extern crate obj;
extern crate image;

use image::GenericImage;
use image::DynamicImage;

use std::fs::File;
use std::io::prelude::*;

fn line<T: GenericImage>(x1: u32, y1: u32, x2: u32, y2: u32, image: &mut T, pixel: &T::Pixel) {
	for x in x1..=x2 {
		let a = (y2-y1) as f32/(x2-x1) as f32;
		let y = a * x as f32;
		image.put_pixel(x, y as u32, *pixel);
	}
}

fn main() -> std::io::Result<()> {
	let mut image = DynamicImage::new_rgb8(100, 100);
	let color = image::Rgba {
		data: [255, 255, 255, 255]
	};
    line(10, 10, 80, 80, &mut image, &color);
	let file = File::create("im.png")?;
	image.save("truc.png");
	Ok(())
}
