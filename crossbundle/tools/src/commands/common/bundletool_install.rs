use std::path::PathBuf;

pub struct BundletoolInstall {
    version: &'static str,
}

impl BundletoolInstall {
    pub fn new(version: &'static str) -> Self {
        Self { version }
    }

    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }

    pub fn install(&self) -> crate::error::Result<()> {
        let default_jar_path = dirs::home_dir()
            .ok_or_else(|| crate::error::Error::PathNotFound(PathBuf::from("$HOME")))?
            .join(self.file_name());

        let download_url = format!(
            "https://github.com/google/bundletool/releases/download/{}/{}",
            self.version,
            self.file_name()
        );

        if !default_jar_path.exists() {
            let response = ureq::get(&download_url)
                .call()
                .map_err(crate::error::Error::DownloadFailed)?;

            let mut out = std::fs::File::create(&default_jar_path).map_err(|cause| {
                crate::error::Error::JarFileCreationFailed {
                    path: default_jar_path.clone(),
                    cause,
                }
            })?;
            std::io::copy(&mut response.into_reader(), &mut out).map_err(|cause| {
                crate::error::Error::CopyToFileFailed {
                    path: default_jar_path,
                    cause,
                }
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bundletool_install_test() {
        BundletoolInstall::new("1.8.2").install().unwrap();
    }
}
