AS := i686-elf-as
CC := i686-elf-gcc

CFLAGS := -std=gnu99 -ffreestanding -O2 -Wall -Wextra

all: rustk

run: rustk
	kvm -kernel rustk

rustk: boot.o kernel.o
	$(CC) -T linker.ld -o rustk -nostdlib $^ -lgcc

iso: rustk.iso
rustk.iso: rustk grub.cfg
	mkdir -p isodir
	mkdir -p isodir/boot
	cp rustk isodir/boot/rustk
	mkdir -p isodir/boot/grub
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o rustk.iso isodir

clean:
	rm -f rustk *.o rustk.iso

.PHONY: all run iso clean
