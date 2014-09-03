use core::fmt;
use halt;

#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    log!("Failure: {} at {}:{}", fmt, file, line);
    halt();
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_mangle]
#[no_split_stack]
pub fn __morestack() -> ! {
    fail!("__morestack called");
}
