mod builder;
mod cargo;
mod cli;
mod error;

use builder::*;
use cli::*;

use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let cli_creator = CliCreator::parse();
    // let args = std::env::args().peekable();
    // println!("{:?}", args.collect::<Vec<String>>());
    match cli_creator.cmd {
        CliCreatorCmd::Build(build) => match build.cmd {
            CliBuildCmd::Android(android) => {
                let args = cargo::cmd::CargoBuildArgs::from_cargo_args(android.cargo_args)?;
                let manifest = cargo::current_manifest()?;
                // println!("{:#?}", manifest);
                let apk = CreatorBuilder::android()
                    .apk()?
                    // .cli(android)?
                    .manifest(manifest)?;
                // println!("{:?}", apk);
            }
        },
    }
    Ok(())
}

// pub fn find_package(
//     path: &Path,
//     name: Option<&str>,
// ) -> Result<(PathBuf, String, Option<String>), Error> {
//     let path = std::fs::canonicalize(path)?;
//     for manifest_path in path
//         .ancestors()
//         .map(|dir| dir.join("Cargo.toml"))
//         .filter(|dir| dir.exists())
//     {
//         let manifest = Manifest::parse_from_toml(&manifest_path)?;
//         let lib_name = manifest.lib.as_ref().and_then(|lib| lib.name.clone());
//         if let Some(p) = manifest.package.as_ref() {
//             if let (Some(n1), n2) = (name, &p.name) {
//                 if n1 == n2 {
//                     return Ok((manifest_path, p.name.clone(), lib_name));
//                 }
//             } else {
//                 return Ok((manifest_path, p.name.clone(), lib_name));
//             }
//         }
//         if let (Some(w), Some(name)) = (manifest.workspace.as_ref(), name) {
//             if let Some(manifest_path) = member(&manifest_path, &w.members, name)? {
//                 return Ok((manifest_path, name.to_string(), lib_name));
//             }
//         }
//     }
//     Err(Error::ManifestNotFound)
// }
