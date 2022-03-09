/// Change crate-type from bin to cdylib. Replace output directory with the directory we
/// created
pub fn change_crate_name(
    build_target_dir: &std::path::Path,
    cmd: &cargo_util::ProcessBuilder,
    change_out_dir: bool,
    from: &str,
    to: &str,
) -> cargo::CargoResult<Vec<std::ffi::OsString>> {
    let mut new_args = cmd.get_args().to_owned();
    let mut iter = new_args.iter_mut().rev().peekable();
    while let Some(arg) = iter.next() {
        if let Some(prev_arg) = iter.peek() {
            if *prev_arg == "--crate-type" && arg == from {
                *arg = to.into();
            } else if *prev_arg == "--out-dir" && change_out_dir {
                *arg = build_target_dir.into();
            }
        }
    }
    Ok(new_args)
}
