mod android_ndk;
mod android_sdk;

pub use android_ndk::*;
pub use android_sdk::*;
pub use android_tools::aapt2::*;
pub use android_tools::bundletool::*;
pub use android_tools::error::Error as AndroidToolsError;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CheckInfo {
    pub dependency_name: String,
    pub check_name: String,
    pub passed: bool,
}

impl CheckInfo {
    fn invert_passed(mut self) -> CheckInfo {
        self.passed = !self.passed;
        self
    }
}

pub trait IntoCheckInfo: Sized {
    fn check_passed(self) -> CheckInfo;
    fn check_failed(self) -> CheckInfo {
        self.check_passed().invert_passed()
    }
}
