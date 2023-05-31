#![no_std]
#![no_main]

extern crate nx;
extern crate alloc;
use core::{panic};

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

type EGLNativeDisplayType = *mut core::ffi::c_void;
type EGLDisplay = *mut core::ffi::c_void;

extern "C" {
    pub fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
}

#[no_mangle]
pub fn main() {
    unsafe { eglGetDisplay(0 as *mut core::ffi::c_void) };

    loop {
        //svc::sleep_thread(10_000_000)?;
    }
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    //util::simple_panic_handler::<log::lm::LmLogger>(info, abort::AbortLevel::FatalThrow())
    loop {
        // Sleep 10ms (aka 10'000'000 ns)
    }
}