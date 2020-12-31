use crate::error::*;
use clap::Clap;
use creator_tools::types::Profile;

#[derive(Clap)]
pub struct AndroidBuildCommand {
    /// Build profile. Can be one of: debug, release
    #[clap(short, long, default_value = "debug")]
    pub profile: Profile,
}

impl AndroidBuildCommand {
    pub fn run(&self) -> Result<()> {
        println!("Run AndroidBuildCommand");
        Ok(())
    }
}
