AS := i686-elf-as
CC := i686-elf-gcc
LD := i686-elf-ld
RUSTC := rustc

CFLAGS := -std=gnu99 -ffreestanding -O2 -Wall -Wextra

all: rustk

run: rustk
	@echo "Hit CTRL-a x to terminate, CTRL-a h for help"
	kvm -kernel rustk \
		-netdev user,id=hostnet0 \
		-device virtio-net-pci,romfile=,netdev=hostnet0 \
		-nographic

%.o: %.rs
	$(RUSTC) -O --target i686-unknown-linux-gnu \
		--crate-type staticlib -o $@ \
		--emit obj $< \
		-Z no-landing-pads -Z lto \
		-C relocation-model=static -g

rustk: boot.o kernel.o main.o
	$(LD) -T linker.ld -o $@ $^

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
