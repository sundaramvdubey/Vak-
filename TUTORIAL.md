# Learn Vāk by Building Small Programs

This tutorial assumes that the repository is installed and that `cargo run -- help` works. If the optimized compiler is installed, replace `cargo run --` with `vak` in every command.

## Exercise 1: Check an existing program

Start with the smallest example:

```sh
vak check examples/hello.vak
```

The `check` command does not build a native executable. It verifies that the source can be lexed, parsed, resolved, and semantically checked.

The compiler prints the number of top-level items when checking succeeds. A source or semantic error exits with status 1.

## Exercise 2: Build and run a program

Build the string example:

```sh
vak build examples/strings.vak ./strings-example
./strings-example
```

The program prints a string and returns zero. The current runtime uses libc `puts`, so the example demonstrates the compiler-to-runtime boundary without hiding it behind a large standard library.

## Exercise 3: Mutable local state

Create `counter.vak`:

```vak
fn main() -> I32 {
  let mut counter: I32 = 0;
  counter = counter + 1;
  counter = counter + 1;
  return counter;
}
```

Build and run it:

```sh
vak build counter.vak ./counter
./counter
printf 'exit status: %s\n' "$?"
```

The process status should be 2. Vāk uses the returned `I32` as the native program exit status in this example.

## Exercise 4: Fixed arrays

```vak
fn main() -> I32 {
  let numbers: [I32; 3] = [4, 5, 6];
  return numbers[1];
}
```

The result is 5. The compiler emits a bounds check before the array access. Try changing `numbers[1]` to `numbers[3]`. The program should trap at runtime instead of silently reading memory outside the array.

## Exercise 5: Conditions and loops

```vak
fn main() -> I32 {
  let mut total: I32 = 0;
  while total < 4 {
    total = total + 1;
  }

  if total == 4 {
    return 0;
  } else {
    return 1;
  }
}
```

The condition is checked as a `Bool`. The loop updates a mutable local. The if statement selects one of two blocks.

## Exercise 6: Half-open ranges

```vak
fn main() -> I32 {
  let mut sum: I32 = 0;
  for item in 0..4 {
    sum = sum + item;
  }
  return sum;
}
```

The range is half-open, so the loop visits `0`, `1`, `2`, and `3`. The result is 6.

## Exercise 7: Struct fields

```vak
struct Point { x: I32, y: I32 }

fn main() -> I32 {
  let mut point: Point;
  point.x = 10;
  point.y = 20;
  return point.x + point.y;
}
```

A struct declaration gives a nominal type. A mutable struct binding can receive field assignments. The current compiler supports simple field access, but advanced aggregate copying and passing remain future work.

## Exercise 8: Read the generated LLVM

Emit LLVM instead of linking a native executable:

```sh
vak emit examples/strings.vak /tmp/strings.ll
cat /tmp/strings.ll
llvm-as /tmp/strings.ll -o /tmp/strings.bc
```

Look for the string global, the `getelementptr` instruction, and the call to `puts`. For an array example, look for `icmp ult`, the success and failure labels, and `llvm.trap`.

## Exercise 9: Trigger useful compiler errors

Uninitialized read:

```vak
fn main() -> I32 {
  let value: I32;
  return value;
}
```

Integer literal overflow:

```vak
fn main() {
  let value: I32 = 2147483648;
}
```

Immutable assignment:

```vak
fn main() {
  let value: I32 = 1;
  value = 2;
}
```

Each example should fail during checking before LLVM generation.

## Where to go next

After completing these exercises, read [LANGUAGE.md](LANGUAGE.md) for formal current semantics. Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand the compiler pipeline. Read [OWNERSHIP_BORROWING_PLAN.md](OWNERSHIP_BORROWING_PLAN.md) to understand the planned memory-safety milestone.

The most important limitation is that Vāk does not yet implement ownership or borrowing. The current safety layer is useful, but it is not equivalent to a complete memory-safe language.
