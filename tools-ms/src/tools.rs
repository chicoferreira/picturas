use crate::tools::ToolApplyResult::{Image, Text};
use anyhow::Context;
use image::DynamicImage::ImageRgba8;
use photon_rs::transform::SamplingFilter;
use photon_rs::{helpers, PhotonImage, Rgba};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "procedure", content = "parameters", rename_all = "camelCase")]
pub enum Tool {
    Crop { start: (u32, u32), end: (u32, u32) },
    Scale { x: u32, y: u32 },
    AddBorder { size: u32, color: (u8, u8, u8) },
    AdjustBrightness { value: f32 },
    AdjustContrast { value: f32 },
    Rotate { angle: f32 },
    Blur { radius: i32 },
    Ocr,
}

pub enum ToolApplyResult {
    Image(PhotonImage),
    Text(String),
}

impl Tool {
    pub fn apply(&self, image: PhotonImage) -> anyhow::Result<ToolApplyResult> {
        Ok(match self {
            Tool::Crop { start, end } => Image(crop(image, start.0, end.0, start.1, end.1)),
            Tool::Scale { x, y } => Image(scale(image, *x, *y)),
            Tool::AddBorder { size, color } => Image(add_border(image, *size, *color)),
            Tool::AdjustBrightness { value } => Image(adjust_brightness(image, *value)),
            Tool::AdjustContrast { value } => Image(adjust_contrast(image, *value)),
            Tool::Rotate { angle } => Image(rotate_image(image, *angle)),
            Tool::Blur { radius } => Image(blur_image(image, *radius)),
            Tool::Ocr => Text(ocr(image)?),
        })
    }
}

fn crop(image: PhotonImage, x1: u32, x2: u32, y1: u32, y2: u32) -> PhotonImage {
    photon_rs::transform::crop(&image, x1, x2, y1, y2)
}

fn scale(image: PhotonImage, x: u32, y: u32) -> PhotonImage {
    photon_rs::transform::resize(&image, x, y, SamplingFilter::Lanczos3)
}

fn add_border(image: PhotonImage, border_size: u32, color: (u8, u8, u8)) -> PhotonImage {
    photon_rs::transform::padding_uniform(
        &image,
        border_size,
        Rgba::new(color.0, color.1, color.2, 255),
    )
}

fn adjust_brightness(mut image: PhotonImage, value: f32) -> PhotonImage {
    let value = value.clamp(-1f32, 1f32);

    if value > 0.0 {
        photon_rs::effects::inc_brightness(&mut image, (value * 255.0) as u8);
        println!("Increased brightness by {}", value * 255.0);
    } else {
        photon_rs::effects::dec_brightness(&mut image, (-value * 255.0) as u8);
        println!("Decreased brightness by {}", -value * 255.0);
    }
    image
}

fn adjust_contrast(mut image: PhotonImage, value: f32) -> PhotonImage {
    photon_rs::effects::adjust_contrast(&mut image, value);
    image
}

fn rotate_image(image: PhotonImage, angle: f32) -> PhotonImage {
    photon_rs::transform::rotate(&image, angle)
}

fn blur_image(mut image: PhotonImage, radius: i32) -> PhotonImage {
    photon_rs::conv::gaussian_blur(&mut image, radius);
    image
}

fn ocr(image: PhotonImage) -> anyhow::Result<String> {
    let mut lt = leptess::LepTess::new(None, "eng").context("Couldn't initialize leptess")?;

    let mut img = helpers::dyn_image_from_raw(&image);
    img = ImageRgba8(img.to_rgba8());
    let mut buffer = vec![];
    let out_format = image::ImageOutputFormat::Tiff;
    img.write_to(&mut Cursor::new(&mut buffer), out_format)
        .context("Couldn't convert image to tiff")?;

    lt.set_image_from_mem(&buffer)
        .context("Couldn't set image from mem")?;
    lt.set_source_resolution(70);

    lt.get_utf8_text().context("Couldn't get utf8 text")
}
