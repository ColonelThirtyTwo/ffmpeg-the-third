use crate::ffi::*;
use std::ffi::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const FORCED = AV_SUBTITLE_FLAG_FORCED;
    }
}
