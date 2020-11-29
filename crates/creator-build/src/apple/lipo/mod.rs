// mod cargo;
// mod exec;
// mod lipo;
mod meta;
// mod xcode;

use cargo_metadata::MetadataCommand;

pub fn compile() -> anyhow::Result<()> {
    if cfg!(not(target_os = "macos")) {
        log::warn!("Running on non-macOS, `lipo` likely will be not found");
    }

    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()?;
    log::trace!("Metadata: {:#?}", meta);

    // let meta = meta::Meta::new(&invocation, &meta)?;

    // if invocation.xcode_integ {
    //     xcode::integ(&meta, invocation)
    // } else {
    //     lipo::build(&cargo::Cargo::new(&invocation), &meta, &invocation.targets)
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        compile().unwrap();
    }
}
