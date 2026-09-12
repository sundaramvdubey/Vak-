# Vāk

Vāk is an experimental, statically typed systems-language compiler written in Rust. It translates Vāk source code into validated LLVM IR and can build native executables through Clang.

## Website source and self-hosting

The Vāk website is maintained as a separate, dependency-light React/Vite repository so it can be uploaded to a private repository and deployed independently on Cloudflare Pages:

- **Local website source handoff:** `/home/ubuntu/projects/v-k-d353fe15/vak-website-source/`
- **Private website repository:** add its URL here after upload
- **Cloudflare Pages URL:** add the deployed `pages.dev` or custom-domain URL here after deployment

The website source has no Manus analytics, debug collector, Manus storage proxy, generated Manus markers, server requirement, or paid-service dependency. See its `README.md` for the exact Cloudflare Pages settings.

## Learn Vāk in the browser

The repository includes a zero-dependency learner-facing documentation site in [`docs-site/`](docs-site/). Preview it with `python3 -m http.server 8080 --directory docs-site`; it is ready to deploy as a static Cloudflare Pages site. The site is an orientation layer over the authoritative [`LANGUAGE.md`](LANGUAGE.md), [`TUTORIAL.md`](TUTORIAL.md), and [`PROJECT_STATUS.md`](PROJECT_STATUS.md) documents.

The repository is designed for two audiences. A learner can start with the quick-start commands and the language tutorial. A compiler engineer can continue into the architecture, ownership plan, runtime contract, LLVM inspection report, [the browser playground security plan](PLAYGROUND_PLAN.md), and milestone status documents.

> Vāk is a serious compiler foundation, not a finished production language. The current implementation is intentionally explicit about what is implemented and what remains planned.

## Vāk completion progress

The progress bars below distinguish completed documented slices from the broader language vision. A completed milestone does not mean that every future feature related to that milestone exists.

```text
Overall Vāk project             [█████████████░░░░░░░] 68%
Current Milestones 0–5          [████████████████████] 100%
Memory-safety milestone         [███████░░░░░░░░░░░░░] 35%
Learner-facing documentation    [███████████████████░] 95%
LLVM/native verification        [███████████████████░] 95%
```

The unfinished 32% of the overall project includes ownership and borrowing, pointers, lifetimes, destruction, a complete runtime library, modules, FFI, advanced types, executable fuzzing, and production tooling. See [PROJECT_STATUS.md](PROJECT_STATUS.md) for the detailed accounting.

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
| Tests | Rust unit tests, integration tests, native reference tests, LLVM assembly validation, native examples |

Move semantics, lexical borrow regions, pointers, lifetimes, destruction, modules, a complete runtime library, generics, FFI, and production tooling remain planned. The first explicit reference slice is implemented and documented in [OWNERSHIP_BORROWING_PLAN.md](OWNERSHIP_BORROWING_PLAN.md). See [PROJECT_STATUS.md](PROJECT_STATUS.md) for the current completion estimate.

## Quick start on Linux, macOS, or Windows

### 1. Install prerequisites

Vāk requires:

- Git, if cloning the repository.
- Rust stable and Cargo.
- LLVM tools, including `llvm-as`.
- Clang for native executable builds.

On Ubuntu or Debian:

