.PHONY : all build clean setup

all:
	@cd user && make -s all
	@cd kernel && make -s all

build:
	@cd user && make -s all
	@cd kernel && make -s build

clean:
	@cd user && make -s clean
	@cd kernel && make -s clean

setup:
	rustup target add riscv64gc-unknown-none-elf
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

%:
	@cd kernel && make -s $@