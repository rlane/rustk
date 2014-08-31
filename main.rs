#![no_std]
#![allow(ctypes)]
#![feature(globs)]
#![feature(lang_items)]
#![feature(asm)]

extern crate core;

use core::prelude::*;
use core::fmt;

extern {
    fn write_serial(c: char);
}

#[no_split_stack]
fn log(msg: &str) {
    for c in msg.as_slice().chars() {
        unsafe { write_serial(c) };
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    log("Hello from Rust\n");
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
