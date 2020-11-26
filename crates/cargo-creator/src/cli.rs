use clap::AppSettings;
pub use clap::Clap;

// Todo: maybe we need to think about better naming

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::TrailingVarArg)]
pub struct CliBuildAndroid {
    #[clap(allow_hyphen_values = true)]
    pub cargo_args: Vec<String>,
}

#[derive(Debug, Clap)]
pub enum CliBuildCmd {
    Android(CliBuildAndroid),
}

#[derive(Debug, Clap)]
pub struct CliCreatorBuild {
    #[clap(subcommand)]
    pub cmd: CliBuildCmd,
}

#[derive(Debug, Clap)]
pub enum CliCreatorCmd {
    Build(CliCreatorBuild),
}

#[derive(Debug, Clap)]
#[clap(author, about, version)]
pub struct CliCreator {
    #[clap(subcommand)]
    pub cmd: CliCreatorCmd,
}
