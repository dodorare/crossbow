use super::Shell;
use crate::error::Result;
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

    /// Shortcut to right-align and color green a status message.
    pub fn status_message<T, U>(&self, status: T, message: U) -> Result<()>
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        self.shell().status_message(status, message)
    }

    /// Shortcut to right-align and color green a status.
    pub fn status<T>(&self, status: T) -> Result<()>
    where
        T: std::fmt::Display,
    {
        self.shell().status(status)
    }

    /// Gets a reference to the current working dir.
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
}
