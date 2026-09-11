# Vāk Compiler Presentation Script

**Title:** Vāk: A Specification-Vākst Systems Language Compiler  
**Format:** 12-slide presentation script with speaker notes  
**Audience:** Compiler engineers, systems programmers, and technical reviewers  
**Estimated duration:** 15–20 minutes  
**Author:** Manus AI

## Slide 1 — Vāk in one sentence

**On-slide content:**

> Vāk is a Rust-based systems-language compiler foundation that translates a small statically typed language into validated LLVM IR and native executables.

**Speaker script:**

Vāk is an experimental systems-oriented programming language and compiler. The project began with a specification-first approach. The compiler is implemented in Rust, while Vāk is the target language. The implementation currently covers the frontend, semantic analysis, a textual LLVM backend, control flow, fixed aggregates, a minimal string runtime, and an initial safety layer.

The central engineering principle is to implement a narrow slice correctly before expanding the language surface. Features that are not implemented are documented as deferred rather than simulated with unsafe placeholders.

## Slide 2 — Current completion status

**On-slide content:**

| Measure | Status |
|---|---:|
| Milestones 0–5 documented slices | 100% |
| Broader Vāk language project | Approximately 68% |
| Memory-safety slice | Approximately 35% |
| Verified Rust tests | 21 passing |

**Speaker script:**

Vāk is not 100% complete as a language project. The explicitly enumerated Milestones 0 through 5 are complete for their documented slices. The broader project is approximately 68% complete by scope. The memory-safety slice is approximately 35% complete.

The remaining work is substantial. It includes ownership, borrowing, pointers, lifetimes, destruction, a complete runtime, modules, FFI, executable fuzzing, and production tooling. The percentages are scope estimates, not claims of formal verification.

## Slide 3 — Compiler architecture

**On-slide content:**

```text
Source
  -> Lexer
  -> Parser and AST
  -> Name resolution
  -> Type checking
  -> Semantic safety checks
  -> LLVM lowering
  -> LLVM assembly
  -> Clang native linking
```

**Speaker script:**

The compiler pipeline begins with source text. The lexer produces tokens with source spans. The parser builds an abstract syntax tree. Name resolution and type checking then validate declarations, scopes, operators, calls, assignments, arrays, structs, and control flow.

The current safety checks include initialization tracking, mutability enforcement, integer literal range checks, and generated array bounds checks. The LLVM backend lowers the supported typed AST directly into textual LLVM IR. LLVM validates the IR, and Clang links native executables.

The next architectural improvement is an ownership-aware intermediate representation between semantic analysis and LLVM generation. That representation will make moves, borrows, and destruction explicit.

## Slide 4 — Language surface implemented

**On-slide content:**

- Functions and returns
- `let` and `let mut`
- Scalars and fixed arrays
- Structs and field access
- `if` / `else`
- `while` and `for` ranges
- `break` and `continue`
- String literals and `print(String)`

**Speaker script:**

The implemented language surface is intentionally small. It includes functions, local bindings, scalar values, fixed-size arrays, structs, field access, arithmetic and comparisons, conditionals, while loops, half-open integer ranges, loop control, string literals, and a minimal print operation.

The current type system does not yet include pointers, references, slices, ownership syntax, generic types, modules, or algebraic data types. This distinction matters because the current stack-oriented value model is not the final memory model.

## Slide 5 — Current value and memory model

**On-slide content:**

```text
Locals: compiler-managed stack storage
Heap: not yet exposed as a Vāk feature
Pointers: not yet exposed
Ownership: planned, not implemented
References: planned, not implemented
```

**Speaker script:**

Vāk currently uses value semantics. Local variables lower to compiler-managed stack storage. Heap allocation, pointers, references, destructors, and unsafe operations are not yet part of the language.

This gives the current compiler a simple and predictable baseline, but it is not a complete memory-safety model. The implemented safety checks prevent several classes of mistakes, but they do not prove that arbitrary future programs are memory safe.

## Slide 6 — Safety features implemented today

**On-slide content:**

1. Definite initialization for local reads.
2. Conservative initialization joins across `if` branches.
3. Immutable and mutable assignment checks.
4. I32 literal range validation.
5. Runtime fixed-array bounds traps.

**Speaker script:**

The current safety layer starts with local invariants. Reading a binding before initialization is rejected. If a value is initialized in both branches of an if statement, the value is considered initialized afterward. If only one branch initializes it, the later read is rejected.

The checker also preserves mutability rules. Fixed-array indices are checked at runtime before the generated LLVM GEP instruction. Integer literals above the supported I32 range are rejected during semantic analysis.

These checks are useful, but they are not ownership or borrowing. They do not handle heap aliases, resource destruction, or references.

## Slide 7 — Ownership model proposal

**On-slide content:**

> One owner, many shared borrows, or one exclusive mutable borrow.

```text
Owned -> Moved
Owned -> SharedBorrowed
Owned -> MutablyBorrowed
```

**Speaker script:**

