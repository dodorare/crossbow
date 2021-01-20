use super::Shell;
use std::cell::{RefCell, RefMut};
use std::path::{Path, PathBuf};

/// Configuration information for creator. This is not specific to a build,
/// it is information relating to creator itself.
#[derive(Debug)]
pub struct Config {
    /// Information about how to write messages to the shell
    shell: RefCell<Shell>,
    /// Current working dir
    current_dir: PathBuf,
}

impl Config {
    /// Creates a new config instance.
    pub fn new(shell: Shell, current_dir: PathBuf) -> Config {
        Config {
            shell: RefCell::new(shell),
            current_dir,
        }
    }

    /// Gets a reference to the shell, e.g., for writing error messages.
    pub fn shell(&self) -> RefMut<'_, Shell> {
        self.shell.borrow_mut()
    }

    /// Gets a reference to the current working dir.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