```sh
sudo apt-get update
sudo apt-get install -y git curl build-essential llvm clang
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

On macOS with Homebrew:

```sh
brew install rust llvm
```

Install Clang if it is not already available through Xcode Command Line Tools. If Homebrew LLVM is not on your `PATH`, follow Homebrew's printed instructions or export its `bin` directory.

Windows is covered by CI with the LLVM/Clang toolchain. Install the Rust stable toolchain and LLVM/Clang through the official installers or Chocolatey, then run the same Cargo commands from PowerShell. The first Windows release should still be validated on a native runner before distribution.

### 2. Obtain the repository

Using Git:

```sh
git clone <your-github-repository-url> serious-lang
cd serious-lang
```

Using the supplied archive:

```sh
unzip vak-learner-ready-repository.zip
cd serious-lang
```

### 3. Build and test Vāk

```sh
cargo fmt -- --check
cargo test
```

A successful test run should report all unit and integration tests passing.

### 4. Use the compiler

During development, run the compiler through Cargo:

```sh
cargo run -- help
cargo run -- version
cargo run -- check examples/hello.vak
cargo run -- emit examples/native.vak /tmp/native.ll
cargo run -- build examples/native.vak /tmp/vak-native
```

If you install the binary locally, the same commands become:

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

## Build the compiler for local use

To create an optimized local binary:

```sh
cargo build --release
```

The binary is then located at `target/release/vak`. You can run it directly:

```sh
./target/release/vak check examples/hello.vak
```

To make it available in your personal command path without administrator privileges:

```sh
mkdir -p "$HOME/.local/bin"
cp target/release/vak "$HOME/.local/bin/vak"
export PATH="$HOME/.local/bin:$PATH"
vak version
```

Add the `export PATH=...` line to `~/.bashrc`, `~/.zshrc`, or the shell configuration used by your laptop if you want it to persist across terminal sessions.

## Your first Vāk program

Create a file called `hello.vak`:

```vak
fn main() -> I32 {
  print("hello from my first Vāk program");
  return 0;
}
```

Check it:

```sh
vak check hello.vak
```

Build it:

```sh
vak build hello.vak ./hello
```

Run it:

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

## Recommended learning path

Read the repository in this order:

1. [README.md](README.md), for installation and first programs.
2. [TUTORIAL.md](TUTORIAL.md), for progressive exercises and expected results.
3. [LANGUAGE.md](LANGUAGE.md), for current syntax and semantic decisions.
4. The examples in [examples/](examples/), from `hello.vak` through `aggregates.vak` and `strings.vak`.
5. [ARCHITECTURE.md](ARCHITECTURE.md), for compiler pipeline decisions.
6. [LLVM_SAFETY_INSPECTION.md](LLVM_SAFETY_INSPECTION.md), for generated IR and safety checks.
7. [RUNTIME.md](RUNTIME.md), for the current runtime boundary.
8. [OWNERSHIP_BORROWING_PLAN.md](OWNERSHIP_BORROWING_PLAN.md), for the future memory model.
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
VAK_PRESENTATION_SCRIPT.md         Presentation-ready architecture script
TUTORIAL.md                        Progressive learner exercises
PROJECT_STATUS.md                  Implementation status and completion estimate
ROADMAP.md                         Milestones and deferred work
.github/workflows/ci.yml            GitHub Actions validation
fuzz/README.md                     Fuzzing preparation notes
```

## Troubleshooting

If `cargo` is not found after installing Rust, start a new terminal or run:

```sh
source "$HOME/.cargo/env"
```

If native builds fail with `clang: command not found`, install Clang and verify:

```sh
clang --version
```

If LLVM validation fails with `llvm-as: command not found`, install LLVM and verify:

```sh
llvm-as --version
```

If `vak check` reports a source error, use the filename, line, column, and diagnostic text. The compiler exits with status 1 for source and semantic errors.

## Development commands

```sh
cargo fmt
cargo fmt -- --check
cargo test
cargo run -- check examples/hello.vak
cargo run -- emit examples/native.vak /tmp/native.ll
cargo run -- build examples/native.vak /tmp/vak-native
```

Before submitting a change, update the language specification if semantics changed, add focused tests, run formatting and tests, and update the relevant status documentation.

## Honest limitations

Vāk is not yet 100% complete. It does not currently implement ownership, borrowing, pointers, references, slices, lifetimes, destruction, heap allocation, a complete runtime, modules, imports, generics, pattern matching, FFI declarations, executable fuzzing, an LSP, a formatter, a debugger, or cross-compilation.

The current code is best understood as a thoroughly tested compiler foundation and learning repository. It is suitable for studying compiler stages, writing small Vāk examples, inspecting LLVM IR, and continuing the language-design work. It should not yet be used as a production systems language or as a replacement for a mature memory-safe compiler.

## License

Vāk is distributed under the MIT License. See [LICENSE](LICENSE).
