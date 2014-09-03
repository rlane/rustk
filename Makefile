AS := x86_64-elf-as
LD := x86_64-elf-ld
RUSTC := rustc

all: rustk

run: rustk
	@echo "Hit CTRL-a x to terminate, CTRL-a h for help"
	kvm -kernel rustk \
		-netdev user,id=hostnet0 \
		-device virtio-net-pci,romfile=,netdev=hostnet0 \
		-nographic \
		-debugcon file:debug.log \
		-gdb tcp::1234,server,nowait


%.o: %.rs
	$(RUSTC) -O --target x86_64-unknown-linux-gnu \
		--crate-type staticlib -o $@ \
		--emit obj $< \
		-Z no-landing-pads -Z lto \
		-C relocation-model=static -g \
		--dep-info $@.dep

boot.o: ASFLAGS=--32 -g

rustk: boot.o main.o
	$(LD) -T linker.ld -z max-page-size=4096 --no-warn-mismatch -o $@ $^

iso: rustk.iso
rustk.iso: rustk grub.cfg
	mkdir -p isodir
	mkdir -p isodir/boot
	cp rustk isodir/boot/rustk
	mkdir -p isodir/boot/grub
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o rustk.iso isodir

clean:
	rm -f rustk *.o rustk.iso *.dep

.PHONY: all run iso clean

-include *.dep
