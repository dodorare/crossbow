use super::apk::config::ApkBuilderConfig;
use super::error::NdkError;

pub struct AndroidBuilder;

impl AndroidBuilder {
    pub fn apk(self) -> Result<ApkBuilderConfig, NdkError> {
        ApkBuilderConfig::new()
    }

    // Todo: add `aab` function later
}
