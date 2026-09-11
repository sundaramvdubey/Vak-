# Architecture

The compiler pipeline is `source -> lexer -> parser -> AST -> name resolution -> type checking -> semantic analysis -> LLVM/native codegen`.

Milestones 3 through 5 use a small textual LLVM backend. The backend lowers scalar `Bool`, integer, and `F64` values, local bindings, arithmetic, comparisons, calls, assignments, returns, `if`/`else`, `while`, `for`, `break`, `continue`, fixed-size arrays, simple struct field access, string literals, and `print(String)` through libc `puts`. `vak emit` writes LLVM IR and `vak build` invokes the system Clang driver. Richer string operations and aggregate values remain deferred; no fake IR is emitted for them. No custom machine-code backend or linker is introduced. Rust is the implementation language, not Vāk semantics.

The current safety boundary is deliberately narrow. Semantic analysis tracks definite initialization for local reads and mutability for writes. LLVM array access emits an unsigned index comparison and branches to `llvm.trap` on failure. This protects the implemented fixed-array operations but does not establish ownership, aliasing, lifetime, overflow, or path-sensitive dataflow guarantees.

Each frontend error is a value. The CLI returns exit code 1 for user input errors and reserves panics for compiler bugs. There are no third-party dependencies in the foundation.

## Design decision record

**Decision:** Use value semantics and defer references/ownership/pointers.

**Context:** v0.1 must be understandable and compatible with eventual C ABI work without pretending to be memory safe.

**Chosen approach:** stack-oriented values, no implicit null, no heap or pointer syntax in the current milestone.

**Alternatives considered:** a partial borrow checker or a Rust-like ownership hybrid.

**Why this approach:** partial ownership would create inconsistent guarantees and disproportionate complexity.

**Consequences:** v0.1 is not memory-safe and cannot yet express FFI; later milestones must define those semantics explicitly.

**Decision:** Use no custom IR in Milestone 1.

**Context:** there is no demonstrated transformation requiring one yet.

**Chosen approach:** retain a clean AST boundary and add an IR only when code generation benefits from it.

**Alternatives considered:** implement a speculative SSA/custom IR now.

**Why this approach:** less code and fewer semantic translation bugs.

**Consequences:** LLVM lowering will be designed in the code-generation milestone.
