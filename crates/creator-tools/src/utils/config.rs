use super::Shell;
use std::cell::{RefCell, RefMut};

/// Configuration information for creator. This is not specific to a build,
/// it is information relating to creator itself.
#[derive(Debug)]
pub struct Config {
    /// Information about how to write messages to the shell
    shell: RefCell<Shell>,
}

impl Config {
    /// Creates a new config instance.
    pub fn new(shell: Shell) -> Config {
        Config {
            shell: RefCell::new(shell),
        }
    }

    /// Gets a reference to the shell, e.g., for writing error messages.
    pub fn shell(&self) -> RefMut<'_, Shell> {
        self.shell.borrow_mut()
    }
}
