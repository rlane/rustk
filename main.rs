#![no_std]
#![allow(ctypes)]
#![feature(globs)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(phase)]
#![feature(macro_rules)]

#[phase(link, plugin)]
extern crate core;

/* HACK https://github.com/rust-lang/rust/issues/14342 */
extern crate std = "core";

use core::prelude::*;

mod serial;

#[macro_export]
macro_rules! log(
    ($($arg:tt)*) => ({
        use serial::SerialFmtWriter;
        use core::fmt::FormatWriter;
        let mut w = SerialFmtWriter;
        match writeln!(w, $($arg)*) {
            _ => ()
        };
    })
)

#[no_mangle]
pub fn main() -> ! {
    serial::init();
    log!("Hello from Rust");
    log!("Initialization complete");
    fail!("Finished");
}

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("cli; hlt");
        }
    }
}

#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &core::fmt::Arguments, file: &str, line: uint) -> ! {
    log!("Failure: {} at {}:{}", fmt, file, line);
    halt();
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_mangle]
pub fn __morestack() -> ! {
    fail!("__morestack called");
}
