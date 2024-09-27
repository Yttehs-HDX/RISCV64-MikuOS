.PHONY : all

all:
	@cd kernel && make -s all

setup:
	rustup target add riscv64gc-unknown-none-elf
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

%:
	@cd kernel && make -s $@