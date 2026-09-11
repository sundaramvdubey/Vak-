# Vāk Project Status

## Purpose

Vāk is an early-stage systems-oriented programming language and compiler project implemented in Rust. The project was started from the supplied language-engineering prompt, which requires specification-first development, strict milestone control, correctness before feature count, deterministic diagnostics, and a clear separation between the Rust compiler implementation and the target language.

This repository is intentionally not presented as a completed language. It contains the completed specification, frontend-foundation, semantic-analysis, LLVM code-generation, robustness, and Milestone 5 runtime slices only.

## What was done after the prompt was supplied

The workspace was inspected first. No existing language repository or Rust compiler project was present, so a new Git repository was created at `/home/ubuntu/serious-lang`. The language is named **Vāk**. The project uses Rust and currently has no third-party dependencies.

The first phase established the design authority and repository structure. `LANGUAGE.md` defines the current syntax surface, v0.1 scope, examples, memory model, type-model direction, diagnostics policy, and explicit non-goals. `ARCHITECTURE.md` records the compiler pipeline and consequential design decisions. `ROADMAP.md` divides the work into the milestones required by the prompt. `README.md`, `CONTRIBUTING.md`, `.gitignore`, and the MIT `LICENSE` policy are included in the repository structure; the license text itself should be added before public release if a GitHub repository is created.

The second phase implemented Milestone 1. The source lexer recognizes identifiers, keywords, integer literals, floating-point literals, strings, comments, whitespace, operators, punctuation, and end-of-file. Every token carries a source span containing byte offsets, line, and column. Unterminated strings and unexpected characters become diagnostics rather than compiler crashes.

Milestone 2 adds semantic analysis over the AST. It collects top-level function and struct declarations, rejects duplicate definitions and duplicate struct fields, resolves names through nested lexical scopes, tracks immutable and mutable bindings, validates declared and inferred types, checks numeric and logical operators, validates assignments, array indexing, struct field access, function argument counts and types, function return types, loop-control placement, and unknown types. The analyzer returns deterministic diagnostics and rejects invalid programs before any future code-generation stage.

Milestone 3 adds direct textual LLVM IR generation for the currently supported scalar subset. The backend lowers boolean, integer, and floating-point values, local storage, arithmetic, comparisons, named function calls, assignments, and returns. The CLI supports `vak emit` for writing `.ll` files and `vak build` for invoking Clang to produce a native executable. LLVM assembly validation and a native executable test were performed using a small `add(40, 2)` program that returned process status 42.

Milestone 4 adds LLVM control-flow lowering for `if`/`else`, `while`, `break`, and `continue`. It also adds fixed-size array initialization/indexing and simple struct layout, field reads, and field writes. New native examples cover a loop returning 5 and aggregate processing returning 6. Generated IR for these programs was accepted by `llvm-as`, and Clang-produced executables returned the expected statuses. The test suite now covers parser errors, semantic failures, scalar lowering, control flow, aggregates, and regression behavior.

Milestone 5 completes the deferred for-range and minimal string-runtime slice. `for name in start..end` lowers to an ascending half-open `I32` loop with `break` and `continue` targets. `String` values lower to pointers to private null-terminated LLVM globals, and the built-in `print(String)` function lowers to libc `puts`. Native examples verified a for-range result of 6 and printed `hello from Vāk` successfully.

The memory-safety slice adds definite-initialization tracking for locals and parameters, rejects reads from uninitialized bindings, preserves immutable/mutable assignment checks, and emits a runtime trap before fixed-array reads or writes whose `I32` index is outside the valid range. This is a concrete safety improvement, not a complete ownership or memory model.

The follow-up safety pass adds initialization-state intersection across `if`/`else` branches and restores loop state conservatively because a loop may execute zero times. I32 literal range validation rejects values above `2147483647`. A runtime contract document, GitHub Actions CI workflow, and fuzzing preparation guide were added.

