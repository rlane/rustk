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

mod log;
mod serial;
pub mod lang;
mod multiboot;

#[no_mangle]
pub fn main() -> ! {
    serial::init();
    log!("Hello from Rust");
    multiboot::init();
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
