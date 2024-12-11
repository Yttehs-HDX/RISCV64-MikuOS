.PHONY : all build test clean setup

all:
	@cd user && make -s all
	@cd kernel && make -s all

build:
	@cd user && make -s all
	@cd kernel && make -s build

test:
	@cd user && make -s build
	@cd kernel && make -s test
	@cd test && make -s all

clean:
	@cd user && make -s clean
	@cd kernel && make -s clean

setup:
	rustup toolchain install nightly-2024-02-03 --profile minimal
	rustup default nightly-2024-02-03
	rustup target add riscv64gc-unknown-none-elf
	rustup component add rust-src
	rustup component add llvm-tools-preview

%:
	@cd kernel && make -s $@
