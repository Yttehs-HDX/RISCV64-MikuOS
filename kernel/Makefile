.PHONY : all build run test submit debug connect fmt clean

ARCH := riscv64gc-unknown-none-elf
BUILD_TYPE := release
BIOS := rustsbi.bin
KERNEL := kernel
TARGET := target/$(ARCH)/$(BUILD_TYPE)/$(KERNEL)
FS_IMG := ../user/target/${ARCH}/${BUILD_TYPE}/sdcard.img
TEST_IMG := ../test/sdcard.img
TEST_IMG_COPY := target/$(ARCH)/$(BUILD_TYPE)/sdcard.img
OUTPUT := target/$(ARCH)/$(BUILD_TYPE)/output.log

CARGO := cargo
CARGO_FLAGS := --$(BUILD_TYPE)

QEMU := qemu-system-riscv64
QEMU_FLAGS := -machine virt \
	-bios ../bootloader/$(BIOS) \
	-kernel $(TARGET) \
	-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-nographic

TEST_FLAGS := -machine virt \
	-bios ../bootloader/$(BIOS) \
	-kernel $(TARGET) \
	-drive file=$(TEST_IMG_COPY),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-nographic

GDB := riscv64-elf-gdb
GDB_FLAGS := -ex "file $(TARGET)" \
	-ex "target remote :1234" \
	-ex "set arch riscv:rv64"

all: submit

build:
	@$(CARGO) build $(CARGO_FLAGS)

run: build
	@$(QEMU) $(QEMU_FLAGS)

test-build:
	@$(CARGO) build $(CARGO_FLAGS) --features test

test: test-build
	@cp $(TEST_IMG) $(TEST_IMG_COPY)
	@$(QEMU) $(TEST_FLAGS) | tee $(OUTPUT)

submit: build
	cp $(TARGET) ../kernel-qemu
	cp ../bootloader/$(BIOS) ../sbi-qemu
	cp $(TEST_IMG) ../sdcard.img

debug:
	@echo -e "\033[33m[+] Run 'make connect' in another terminal at the same dir\033[0m"
	@$(QEMU) $(TEST_FLAGS) -S -s

connect:
	@$(GDB) $(GDB_FLAGS)

fmt:
	@$(CARGO) fmt

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)