The planned ownership model is based on move semantics and controlled borrowing. A non-copy value has one owner. Moving transfers ownership and makes the original binding unavailable. Shared immutable borrows may coexist. A mutable borrow is exclusive.

The core invariant is that mutation cannot occur while shared aliases exist, and two mutable aliases cannot overlap. This model is designed to prevent use-after-move, double ownership, and mutation through stale aliases.

The first milestone should be narrower than Rust. It should start with local bindings, function parameters, and simple aggregates. It should defer complex lifetime parameters, interior mutability, and self-referential data.

## Slide 8 — Planned borrowing syntax and behavior

**On-slide content:**

```vak
fn inspect(value: &String) { print(*value); }
fn update(value: &mut I32) { *value = *value + 1; }
```

| Form | Meaning |
|---|---|
| `T` parameter | Owned value or copyable value |
| `&T` | Shared immutable borrow |
| `&mut T` | Exclusive mutable borrow |

**Speaker script:**

The proposed syntax uses `&T` for a shared borrow and `&mut T` for an exclusive mutable borrow. An owned argument transfers ownership unless the type is copyable. A shared reference allows reads but no mutation. A mutable reference allows mutation while excluding other accesses.

The initial checker should use lexical borrow regions. A borrow ends at the end of the smallest enclosing statement or block containing its last use. Non-lexical lifetime inference can be considered later, after the basic model is stable.

## Slide 9 — Ownership-aware compiler pipeline

**On-slide content:**

```text
Typed AST
  -> Ownership and borrow checking
  -> Ownership-aware IR
       Move
       Copy
       BorrowShared
       BorrowMut
       Drop
  -> LLVM IR
```

**Speaker script:**

Ownership should be checked before LLVM code generation. Directly embedding ownership decisions into textual LLVM emission would make diagnostics, cleanup, and testing difficult.

The planned intermediate representation makes storage and lifetime actions explicit. It can represent storage becoming live, values being moved or copied, borrows being created, values being used, values being dropped, and storage becoming dead.

This separation allows the ownership checker to be tested independently from LLVM syntax and allows cleanup logic to be verified before heap allocation is introduced.

## Slide 10 — LLVM inspection: strings

**On-slide content:**

```llvm
declare i32 @puts(ptr)
@.str.0 = private unnamed_addr constant [15 x i8] c"hello\20from\20Vāk\00"

%t0 = getelementptr inbounds [15 x i8], ptr @.str.0, i64 0, i64 0
call i32 @puts(ptr %t0)
```

**Speaker script:**

The emitted string IR declares the libc `puts` function and defines a private null-terminated byte array. The compiler obtains a pointer to the first byte with `getelementptr inbounds` and passes that pointer to `puts`.

This confirms that the current string runtime is a minimal ABI bridge. It supports string literals and printing. It does not yet provide string length, concatenation, comparison, allocation, or a portable Vāk runtime library.

## Slide 11 — LLVM inspection: array bounds safety

**On-slide content:**

```llvm
%t5 = icmp ult i32 2, 2
br i1 %t5, label %bounds.ok.3, label %bounds.fail.4

bounds.fail.4:
  call void @llvm.trap()
  unreachable
```

**Speaker script:**

The invalid array example indexes a two-element array with index 2. The generated IR compares the index with the array length using an unsigned less-than comparison. Since 2 is not less than 2, control flows to the failure block.

The failure block calls `llvm.trap` and is marked unreachable. The valid block contains the GEP and load only after the check. The native executable was tested and terminated with a nonzero trap status.

This is runtime bounds safety for the implemented fixed-array operations. It does not replace ownership, pointer safety, or lifetime checking.

## Slide 12 — Completed milestones and next steps

**On-slide content:**

| Milestone | Completion |
|---|---|
| 0 — Specification | Complete |
| 1 — Frontend | Complete |
| 2 — Semantics | Complete for current surface |
| 3 — LLVM backend | Complete for supported subset |
| 4 — Robustness | Complete for implemented subset |
| 5 — Runtime slice | Complete |
| Ownership and borrowing | Planned |

**Speaker script:**

Milestones 0 through 5 are complete for their documented scopes. The project has a verified compiler path from source text to native binaries. The safety layer has initialization checks, mutability checks, literal range checks, and array bounds traps.

The ownership and borrowing milestone is not complete. Its planned phases are move checking, copy classification, shared and mutable borrows, borrow regions, ownership-aware IR, aggregate place analysis, heap destruction, and an explicit unsafe and FFI boundary.

The correct conclusion is that Vāk is a substantial compiler foundation, not a finished production language.

## Closing statement

Vāk is now beyond a parser prototype. It has a working frontend, semantic checker, LLVM backend, runtime bootstrap, safety checks, tests, CI preparation, and a documented ownership plan. It is still not 100% complete because the memory model, ownership system, runtime library, modules, FFI, advanced types, and production tooling remain unfinished.

## References

[1]: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html "The Rust Programming Language: Understanding Ownership"

[2]: https://llvm.org/docs/LangRef.html "LLVM Language Reference Manual"

[3]: https://doc.rust-lang.org/nomicon/ "The Rustonomicon"
