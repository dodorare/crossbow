use crate::*;
use clap::Clap;
use creator_tools::types::Profile;

#[derive(Clap)]
pub struct AppleBuildCommand {
    /// Build profile. Can be one of: debug, release
    #[clap(short, long, default_value = "debug")]
    pub profile: Profile,
}

impl AppleBuildCommand {
    pub fn run(&self) -> Result<()> {
        println!("Run AppleBuildCommand");
        Ok(())
    }
}
