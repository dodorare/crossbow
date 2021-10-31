use crate::error::Result;
use clap::Parser;
use crossbundle_tools::{
    commands::{check_cargo_generate, create_project},
    utils::Config,
};

const TEMPLATES_REPO: &str = "https://github.com/creator-rs/creator-templates.git";

#[derive(Parser, Clone, Debug)]
pub struct NewCommand {
    /// Directory to create / project name; if the name isn't in kebab-case, it will be converted
    /// to kebab-case unless `--force` is given.
    pub name: String,
    /// Name of the template to create.
    #[clap(long, short)]
    pub template: Option<String>,
    /// Don't convert the project name to kebab-case before creating the directory.
    /// Note that cargo generate won't overwrite an existing directory, even if `--force` is given.
    #[clap(long, short)]
    pub force: bool,
}

impl NewCommand {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        if !check_cargo_generate() {
            config
                .shell()
                .warn("To use `crossbundle new ...` command you need to install `cargo-generate`\n         run `cargo install cargo-generate`")?;
            return Ok(());
        };
        create_project(
            config.current_dir(),
            &self.name,
            TEMPLATES_REPO,
            &self.template,
        )?;
        Ok(())
    }
}
