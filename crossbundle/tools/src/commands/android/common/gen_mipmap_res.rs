use image::{DynamicImage, GenericImageView, ImageFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::error::*;
use crate::types::Config;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ImageGeneration {
    /// The path to the source icon will be provided to generate mipmap resources
    pub icon_path: PathBuf,
    /// Output path for generated mipmap resources. By default it is located in crossbow
    /// root dir
    pub output_path: Option<PathBuf>,
    /// Overwrite android resource directory
    pub force: bool,
}

impl ImageGeneration {
    /// Creates an empty ImageGeneration instance
    pub fn new(icon_path: PathBuf) -> Self {
        Self {
            icon_path,
            ..Default::default()
        }
    }

    /// Generate mipmap resources from the icon. Width and height of the icon must be
    /// equal
    pub fn gen_mipmap_res_from_icon(&self, config: &Config) -> Result<()> {
        let image = image::open(&self.icon_path)?;
        let (width, height) = image.dimensions();
        if width != height {
            return Err(Error::WidthAndHeightDifSizes);
        }
        for (name, size) in scale_down() {
            let scaled = image.thumbnail(size, size);
            if let Some(ref res_dir) = self.output_path {
                write_image(&res_dir, name, size, scaled, self.force, config)?;
            } else {
                let current_dir = current_dir()?
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_owned();
                write_image(&current_dir, name, size, scaled, self.force, config)?;
            }
        }
        Ok(())
    }
}

/// Check res directory and then create mipmap resource if it empty
fn write_image(
    res_dir: &Path,
    name: String,
    size: u32,
    scaled: DynamicImage,
    overwrite: bool,
    config: &Config,
) -> Result<()> {
    let mipmap_dirs = &res_dir
        .join("assets")
        .join("res")
        .join("android")
        // TODO: How to storage generated res?
        .join("generated_res")
        .join(format!("mipmap-{}", name));
    if mipmap_dirs.exists() {
        if overwrite {
            std::fs::remove_dir(&mipmap_dirs)?;
            std::fs::create_dir_all(&mipmap_dirs)?;
        }
        return Ok(());
    } else if !mipmap_dirs.exists() {
        std::fs::create_dir_all(&mipmap_dirs)?;
    }
    let mut output = File::create(mipmap_dirs.join("ic_launcher.png"))?;
    scaled.write_to(&mut output, ImageFormat::Png)?;
    config.status_message(
        "Generating mipmap resource",
        format!("{} with {}x{}", name, size, size,),
    )?;
    Ok(())
}

/// Scale image down according to scale ratio
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
    use crate::types::Shell;

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
        let image_generation = ImageGeneration {
            icon_path,
            output_path: Some(res_dir_path.clone()),
            force,
        };
        let shell = Shell::new();
        let config = Config::new(shell, res_dir_path.clone());
        image_generation.gen_mipmap_res_from_icon(&config).unwrap();
        assert!(res_dir_path
            .join("assets")
            .join("res")
            .join("android")
            .join("mipmap-hdpi")
            .join("ic_launcher.png")
            .exists())
    }
}
