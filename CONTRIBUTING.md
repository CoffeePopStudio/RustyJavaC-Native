# Contributing to RustyJavaC Native

Thanks for being interested in contributing! This crate provides the C-compatible FFI layer for RustyJavaC, making the compiler accessible from Java via Panama FFM or JNA.

## Getting Started

### Prerequisites

- **Rust 1.85+** (we use the 2024 edition)

### Building

```bash
git clone https://github.com/CoffeePopStudio/RustyJavaC-Native.git
cd RustyJavaC-Native
cargo build
```

For release:

```bash
cargo build --release
```

## Project Structure

```
├── Cargo.toml          # cdylib target, git dependency on javac-compiler
├── src/
│   └── lib.rs          # extern "C" FFI exports
├── rustyjavac.h        # C header reference for consumers
├── .gitignore
├── README.md
└── LICENSE
```

## Development Workflow

1. **Fork and clone** the repo.
2. **Create a branch** from `main`.
3. **Make your changes.**
4. **Run `cargo build`** to make sure everything compiles.
5. **Run `cargo clippy`** to lint.
6. **Open a pull request** against `main`.

## What to Work On

- **Richer error reporting** — Currently errors go to stderr. A callback or structured error output would help Java consumers.
- **In-memory compilation** — Accept source as strings rather than file paths.
- **Streaming output** — Return `.class` bytes directly instead of writing to disk.
- **Classpath support** — Expose the classpath parameter via FFI.
- **Cross-platform CI** — Build and test on Windows, Linux, and macOS.

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/). Keep subject lines short and lowercase.

```
feat: add in-memory compilation support
fix: handle null output_dir correctly
```

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE), same as the rest of the project.
