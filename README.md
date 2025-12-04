# TCP Port Scanner

![crates.io](https://img.shields.io/badge/crate-v0.1.0-blue.svg)
![license](https://img.shields.io/badge/license-MIT-lightgrey.svg)

A fast, concurrent TCP port scanner written in Rust for learning, testing, and lightweight security auditing.

Why this project
- Fast concurrent scanning using Rust async/concurrency primitives.
- Configurable timeouts, concurrency and rate limiting.
- Small, dependency-light command-line tool suitable for scripting and integration in test environments.

Key features
- Scan a single port or a range of ports.
- Configurable: `--concurrency`, and `--rate-limit`.
- Outputs a simple scan summary.

Quick start

Prerequisites
- Rust and Cargo (1.70+ recommended). Install from https://rustup.rs if needed.

Clone and build

```bash
git clone https://github.com/michaeljudge75/TCP-Port-Scanner.git
cd TCP-Port-Scanner/tcp-port-scanner
cargo build --release
```

Run (examples)

- Scan a single port (SSH) on localhost:

```bash
cargo run -- --target 127.0.0.1 --port-single 22 --rate-limit 100
```

- Scan a port range on a host (first 1024 ports):

```bash
cargo run -- --target example.com --port-start 1 --port-end 1024 --concurrency 200 --timeout-ms 500 --rate-limit 200
```

- Use `--mode` to select the scanning implementation (`connect` or `timed`):

```bash
cargo run -- --target 192.168.1.1 --port-start 1 --port-end 1024 --mode timed --rate-limit 100
```

CLI options reference
- `--target` (string): Target host to scan (default: `127.0.0.1`).
- `--port-single` (u32): Scan only this single port.
- `--port-start` (u32): Starting port of range.
- `--port-end` (u32): Ending port of range.
- `--timeout-ms` (u64): Connection timeout in milliseconds.
- `--concurrency` (usize): Max concurrent scans.
- `--rate-limit` (u64): Limit scans per second (required at runtime in current build).

Project layout

- `tcp-port-scanner/` — main crate
  - `src/` — implementation source
  - `src/main.rs` — CLI entrypoint
  - `src/cli.rs` — argument parsing and validation
  - `src/scan_engine.rs` — core scan logic
  - `tests/` — integration and unit tests

Development

- Run tests:

```bash
cd tcp-port-scanner
cargo test
```

- Format code (rustfmt):

```bash
cargo fmt
```

Where to get help
- Open an issue on this repository for bugs or feature requests.
- For questions about using the tool, open a discussion or issue and tag `@michaeljudge75`.

Maintainers & contributing
- Maintainer: Michael Judge (<judgemichael04@gmail.com>) — listed in `tcp-port-scanner/Cargo.toml`.
- Contributions welcome: please open a pull request or issue. Please use the repository issue/PR workflow.

Suggested contribution workflow
- Fork the repo and create a feature branch.
- Write tests for new behavior and ensure `cargo test` passes.
- Submit a pull request with a clear description of the change.

License
- This project is released under the terms of the `LICENSE` file (MIT).

Acknowledgements
- Built with `clap` for CLI parsing
# and `serde` for internal data handling.
<!-- ⚠️ This README has been generated from the file(s) "blueprint.md" ⚠️-->
[![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/colored.png)](#port-scanner)


