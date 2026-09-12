# Vāk

> **A small, statically typed systems-language compiler written from scratch in Rust.**

[![CI](https://img.shields.io/github/actions/workflow/status/sundaramvdubey/Vak-/ci.yml?label=CI&style=flat-square)](https://github.com/sundaramvdubey/Vak-/actions)
[![Language](https://img.shields.io/badge/language-Rust-162233?style=flat-square)](https://www.rust-lang.org/)
[![Backend](https://img.shields.io/badge/backend-LLVM-D59B48?style=flat-square)](LLVM_SAFETY_INSPECTION.md)
[![License](https://img.shields.io/badge/license-MIT-365746?style=flat-square)](LICENSE)

Vāk translates source code into validated LLVM IR and can build native executables through Clang. The project is designed as both a working compiler foundation and a transparent language-design laboratory: each implemented slice is documented, tested, and separated from the future roadmap.

## See the pipeline in one minute

```text
.vak source → lexer → parser / AST → names + types → LLVM IR → native executable
```

```vak
fn main() -> I32 {
  print("hello from Vāk");
  return 0;
}
```

```sh
cargo run -- check examples/hello.vak
cargo run -- build examples/hello.vak ./hello
./hello
```

## Current milestone

The documented Milestones 0–5 are implemented for their current slices. The broader project is approximately **70% complete by scope**. The memory-safety work is intentionally still in progress; full ownership, lifetimes, destruction, modules, FFI, and production tooling remain future work.

```text
Current Milestones 0–5       [████████████████████] 100%
Broader language scope       [██████████████░░░░░░] 70%
Memory-safety milestone      [████████████░░░░░░░░] 60%
```

## Why Vāk is worth studying

- It has a real frontend: lexer, spans, parser, AST, and syntax diagnostics.
- It performs name resolution, type checking, mutability checks, initialization checks, and reference/borrow diagnostics.
- It emits textual LLVM IR, validates it, and can produce native executables.
- It includes examples, integration tests, LLVM checks, fuzzing foundations, and cross-platform CI targets.
- It is honest about what is not implemented yet.

## Learn and inspect

- **[Start with the tutorial](TUTORIAL.md)** for progressive exercises.
- **[Read the language authority](LANGUAGE.md)** for current syntax and semantics.
- **[Trace the architecture](ARCHITECTURE.md)** to understand the compiler pipeline.
- **[Inspect generated LLVM and safety checks](LLVM_SAFETY_INSPECTION.md)**.
- **[Read the ownership plan](OWNERSHIP_BORROWING_PLAN.md)** for the next major language-design boundary.
- **[Review current status](PROJECT_STATUS.md)** before assuming a feature exists.

## What works today

The current compiler supports the following language and compiler features:

| Area | Current support |
|---|---|
| Frontend | Lexer, source spans, parser, AST, syntax diagnostics |
| Semantics | Name resolution, lexical scopes, type checking, mutability checks |
| Safety | Definite initialization, `if`-branch initialization joins, I32 literal range checks, fixed-array runtime bounds traps, explicit shared/mutable references, shared-reference mutation rejection, reference-return rejection |
| Control flow | `if`, `else`, `while`, `for start..end`, `break`, `continue`, `return` |
| Data | `Bool`, `I32`, `U32`, `U64`, `F64`, `String`, fixed arrays, simple structs, `&T` and `&mut T` reference parameters |
| Backend | Textual LLVM IR generation, LLVM validation, Clang native builds |
| Runtime slice | String literals and `print(String)` through libc `puts` |
| Tests | Rust unit tests, 19 integration tests, ownership/diagnostic tests, native reference tests, LLVM assembly validation, native examples |

Full place-based move semantics, lexical lifetime inference, pointers, destruction, modules, a complete runtime library, generics, FFI, and production tooling remain planned. The initial move/borrow foundation is implemented and documented in [OWNERSHIP_BORROWING_PLAN.md](OWNERSHIP_BORROWING_PLAN.md). See [PROJECT_STATUS.md](PROJECT_STATUS.md) for the current completion estimate.

## Environment and command reference for Linux, macOS, and Windows

### Prerequisite inventory

Vāk requires:

- Git, if cloning the repository.
- Rust stable and Cargo.
- LLVM tools, including `llvm-as`.
- Clang for native executable builds.

A representative Ubuntu or Debian environment uses:

```sh
sudo apt-get update
sudo apt-get install -y git curl build-essential llvm clang
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

A macOS environment using Homebrew is represented by:

```sh
brew install rust llvm
```

Clang is supplied by Xcode Command Line Tools or another LLVM installation. Homebrew LLVM may require its `bin` directory to be present on `PATH`.

Windows is covered by CI with the LLVM/Clang toolchain. The supported environment consists of the Rust stable toolchain and LLVM/Clang from official installers or Chocolatey; the Cargo command reference is compatible with PowerShell. A native Windows runner remains part of release validation.

### Repository acquisition examples

A Git-based acquisition is represented by:

```sh
git clone <repository-url> serious-lang
cd serious-lang
```

An archive-based acquisition is represented by:

```sh
unzip vak-learner-ready-repository.zip
cd serious-lang
```

### Build and test command reference

```sh
cargo fmt -- --check
cargo test
```

A successful test run reports all unit and integration tests as passing.

### Compiler command reference

During development, Cargo provides the compiler entry point:

```sh
cargo run -- help
cargo run -- version
cargo run -- check examples/hello.vak
cargo run -- emit examples/native.vak /tmp/native.ll
cargo run -- build examples/native.vak /tmp/vak-native
```

A locally installed binary exposes the same command surface:

```sh
vak help
vak check examples/hello.vak
vak build examples/native.vak /tmp/vak-native
```

The command exit codes are stable:

| Code | Meaning |
|---:|---|
| 0 | Successful check, IR emission, or native build |
| 1 | Source, semantic, or code-generation failure |
| 2 | CLI usage, file, or external-tool failure |

## Local compiler binary

An optimized local binary is produced by:

```sh
cargo build --release
```

The resulting binary is located at `target/release/vak`; a direct invocation is represented by:

```sh
./target/release/vak check examples/hello.vak
```

A user-local command-path placement without administrator privileges is represented by:

```sh
mkdir -p "$HOME/.local/bin"
cp target/release/vak "$HOME/.local/bin/vak"
export PATH="$HOME/.local/bin:$PATH"
vak version
```

Persistence across terminal sessions depends on the shell configuration, such as `~/.bashrc` or `~/.zshrc`.

## First-program example

The following source represents a file named `hello.vak`:

```vak
fn main() -> I32 {
  print("hello from my first Vāk program");
  return 0;
}
```

The corresponding check command is:

```sh
vak check hello.vak
```

The corresponding native-build command is:

```sh
vak build hello.vak ./hello
```

The resulting executable is represented by:

```sh
./hello
```

Expected output:

```text
hello from my first Vāk program
```

The current `print` implementation uses the platform C `puts` function. It is a bootstrap runtime facility, not yet the final Vāk standard library.

## Small language tour

### Bindings and mutability

Bindings are immutable by default. Use `let mut` when assignment is required:

```vak
fn main() -> I32 {
  let mut count: I32 = 0;
  count = count + 1;
  return count;
}
```

Reading a binding before initialization is rejected:

```vak
fn main() -> I32 {
  let value: I32;
  return value;
}
```

### Conditions and loops

```vak
fn main() -> I32 {
  let mut total: I32 = 0;
  while total < 3 {
    total = total + 1;
  }

  for item in 0..3 {
    print("loop iteration");
  }

  if total == 3 {
    return 0;
  } else {
    return 1;
  }
}
```

A range is ascending and half-open. `0..3` visits `0`, `1`, and `2`.

### Arrays

Arrays have a fixed length and an element type:

```vak
fn main() -> I32 {
  let values: [I32; 3] = [10, 20, 30];
  return values[1];
}
```

The compiler emits a runtime bounds check before each array read and write. An invalid access calls `llvm.trap` rather than silently reading arbitrary memory.

### Structs

```vak
struct Point { x: I32, y: I32 }

fn main() -> I32 {
  let mut point: Point;
  point.x = 10;
  point.y = 20;
  return point.x + point.y;
}
```

The current backend supports simple struct storage and field access. Rich aggregate copying, passing, returning, and nested place analysis remain future work.

## Compiler commands

```text
vak help
```

Prints command documentation and exit-code behavior.

```text
vak check <file.vak>
```

Lexes, parses, resolves, and semantically checks a Vāk file without producing an executable.

```text
vak emit <file.vak> [output.ll]
```

Checks the source and writes textual LLVM IR. If the output path is omitted, the compiler writes `<file.vak>.ll`.

```text
vak build <file.vak> [output]
```

Checks the source, emits temporary LLVM IR, invokes Clang, and produces a native executable. If the output path is omitted, the executable is named `a.out`.

```text
vak version
```

Prints the compiler version and backend label.

## Documentation sequence

The documentation sequence is:

1. [README.md](README.md), for environment details and first-program examples.
2. [TUTORIAL.md](TUTORIAL.md), for progressive exercises and expected results.
3. [LANGUAGE.md](LANGUAGE.md), for current syntax and semantic decisions.
4. The examples in [examples/](examples/), from `hello.vak` through `aggregates.vak` and `strings.vak`.
5. [ARCHITECTURE.md](ARCHITECTURE.md), for compiler pipeline decisions.
6. [LLVM_SAFETY_INSPECTION.md](LLVM_SAFETY_INSPECTION.md), for generated IR and safety checks.
7. [RUNTIME.md](RUNTIME.md), for the current runtime boundary.
8. [OWNERSHIP_BORROWING_PLAN.md](OWNERSHIP_BORROWING_PLAN.md), for the ownership and borrowing model.
9. [PROJECT_STATUS.md](PROJECT_STATUS.md) and [ROADMAP.md](ROADMAP.md), for honest scope and next milestones.

## Repository layout

```text
src/lib.rs                         Lexer, parser, AST, semantics, LLVM backend
src/main.rs                        Vāk command-line compiler
examples/                          Small runnable Vāk programs
tests/frontend.rs                  Integration and backend tests
LANGUAGE.md                        Language authority and syntax rules
ARCHITECTURE.md                    Compiler architecture and decisions
RUNTIME.md                         Current runtime ABI contract
OWNERSHIP_BORROWING_PLAN.md        Planned memory model
LLVM_SAFETY_INSPECTION.md          Generated IR inspection report
TUTORIAL.md                        Progressive learner exercises
PROJECT_STATUS.md                  Implementation status and completion estimate
ROADMAP.md                         Milestones and deferred work
.github/workflows/ci.yml            GitHub Actions validation
fuzz/README.md                     Executable fuzz target and bounded smoke guidance
```

## Diagnostic reference

When `cargo` is not present after a Rust installation, the relevant shell initialization is:

```sh
source "$HOME/.cargo/env"
```

When native builds report `clang: command not found`, the relevant executable check is:

```sh
clang --version
```

When LLVM validation reports `llvm-as: command not found`, the relevant executable check is:

```sh
llvm-as --version
```

A `vak check` source error contains a filename, line, column, and diagnostic message. The compiler exits with status 1 for source and semantic errors.

## Development command reference

```sh
cargo fmt
cargo fmt -- --check
cargo test
cargo run -- check examples/hello.vak
cargo run -- emit examples/native.vak /tmp/native.ll
cargo run -- build examples/native.vak /tmp/vak-native
```

A repository change that alters semantics is accompanied by a specification update, focused tests, formatting and test results, and corresponding status documentation.

## Honest limitations

Vāk is not yet 100% complete. It does not currently implement full place-based ownership, pointers, slices, non-lexical lifetimes, destruction, heap allocation, a complete runtime, modules, imports, generics, pattern matching, FFI declarations, sanitizer fuzzing automation, an LSP, a formatter, a debugger, or cross-compilation.

The current code is best understood as a thoroughly tested compiler foundation and learning repository. It is suitable for studying compiler stages, writing small Vāk examples, inspecting LLVM IR, and continuing the language-design work. Production systems use and replacement of a mature memory-safe compiler remain outside the current scope.

## License

Vāk is distributed under the MIT License. See [LICENSE](LICENSE).
