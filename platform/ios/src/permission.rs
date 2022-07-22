use crate::{error::*, types::*};

pub fn request_permission(_permission: &IosPermission) -> Result<bool> {
    panic!("iOS permissions not supported yet");
    // Ok(false)
}
