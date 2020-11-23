use super::apk::config::ApkBuilderConfig;

pub struct AndroidBuilder;

impl AndroidBuilder {
    pub fn apk(self) -> ApkBuilderConfig {
        ApkBuilderConfig::default()
    }

    // Todo: add `aab` function later
}
