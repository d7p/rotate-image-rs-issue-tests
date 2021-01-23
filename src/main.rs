extern crate image;
extern crate photon_rs;

use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;

use image::{DynamicImage, ImageBuffer};

fn main() {
  let photo_image = open_image("testImages/elephants.jpg");
  let rotated_image = rotate(&photo_image);
  save_image(rotated_image, "result/resultImage.jpg");
  println!("done tests")
}

fn rotate(src: &PhotonImage) -> PhotonImage {
  let w1 = src.get_width();
  let h1 = src.get_height();
  let src_pixels = src.get_raw_pixels();
  let img_buffer = ImageBuffer::from_raw(w1, h1, src_pixels).unwrap();
  let rgba_img: DynamicImage = DynamicImage::ImageRgba8(img_buffer);

  //let raw_pixels = rgba_img.to_rgba8().to_vec();
  let raw_pixels = rgba_img.rotate90().to_rgba8().to_vec();
  let res = PhotonImage::new(raw_pixels, w1, h1);

  res
}
