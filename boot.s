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

	call enter_long_mode

	# Rust needs %gs for __morestack
	call setup_gdt

	# Jump to Rust, will not return
	call main
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

# GDTR data
.section .data.gdtr
.word 31
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

    # TODO load 64-bit CS
    ret

.section .data.pml4
.align 4096
.quad 0x0000000000000007
.skip 4088

.section .data.pdpt
.align 4096
.quad 0x0000000000000087
.skip 4088
