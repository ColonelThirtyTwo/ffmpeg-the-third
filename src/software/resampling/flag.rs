use crate::ffi::*;
use std::ffi::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const FORCE = SWR_FLAG_RESAMPLE;
    }
}