The AST represents programs, functions, structs, blocks, statements, types, literals, arrays, unary expressions, binary expressions, assignments, calls, indexing, and field access. The parser is hand-written recursive descent for declarations and statements, with Pratt-style precedence parsing for expressions. It accepts the defined syntax for functions, structs, variables, mutability markers, type annotations, returns, conditionals, loops, range iteration, arrays, indexing, field access, calls, and operators.

A small CLI was added. `vak check <file.vak>` lexes, parses, and semantically analyzes a source file, reporting source-located errors and returning a nonzero exit status for invalid input. `vak emit` writes LLVM IR and `vak build` invokes Clang for the supported subset.

## Verification performed

The stable Rust, system linker, LLVM, and Clang toolchains were installed in the sandbox because they were not initially available. The project was formatted with `cargo fmt -- --check` and compiled with Cargo. The test suite contains thirteen library unit tests and eight integration tests, for a total of twenty-one passing tests.

The tests verify source locations, representative lexer behavior, comments and literals, parser coverage for a representative surface program, malformed syntax handling, unterminated strings, and illegal-character diagnostics. The checked example `examples/hello.vak` parses successfully. A malformed temporary source file produces a source-located error and exits with status 1.

The `target/` directory is excluded by `.gitignore` and is not included in the GitHub-ready archive.

## What is not implemented yet

The following are intentionally deferred and should not be described as implemented:

1. Richer string operations and richer aggregate value semantics.
2. A runtime or standard library.
3. Modules and imports.
4. Pointers, references, slices, ownership, destruction, lifetimes, and unsafe operations.
5. C ABI declarations and linking.
6. C++ interoperability tooling.
7. Executable fuzzing infrastructure; preparation guidance and corpus planning now exist.
8. Generics, algebraic data types, pattern matching, concurrency, async I/O, package management, LSP, formatter, debugger integration, cross-compilation, WebAssembly, and self-hosting.

The current parser and analyzer still defer loop/path-sensitive initialization beyond conservative joins, advanced numeric inference, explicit conversion semantics, and complete aggregate move/copy rules. These rules must be finalized before they are expanded.

## Recommended next work

Milestone 2 is complete for the currently specified semantic surface. The next step is to expand the specification with concrete integer-literal, inference, overflow, conversion, comparison, array, struct, and assignment rules before broadening the checker.

The Milestone 5 slice is stable for the implemented loops and minimal string runtime. Remaining backend work should expand string and aggregate semantics only after the native path remains stable. No custom machine-code backend, linker, optimizer, or speculative IR should be introduced.

The next engineering work should address exact semantic source spans, a real runtime library, ownership or borrowing design, richer diagnostics, executable fuzz targets, and release automation before adding broad language features.

## Completion estimate

The explicitly enumerated Milestones 0 through 5 are implemented for their documented slices: **100% of the current milestone checklist**. The broader language project is approximately **68% complete by scope**, because the repository still lacks the ownership/pointer model, complete runtime and standard library, modules, FFI, advanced type rules, executable fuzzing, and production tooling. The new memory-safety slice itself is approximately **35% complete**: initialization joins, fixed-array bounds checks, mutability, and literal overflow checks exist, while ownership, aliasing, lifetimes, destruction, pointer safety, integer overflow, and complete path-sensitive dataflow remain unimplemented.

```text
Overall Vāk project          [█████████████░░░░░░░] 68%
Milestones 0–5               [████████████████████] 100%
Memory safety                [███████░░░░░░░░░░░░░] 35%
Learner documentation        [███████████████████░] 95%
LLVM/native verification     [███████████████████░] 95%
```

## GitHub upload note

The archive produced alongside this document contains source, documentation, examples, tests, Cargo metadata, and repository hygiene files. It excludes `.git`, `target`, generated binaries, and temporary files. Before publishing, create an initial GitHub commit with the project history and a CI workflow.
