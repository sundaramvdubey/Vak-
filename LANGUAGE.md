# Vāk Language Specification

Vāk is a small, statically and strongly typed systems-oriented language. This document is the authority for the current milestone.

## v0.1 scope

Milestone 1 defines syntax only. The accepted surface syntax includes declarations (`let`, `let mut`), functions, blocks, `if`/`else`, `while`, `for name in start..end`, `break`, `continue`, `return`, literals, arrays, structs, field access, indexing, calls, and the operators `+ - * / % == != < <= > >= && || ! =`. Types are named (`Bool`, `I32`, `U32`, `F64`, `String`), arrays (`[T; N]`), and named structs. Semantic checking and code generation are deliberately deferred to later milestones.

A program is a sequence of declarations. Blocks use braces and whitespace has no syntactic meaning. Statements end at a newline or semicolon; braces also delimit statements. Comments begin with `//` and continue to the end of the line.

Milestone 5 defines `for name in start..end` as an ascending half-open range: the loop variable takes `start`, then increments by one while it is less than `end`. The current backend requires both bounds to be `I32`. The initial string runtime represents `String` as a pointer to a null-terminated UTF-8 byte sequence and provides the built-in `print(String)` function through the platform C `puts` ABI.

## Examples

```vak
fn main() {
  let message: String = "hello, Vāk";
  print(message);
}

fn add(a: I32, b: I32) -> I32 { return a + b; }

fn control() {
  let mut total = 0;
  while total < 10 { total = total + 1; }
  if total == 10 { print("done"); } else { print("bad"); }
  for item in 0..10 { print(item); }
}

struct Sample { id: U32, value: F64 }
let readings: [F64; 4] = [1.0, 2.0, 3.0, 4.0];
let current = readings[0];
```

Modules, errors, FFI, and telemetry direction are documented but not implemented in Milestone 1. Future modules will use explicit `mod`/`use`; errors will be explicit returned values or a later documented mechanism; FFI will cross an explicit unsafe C ABI boundary; a telemetry record may be `struct Sample { timestamp_ns: U64, channel: U32, value: F64 }`.

## Memory and type model decisions

Vāk v0.1 uses value semantics. Locals live in compiler-managed stack storage; heap allocation is not yet part of the language. There is no implicit null, ownership system, reference, pointer, slice, destructor, or unsafe operation in v0.1. The implemented safety subset rejects reads of definitely uninitialized locals, enforces immutable versus mutable assignments, and emits runtime traps for out-of-bounds fixed-array indexing. These checks are not a complete memory-safety proof: control-flow-sensitive initialization joins, integer overflow, aggregate moves, and pointer safety remain future work. Later pointer/FFI work must specify aliasing, lifetimes, ABI layout, and boundaries before implementation. Function returns are values.

Integer literals are unsuffixed syntax nodes until semantic analysis. Numeric inference and overflow rules are deferred to Milestone 2 and must reject lossy implicit conversions. Equality and ordering require compatible types; arrays and structs are nominally typed. No implicit numeric conversion is permitted; explicit conversion syntax will be specified before it is implemented.

## Diagnostics

User errors are returned as deterministic diagnostics with filename, line, column, and source span. Malformed input never intentionally panics. Compiler panics indicate implementation bugs.

## Non-goals

Generics, ownership checking, concurrency, async I/O, package management, C/C++ parsing, optimization, and a standard library are not v0.1 requirements.
