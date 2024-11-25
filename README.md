# RISC-V64-MikuOS

RISC-V64 OS written in Rust

## Workflows

[![mirror](https://github.com/Yttehs-HDX/RISCV64-MikuOS/actions/workflows/mirror.yml/badge.svg)](https://github.com/Yttehs-HDX/RISCV64-MikuOS/blob/main/.github/workflows/mirror.yml)

## Setup

```bash
make setup
```

## Usage

### Build

```bash
make build
```

### Run

```bash
make run LOG=<log_level>
# or simply 'make'
```

> log_level options: TRACE, DEBUG, INFO, WARN, ERROR

### Debug

```bash
make debug
```

At the same dir:

```bash
make connect
```

## License

MIT
