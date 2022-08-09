use image::{DynamicImage, GenericImageView, ImageFormat};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::path::Path;

use crate::error::*;

fn gen_icons(icon_path: &Path, res_dir_path: Option<&Path>, force: bool) -> Result<()> {
    let image = image::open(icon_path)?;
    let (width, height) = image.dimensions();
    if width != height {
        return Err(Error::WidthAndHeightDifSizes);
    }
    for (name, size) in scale_down(width) {
        let scaled = image.thumbnail(size, size);
        // Check or create res directory
        let res = Path::new("assets").join("res");
        if let Some(res_dir) = res_dir_path {
            let res_dir = res_dir.join(res);
            write_image(&res_dir, name, size, scaled, force)?;
        } else {
            let current_dir = current_dir()?;
            let res_dir = current_dir.join(res);
            write_image(&res_dir, name, size, scaled, force)?;
        }
    }
    Ok(())
}

fn write_image(
    res_dir: &Path,
    name: String,
    size: u32,
    scaled: DynamicImage,
    overwrite: bool,
) -> Result<()> {
    let mipmap_dirs = &res_dir
        .join("android")
        .join(format!("mipmap-{}", name))
        .to_owned();
    if mipmap_dirs.exists() {
        return Err(Error::IconsAlreadyExist);
    } else if !mipmap_dirs.exists() {
        std::fs::create_dir_all(&mipmap_dirs)?;
    } else if overwrite {
        std::fs::remove_dir(&res_dir.join("android"))?;
        std::fs::create_dir_all(&mipmap_dirs)?;
    }
    let mut output = File::create(mipmap_dirs.join(format!("{}-{}.png", name, size)))?;
    scaled.write_to(&mut output, ImageFormat::Png)?;
    Ok(())
}

fn scale_down(width: u32) -> HashMap<String, u32> {
    let mut buf = HashMap::new();
    buf.insert(MipmapDpi::Xxxhdpi.to_string(), width * 16 / 16);
    buf.insert(MipmapDpi::Xxhdpi.to_string(), width * 12 / 16);
    buf.insert(MipmapDpi::Xhdpi.to_string(), width * 8 / 16);
    buf.insert(MipmapDpi::Hdpi.to_string(), width * 6 / 16);
    buf.insert(MipmapDpi::Mdpi.to_string(), width * 4 / 16);
    buf.insert(MipmapDpi::Ldpi.to_string(), width * 3 / 16);
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