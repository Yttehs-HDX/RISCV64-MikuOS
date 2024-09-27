# RISC-V64-MikuOS

RISC-V64 OS written in Rust

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