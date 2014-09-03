use core::mem::transmute;

#[repr(u32)]
enum Flag {
    FlagMem = 1<<0,
    FlagBootDevice = 1<<1,
    FlagCmdline = 1<<2,
    FlagMods = 1<<3,
    FlagMmap = 1<<6,
}

#[allow(dead_code)]
struct Info {
    flags : u32,
    mem_lower : u32,
    mem_upper : u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms1: u32,
    syms2: u32,
    syms3: u32,
    syms4: u32,
    mmap_length: u32,
    mmap_addr: u32,
}

#[allow(dead_code)]
struct Mem {
    size: u32,
    base_addr: u32,
    base_addr_hi: u32,
    length: u32,
    length_hi: u32,
    typ: u32,
}

extern {
    static multiboot_ptr : u32;
}

pub fn init() {
    let info : &Info = unsafe { transmute(multiboot_ptr as uint) };
    log!("Reading Multiboot information at {}", multiboot_ptr as *const Info);
    if info.flags & FlagMmap as u32 != 0 {
        log!("Memory map:");
        let mut offset = 0;
        while offset < info.mmap_length {
            let mem : &Mem = unsafe { transmute(info.mmap_addr as uint + offset as uint) };
            if mem.typ == 1 {
                log!("{} bytes available starting at 0x{:x}", mem.length, mem.base_addr);
            }
            offset += mem.size + 4;
        }
    }
}
