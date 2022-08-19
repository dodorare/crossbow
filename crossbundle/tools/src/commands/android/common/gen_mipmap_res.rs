use crate::error::*;
use image::{DynamicImage, GenericImageView, ImageFormat, Rgba};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ImageGeneration {
    /// The path to the source icon will be provided to generate mipmap resources.
    pub icon_path: PathBuf,
    /// The output name of the icon that will be generated in mipmap resources.
    pub out_icon_name: String,
    /// Output path to Android resources for generated mipmap resources.
    pub output_path: PathBuf,
    /// Overwrite android resource directory.
    pub force: bool,
}

impl ImageGeneration {
    /// Generate mipmap resources from the icon. Width and height of the icon must be
    /// equal.
    pub fn gen_mipmap_res_from_icon(&self) -> Result<()> {
        let image = image::open(&self.icon_path)?;
        let (width, height) = image.dimensions();
        if width != height || height % 2 != 0 {
            return Err(Error::WidthAndHeightDifSizes);
        }
        for (name, size) in get_icon_sizes() {
            let scaled = image.thumbnail(size, size);
            let img = Self::round_image(&scaled, size);
            // TODO: Add shadow. See this: https://github.com/romannurik/AndroidAssetStudio
            self.write_image(&name, img)?;
        }
        Ok(())
    }

    fn round_image(scaled: &DynamicImage, size: u32) -> DynamicImage {
        let border = size as f64 / 25.0;
        let radius = size as f64 / 2.0;
        let cxy = size as f64 / 2.0 - 1.0;
        let mut img = scaled.to_rgba8();
        for pix in img.clone().enumerate_pixels() {
            let s = (pix.0 as f64 - cxy).hypot(pix.1 as f64 - cxy);
            if s > radius - border {
                img.put_pixel(pix.0, pix.1, Rgba([0, 0, 0, 0]));
            }
        }
        img.into()
    }

    /// Check res directory and then create mipmap resource if it's empty
    pub fn write_image(&self, mipmap_name: &str, scaled: DynamicImage) -> Result<()> {
        let mipmap_dirs = self.output_path.join(format!("mipmap-{}", mipmap_name));
        if mipmap_dirs.exists() {
            if self.force {
                std::fs::remove_dir(&mipmap_dirs)?;
                std::fs::create_dir_all(&mipmap_dirs)?;
            }
            return Ok(());
        } else if !mipmap_dirs.exists() {
            std::fs::create_dir_all(&mipmap_dirs)?;
        }
        let mut output = File::create(mipmap_dirs.join(&self.out_icon_name))?;
        scaled.write_to(&mut output, ImageFormat::Png)?;
        Ok(())
    }
}

/// Scale image down according to scale ratio.
fn get_icon_sizes() -> Vec<(String, u32)> {
    vec![
        (MipmapDpi::Xxxhdpi.to_string(), 192),
        (MipmapDpi::Xxhdpi.to_string(), 144),
        (MipmapDpi::Xhdpi.to_string(), 96),
        (MipmapDpi::Hdpi.to_string(), 72),
        (MipmapDpi::Mdpi.to_string(), 48),
        (MipmapDpi::Ldpi.to_string(), 36),
    ]
}

pub enum MipmapDpi {
    Xxxhdpi,
    Xxhdpi,
    Xhdpi,
    Hdpi,
    Mdpi,
    Ldpi,
}

impl ToString for MipmapDpi {
    fn to_string(&self) -> String {
        match self {
            Self::Xxxhdpi => "xxxhdpi".to_string(),
            Self::Xxhdpi => "xxhdpi".to_string(),
            Self::Xhdpi => "xhdpi".to_string(),
            Self::Hdpi => "hdpi".to_string(),
            Self::Mdpi => "mdpi".to_string(),
            Self::Ldpi => "ldpi".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_icon_gen() {
        let tempfile = tempfile::tempdir().unwrap();
        let res_dir_path = tempfile.path().to_path_buf();
        let icon_path = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets")
            .join("images")
            .join("icon.png");
        let image_generation = ImageGeneration {
            icon_path,
            out_icon_name: "ic_launcher.png".to_owned(),
            output_path: res_dir_path.join("res"),
            force: false,
        };
        image_generation.gen_mipmap_res_from_icon().unwrap();
        assert!(res_dir_path
            .join("res")
            .join("mipmap-hdpi")
            .join("ic_launcher.png")
            .exists())
    }
}
