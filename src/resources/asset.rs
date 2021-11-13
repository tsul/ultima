use crate::resources::MulLookup;
use image::RgbaImage;

pub trait Asset
where
    Self: std::marker::Sized,
{
    fn load(data: Vec<u8>, lookup: impl MulLookup) -> Option<Self>;
}

pub trait ImageAsset: Asset {
    fn get_image(&self) -> &RgbaImage;
}
