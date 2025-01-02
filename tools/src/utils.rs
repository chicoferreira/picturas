use anyhow::Context;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::DynamicImage::ImageRgba8;
use photon_rs::PhotonImage;

pub fn load_from_base64(base64: &str) -> anyhow::Result<PhotonImage> {
    let img = STANDARD
        .decode(base64)
        .context("Couldn't decode base64 from image")?;
    let img = image::load_from_memory(&img).context("Couldn't load image from converted base64")?;
    let img = ImageRgba8(img.to_rgba8());
    let width = img.width();
    let height = img.height();
    let raw_pixels = img.into_bytes();

    Ok(PhotonImage::new(raw_pixels, width, height))
}
