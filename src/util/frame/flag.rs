use crate::ffi::*;
use std::ffi::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const CORRUPT = AV_FRAME_FLAG_CORRUPT;
    }
}
