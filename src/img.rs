use image::RgbaImage;

pub trait ToPixel {
    fn to_pixel(&self) -> Vec<u8>;
}

pub trait ToImage {
    fn to_image(&self) -> RgbaImage;
}
