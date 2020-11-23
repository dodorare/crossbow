mod builder;
mod cargo;
mod cli;
mod error;

use builder::*;
use cli::*;

fn main() {
    let cli_creator = CliCreator::parse();
    // let args = std::env::args().peekable();
    // println!("{:?}", args.collect::<Vec<String>>());
    match cli_creator.cmd {
        CliCreatorCmd::Build(build) => match build.cmd {
            CliBuildCmd::Android(android) => {
                let apk = CreatorBuilder::android()
                    .apk()
                    .cli_cmd(android)
                    .finish()
                    .build()
                    .unwrap();
            }
        },
    }
}
