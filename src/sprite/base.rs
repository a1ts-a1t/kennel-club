use std::{
    io::{BufWriter, Cursor},
    path::Path,
};

use image::{DynamicImage, ImageFormat, ImageReader, imageops::FilterType};

fn int_mult(float: f64, int: u32) -> u32 {
    (float * int as f64) as u32
}

#[derive(Debug, Clone)]
pub struct Sprite {
    image: DynamicImage,
    scale_factor: f64,
    bytes: Vec<u8>,
    format: ImageFormat,
}

impl Sprite {
    pub fn load(path: &Path) -> Self {
        let image = ImageReader::open(path)
            .unwrap_or_else(|_| panic!("Error opening sprite file: {:?}", path))
            .decode()
            .expect("Error decoding sprite");

        let format = ImageFormat::from_path(path).unwrap_or(ImageFormat::Png);

        let mut cached_buffer = BufWriter::new(Cursor::new(Vec::new()));
        image
            .write_to(&mut cached_buffer, format)
            .expect("Error caching sprite");

        let bytes = cached_buffer
            .into_inner()
            .expect("Error writing sprite buffer cache")
            .into_inner();

        let scale_factor = 1.0 / (u32::max(image.width(), image.height()) as f64);

        Sprite {
            image,
            bytes,
            scale_factor,
            format,
        }
    }

    pub fn get_scaled_image(&self, scale_factor: f64) -> DynamicImage {
        let (width, height) = (
            int_mult(scale_factor, self.image.width()),
            int_mult(scale_factor, self.image.height()),
        );
        self.image.resize(width, height, FilterType::Nearest)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn as_image(&self) -> &DynamicImage {
        &self.image
    }

    pub(crate) fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }
}
