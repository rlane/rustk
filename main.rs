#![no_std]
#![allow(ctypes)]
#![feature(globs)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(phase)]

#[phase(link, plugin)]
extern crate core;

/* HACK https://github.com/rust-lang/rust/issues/14342 */
extern crate std = "core";

use core::prelude::*;
use core::fmt;
use core::fmt::FormatWriter;

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
    fail!("Finished");
}

#[no_split_stack]
pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("cli; hlt");
        }
    }
}

struct SerialFmtWriter;

impl fmt::FormatWriter for SerialFmtWriter {
    fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        for &c in bytes.iter() {
            serial::write(c as char);
        }
        Ok(())
    }
}

#[no_split_stack]
#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    let mut w = SerialFmtWriter;
    match writeln!(w, "Failure: {} at {}:{}", fmt, file, line) {
        _ => ()
    };
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
