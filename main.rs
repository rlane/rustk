#![no_std]
#![allow(ctypes)]
#![feature(globs)]
#![feature(lang_items)]
#![feature(asm)]

extern crate core;

use core::prelude::*;
use core::fmt;

mod serial;
mod gdt;

#[no_split_stack]
pub fn log(msg: &str) {
    for c in msg.as_slice().chars() {
        serial::write(c);
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    serial::init();
    log("Hello from Rust\n");
    gdt::init();
    log("Initialization complete\n");
    halt();
}

#[no_split_stack]
pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("cli; hlt");
        }
    }
}

#[no_split_stack]
#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    log("begin_unwind called, halting");
    halt();
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_split_stack]
#[no_mangle]
pub fn __morestack() -> ! {
    log("__morestack called, halting");
    halt();
}
