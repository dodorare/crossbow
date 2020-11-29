use std::path::PathBuf;

pub struct Meta {
    pub packages: Vec<Package>,
    pub target_dir: PathBuf,
}

pub struct Package {
    pub name: String,
    pub lib_name: String,
}

impl Meta {
    pub fn new(
        packages: Vec<String>,
        all: bool,
        meta: &cargo_metadata::Metadata,
    ) -> anyhow::Result<Meta> {
        let package_names: Vec<_>;
        let staticlib_required;

        if !packages.is_empty() {
            package_names = packages.iter().map(|p| p.as_str()).collect();
            staticlib_required = true;
        } else {
            package_names = meta.workspace_members.iter().map(|m| m.name()).collect();
            // Require a staticlib for single-member workspaces unless `--all` was specified.
            staticlib_required = meta.workspace_members.len() == 1 && !all;
        }

        log::debug!(
            "Considering package(s) {:?}, `staticlib` target {}",
            package_names,
            if staticlib_required {
                "required"
            } else {
                "not required"
            }
        );

        let mut packages = vec![];

        for &name in &package_names {
            let package = match meta.packages.iter().find(|p| p.name == name) {
                Some(p) => p,
                None => bail!("No package metadata found for {:?}", name),
            };

            let lib_targets: Vec<_> = package
                .targets
                .iter()
                .filter(|t| t.kind.iter().any(|k| k == "staticlib"))
                .collect();

            match lib_targets.as_slice() {
                [] => {
                    if !staticlib_required {
                        log::debug!(
                            "Ignoring {:?} because it does not have a `staticlib` target",
                            name
                        );
                        continue;
                    }
                    bail!("No library target found for {:?}", name);
                }
                [target] => {
                    if target.crate_types.iter().any(|ct| ct == "staticlib") {
                        packages.push((package, target.name.replace('-', "_")));
                    } else {
                        if !staticlib_required {
                            log::debug!(
                                "Ignoring {:?} because it does not have a `staticlib` crate type",
                                name
                            );
                            continue;
                        }
                        bail!("No staticlib crate type found for {:?}", name);
                    }
                }
                _ => bail!("Found multiple lib targets for {:?}", name),
            }
        }

        let packages = packages
            .into_iter()
            .map(|(p, lib_name)| Package {
                name: p.name.as_str(),
                lib_name,
            })
            .collect::<Vec<_>>();

        let package_names = packages.iter().map(|p| p.name).collect::<Vec<_>>();

        if packages.is_empty() {
            bail!(
                "Did not find any packages with a `staticlib` target, considered {:?}",
                package_names
            );
        }

        log::info!("Will build universal library for {:?}", package_names);

        Ok(Meta {
            packages,
            target_dir: PathBuf::new(&meta.target_directory),
        })
    }
}
