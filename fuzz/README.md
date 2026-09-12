# Vāk fuzzing

The `frontend` target exercises parsing, semantic analysis, and LLVM generation while enforcing the invariant that arbitrary input must not panic the compiler.

Install cargo-fuzz once:

```sh
cargo install cargo-fuzz
```

Run a bounded local smoke campaign:

```sh
cargo fuzz run frontend -- -runs=1000
```

The fuzz target deliberately stops after parse or semantic errors because invalid programs are expected. Any panic, abort, or sanitizer failure is a compiler bug and should be preserved as a regression test under `tests/`.
