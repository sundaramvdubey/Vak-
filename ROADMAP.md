# Roadmap

- **Milestone 0 — Specification:** complete: scope, syntax, type/memory/error decisions and acceptance examples are documented.
- **Milestone 1 — Frontend foundation:** complete: lexer, spans, parser, AST, syntax diagnostics, and tests.
- **Milestone 2 — Semantics:** complete: scopes, name resolution, types, function checking, assignment checking, operator checking, and semantic diagnostics.
- **Milestone 3 — Code generation:** current: textual LLVM IR generation and `emit`/`build` commands for the supported scalar subset.
- **Milestone 4 — Robustness:** complete for the implemented frontend and backend subset: control-flow and fixed aggregate lowering, native end-to-end examples, LLVM assembly validation, negative semantic tests, and regression tests.
- **Milestone 5 — Runtime completion slice:** complete: for-range LLVM lowering and the minimal null-terminated string runtime through `print(String)`/`puts`.
- **Memory-safety slice:** partially complete: definite-initialization checks with if-branch joins, mutability checks, I32 literal overflow rejection, fixed-array runtime bounds traps, and the first explicit `&T`/`&mut T` reference lowering slice are implemented; move states, lexical borrow regions, pointers, lifetimes, destruction, and full path-sensitive dataflow remain future work.
- **Tooling preparation:** partially complete: GitHub Actions CI and fuzzing guidance/corpus planning are included; an executable fuzz target and full release pipeline remain future work.

Richer string operations, richer aggregate values, move/borrow enforcement beyond explicit reference parameters, fuzzing, and future features such as generics, concurrency, package management, C bindings, WebAssembly, and cross compilation remain deferred.
