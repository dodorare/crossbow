#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionStatus {
    // Permission is in an unknown state.
    Unknown = 0,
    // Denied by user.
    Denied = 1,
    // Feature is disabled on device.
    Disabled = 2,
    // Granted by user.
    Granted = 3,
    // Restricted (only iOS).
    Restricted = 4,
}

impl From<bool> for PermissionStatus {
    fn from(status: bool) -> Self {
        match status {
            true => Self::Granted,
            false => Self::Denied,
        }
    }
}

#[cfg(all(target_os = "ios", feature = "ios"))]
use crossbow_ios::permission::AuthorizationStatus;

#[cfg(all(target_os = "ios", feature = "ios"))]
impl From<AuthorizationStatus> for PermissionStatus {
    fn from(status: AuthorizationStatus) -> Self {
        match status {
            AuthorizationStatus::NotDetermined => Self::Unknown,
            AuthorizationStatus::Restricted => Self::Restricted,
            AuthorizationStatus::Denied => Self::Denied,
            AuthorizationStatus::Authorized => Self::Granted,
            AuthorizationStatus::Limited => Self::Granted,
        }
    }
}
