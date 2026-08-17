use crate::error::{AndroidError, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

/// Find gradle executable file in and initialize it
pub fn gradle_init() -> Result<Command> {
    let path = std::env::var_os("PATH");
    let gradle_home = std::env::var_os("GRADLE_HOME");
    find_gradle(path.as_deref(), gradle_home.as_deref())
        .map(Command::new)
        .ok_or_else(|| AndroidError::GradleNotFound.into())
}

fn find_gradle(path: Option<&OsStr>, gradle_home: Option<&OsStr>) -> Option<PathBuf> {
    path.into_iter()
        .flat_map(std::env::split_paths)
        .map(|directory| directory.join(bat!("gradle")))
        .find(|candidate| is_executable(candidate))
        .or_else(|| {
            gradle_home
                .map(|home| PathBuf::from(home).join("bin").join(bat!("gradle")))
                .filter(|candidate| is_executable(candidate))
        })
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::find_gradle;

    #[test]
    fn path_takes_precedence_over_gradle_home() {
        let temp = tempfile::tempdir().unwrap();
        let path_dir = temp.path().join("path/bin");
        let gradle_home = temp.path().join("gradle-home");
        std::fs::create_dir_all(&path_dir).unwrap();
        let path_gradle = path_dir.join(bat!("gradle"));
        std::fs::write(&path_gradle, "").unwrap();
        make_executable(&path_gradle);

        let path = std::env::join_paths([&path_dir]).unwrap();
        assert_eq!(
            find_gradle(Some(&path), Some(gradle_home.as_os_str())).unwrap(),
            path_gradle
        );
    }

    #[test]
    fn gradle_home_is_used_when_path_has_no_gradle() {
        let temp = tempfile::tempdir().unwrap();
        let gradle_home = temp.path().join("gradle-home");
        let gradle = gradle_home.join("bin").join(bat!("gradle"));
        std::fs::create_dir_all(gradle.parent().unwrap()).unwrap();
        std::fs::write(&gradle, "").unwrap();
        make_executable(&gradle);

        assert_eq!(
            find_gradle(None, Some(gradle_home.as_os_str())).unwrap(),
            gradle
        );
    }

    #[cfg(unix)]
    fn make_executable(path: &std::path::Path) {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = path.metadata().unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).unwrap();
    }

    #[cfg(windows)]
    fn make_executable(_path: &std::path::Path) {}
}
