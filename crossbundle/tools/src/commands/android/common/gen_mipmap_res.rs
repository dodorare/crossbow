use image::{DynamicImage, GenericImageView, ImageFormat};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::error::*;

#[derive(Debug, Default, Clone)]
pub struct ImageGeneration {
    pub icon_path: PathBuf,
    pub res_dir_path: Option<PathBuf>,
    pub force: bool,
    pub image_format: Option<ImageFormat>,
}

impl ImageGeneration {
    pub fn new(icon_path: PathBuf) -> Self {
        Self {
            icon_path,
            ..Default::default()
        }
    }

    pub fn gen_mipmap_res_from_icon(&self) -> Result<()> {
        let image = image::open(&self.icon_path)?;
        let (width, height) = image.dimensions();
        if width != height {
            return Err(Error::WidthAndHeightDifSizes);
        }
        let res = Path::new("assets").join("res");
        for (name, size) in scale_down() {
            let scaled = image.thumbnail(size, size);
            if let Some(ref res_dir) = self.res_dir_path {
                let res_dir = res_dir.join(&res);
                write_image(&res_dir, name, size, scaled, self.force, self.image_format)?;
            } else {
                let current_dir = current_dir()?
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_owned();
                let res_dir = current_dir.join(&res);
                write_image(&res_dir, name, size, scaled, self.force, self.image_format)?;
            }
        }
        Ok(())
    }
}

fn write_image(
    res_dir: &Path,
    name: String,
    size: u32,
    scaled: DynamicImage,
    overwrite: bool,
    image_format: Option<ImageFormat>,
) -> Result<()> {
    let mipmap_dirs = &res_dir
        .join("android")
        .join(format!("mipmap-{}", name))
        .to_owned();
    if mipmap_dirs.exists() {
        return Ok(());
    } else if !mipmap_dirs.exists() {
        std::fs::create_dir_all(&mipmap_dirs)?;
    } else if overwrite {
        std::fs::remove_dir(&res_dir.join("android"))?;
        std::fs::create_dir_all(&mipmap_dirs)?;
    }
    let mut output = File::create(mipmap_dirs.join("ic_launcher.png"))?;
    if let Some(format) = image_format {
        scaled.write_to(&mut output, format)?;
    } else {
        scaled.write_to(&mut output, ImageFormat::Png)?;
    }
    println!("Generated for {} with {}x{} size", name, size, size);
    Ok(())
}

fn scale_down() -> HashMap<String, u32> {
    let mut buf = HashMap::new();
    buf.insert(MipmapDpi::Xxxhdpi.to_string(), 192);
    buf.insert(MipmapDpi::Xxhdpi.to_string(), 144);
    buf.insert(MipmapDpi::Xhdpi.to_string(), 96);
    buf.insert(MipmapDpi::Hdpi.to_string(), 72);
    buf.insert(MipmapDpi::Mdpi.to_string(), 48);
    buf.insert(MipmapDpi::Ldpi.to_string(), 36);
    buf
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
        let force = false;
        let image_format = ImageFormat::Png;
        let image_generation = ImageGeneration {
            icon_path,
            res_dir_path: Some(res_dir_path.clone()),
            force,
            image_format: Some(image_format),
        };
        image_generation.gen_mipmap_res_from_icon().unwrap();
        assert!(res_dir_path
            .join("assets")
            .join("res")
            .join("android")
            .join("mipmap-hdpi")
            .join("ic_launcher.png")
            .exists())
    }
}
