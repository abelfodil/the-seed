use image::RgbaImage;

pub type Pixel = [u8; 4];

pub trait ToPixel {
    fn to_pixel(&self) -> Pixel;
}

pub trait ToImage {
    fn to_image(&self) -> RgbaImage;
}
