static PORT : u16 = 0x3f8; /* COM1 */

unsafe fn outb(port: u16, val: u8) {
    asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
}

unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm!("in $0, $1" : "={al}"(val) : "{dx}"(port) :: "intel");
    val
}

pub fn init() {
    unsafe {
        outb(PORT + 1, 0x00);    // Disable all interrupts
        outb(PORT + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(PORT + 1, 0x00);    //                  (hi byte)
        outb(PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set
    }
}

fn is_transmit_empty() -> bool {
    unsafe {
        return (inb(PORT + 5) & 0x20) != 0;
    }
}

pub fn write(c: char) {
   while !is_transmit_empty() {}

   unsafe { outb(PORT, c as u8); }
}

pub fn debug_write(c: char) {
   unsafe { outb(0xe9, c as u8); }
}
