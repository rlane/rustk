use log;

static mut GDT: [u64, .. 4] = [ 0, 0, 0, 0 ];
static TLS: [u32, .. 16] = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];

#[inline(never)]
#[no_split_stack]
pub fn init() {
    log("Initializing GDT ... ");
    unsafe {
        let gdt_base = &GDT as *const [u64, .. 4] as u32;
        let tls_base = &TLS as *const [u32, .. 16] as u32;
        let gdtr : u64 = 31 | (gdt_base as u64 << 16);

        GDT[0] = 0;
        GDT[1] = make_gdt_entry(0, 0xfffff, 0x9a, 0xc); // code
        GDT[2] = make_gdt_entry(0, 0xfffff, 0x92, 0xc); // data
        GDT[3] = make_gdt_entry(tls_base, 64, 0x92, 0x4); // TLS

        asm!("lgdtw ($0)" :: "r"(&gdtr));
        asm!("mov $0, %ds" :: "r"(0x10u32));
        asm!("mov $0, %es" :: "r"(0x10u32));
        asm!("mov $0, %fs" :: "r"(0x10u32));
        asm!("mov $0, %gs" :: "r"(0x18u32));
        asm!("mov $0, %ss" :: "r"(0x10u32));
        asm!("jmp $0, $$.flush; .flush:" :: "Ir"(8u32));
    }
    log("done\n");
}

#[no_split_stack]
fn make_gdt_entry(base: u32, limit: u32, access: u8, gran: u8) -> u64 {
    let mut v = 0u64;
    v |= limit as u64 & 0xffff;
    v |= (base as u64 & 0xffffff) << 16;
    v |= (access as u64) << 40;
    v |= (limit as u64 >> 16) << 48;
    v |= (gran as u64) << 52;
    v |= (base as u64 >> 24) << 56;
    v
}
