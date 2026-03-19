use std::panic;
use std::process;

use crate::ffi::*;
use libc::{c_int, c_void};

pub struct Interrupt {
    pub interrupt: AVIOInterruptCB,
}

extern "C" fn callback<F>(opaque: *mut c_void) -> c_int
where
    F: Fn() -> bool + Send + Sync,
{
    match panic::catch_unwind(|| unsafe { (*opaque.cast::<F>().cast_const())() }) {
        Ok(ret) => ret as c_int,
        Err(_) => process::abort(),
    }
}

pub fn new<F>(opaque: Box<F>) -> Interrupt
where
    F: Fn() -> bool + Send + Sync,
{
    let interrupt_cb = AVIOInterruptCB {
        callback: Some(callback::<F>),
        opaque: Box::into_raw(opaque) as *mut c_void,
    };
    Interrupt {
        interrupt: interrupt_cb,
    }
}
