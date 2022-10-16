use crate::{cargo_crate_version, error::*};
use crossbundle_tools::types::Config;

/// Self-update crossbundle project and output update status
pub(crate) fn self_update(config: &Config) -> Result<bool> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("dodorare")
        .repo_name("crossbow")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    config.status_message("Update status:", status.version())?;
    Ok(true)
}

#[macro_export]
macro_rules! cargo_crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
