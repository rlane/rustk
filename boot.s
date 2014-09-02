# Originally from http://wiki.osdev.org/Bare_Bones (under public domain)

# Declare constants used for creating a multiboot header.
.set ALIGN,    1<<0             # align loaded modules on page boundaries
.set MEMINFO,  1<<1             # provide memory map
.set FLAGS,    ALIGN | MEMINFO  # this is the Multiboot 'flag' field
.set MAGIC,    0x1BADB002       # 'magic number' lets bootloader find the header
.set CHECKSUM, -(MAGIC + FLAGS) # checksum of above, to prove we are multiboot

# Declare a header as in the Multiboot Standard. We put this into a special
# section so we can force the header to be in the start of the final program.
.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

# Allocate initial stack
.section .bootstrap_stack, "aw", @nobits
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

# Entrypoint from bootloader
.section .text
.global _start
.type _start, @function
_start:
	# To set up a stack, we simply set the esp register to point to the top of
	# our stack (as it grows downwards).
	movl $stack_top, %esp

	# Save pointer to multiboot info
	mov %ebx, .data.multiboot_info

	# Rust needs %gs for __morestack
	call setup_gdt

	call check_long_mode
	call enter_long_mode

	# Jump to Rust, will not return
	call main
	#lcall $0x20, $main # switch to 64-bit
	cli
	hlt
.size _start, . - _start

# Multiboot info pointer
.section .data.multiboot_info
.global multiboot_ptr
multiboot_ptr:
.align 4
.long 0

# TLS data
.section .data.tls
.skip 64

# GDT data
.section .data.gdt
.quad 0
.quad 0x00cf9a000000ffff # code
.quad 0x00cf92000000ffff # data
.quad 0x00cf92000000ffff # TLS
.quad 0x00af9a000000ffff # 64-bit code

# GDTR data
.section .data.gdtr
.word (8*5)-1
.long .data.gdt

.section .text
setup_gdt:
    # Update TLS descriptor
    mov $.data.tls, %eax
    and $0xffffff, %eax
    or %eax, (.data.gdt+26)
    # Load segments
    mov $.data.gdtr, %eax
    lgdtw (%eax)
    mov $0x10, %eax
    mov %eax, %ds
    mov %eax, %es
    mov %eax, %fs
    mov %eax, %ss
    mov $0x18, %eax
    mov %eax, %gs
    ljmp $0x08, $.Lhere
    .Lhere:
    ret

.section .text
dbg:
    push %edx
    push %eax
    mov $0x3f8, %dx
    mov $0x58, %al
    out %al, (%dx)
    mov $0xe9, %dx
    out %al, (%dx)
    pop %eax
    pop %edx
    ret

.section .text
check_long_mode:
    mov $0x80000001, %eax
    cpuid
    test $(1<<29), %edx
    jz abort_no_64bit
    test $(1<<26), %edx
    jz abort_no_1gb_pages
    ret

abort_no_64bit:
    mov $.data.no_64bit, %esi
    mov $39, %ecx
    mov $0xe9, %dx
    rep outsb
    cli
    hlt
    jmp abort_no_64bit

abort_no_1gb_pages:
    mov $.data.no_1gb_pages, %esi
    mov $38, %ecx
    mov $0xe9, %dx
    rep outsb
    cli
    hlt
    jmp abort_no_1gb_pages

.section .text
enter_long_mode:
    # Populate physical address in pml4
    mov $.data.pdpt, %eax
    or %eax, .data.pml4

    # Set CR4.pae = 1
    mov %cr4, %eax
    or $(1 << 5), %eax
    mov %eax, %cr4

    # Load CR3 with pml4
    mov $.data.pml4, %eax
    mov %eax, %cr3

    # Set IA32_EFER.LME = 1.
    mov $0xc0000080, %ecx
    rdmsr
    or $(1 << 8), %eax
    wrmsr

    # Set CR0.pg = 1
    mov %cr0, %eax
    or $(1 << 31), %eax
    mov %eax, %cr0

    ret

.section .data.pml4
.align 4096
.quad 0x0000000000000007
.skip 4088

.section .data.pdpt
.align 4096
.quad 0x0000000000000087
.skip 4088

.section .data.no_64bit
.ascii "Processor does not support 64-bit mode\n"

.section .data.no_1gb_pages
.ascii "Processor does not support 1 GB pages\n"
