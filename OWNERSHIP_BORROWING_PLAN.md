# Vāk Ownership and Borrowing Model Plan

**Author:** Manus AI
**Status:** Partially implemented Priority 0 foundation
**Current implementation baseline:** Vāk has value semantics, compiler-managed local storage, definite-initialization checks, mutability checks, fixed-array runtime bounds traps, explicit `&T`/`&mut T` references, dereference lowering, move states for non-copy values, borrow-conflict checks, lexical temporary borrow cleanup, and structured ownership diagnostics. It does not yet have full place-based aggregate moves, non-lexical lifetime inference, heap allocation, destruction, or unsafe syntax.

## Executive conclusion

Vāk should adopt an ownership model based on **move semantics, exclusive mutable borrows, shared immutable borrows, and lexical lifetime regions**. The first implementation should be deliberately narrower than Rust. It should begin with local variables, function arguments, fixed aggregates, and explicit reference types. It should postpone implicit dereferencing, arbitrary pointer arithmetic, self-referential values, interior mutability, and concurrent ownership until the core checker is stable.

The principal safety invariant is the following:

> At every program point, a value has either one owning binding, or a set of immutable aliases, or one exclusive mutable borrow. These states may not overlap in a way that permits mutation through an alias.

This model prevents use-after-move, double ownership, mutation through shared aliases, and references that outlive their referents. It does not by itself prevent integer overflow, data races across threads, denial-of-service allocations, or logic errors. Those concerns require separate specifications.

## 1. Proposed language surface

The first ownership milestone should introduce explicit reference types rather than silently changing the behavior of existing value types. The proposed syntax is:

```vak
fn inspect(value: &String) { print(*value); }
fn update(value: &mut I32) { *value = *value + 1; }

fn main() -> I32 {
  let mut total: I32 = 0;
  update(&mut total);
  inspect(&"done");
  return total;
}
```

The exact dereference syntax may change before implementation. The semantic distinction is more important than the spelling. `&T` is a shared immutable borrow. `&mut T` is an exclusive mutable borrow. A borrow is not an owning value and does not free or destroy the referent.

| Construct | Ownership effect | Mutation permitted | Lifetime requirement |
|---|---|---:|---|
| `let x = value` | Creates the owner of a value | Only if `x` is mutable | Value lives at least as long as `x` |
| `let y = x` | Moves ownership for non-copy values | Through `y` if mutable | `x` becomes unavailable |
| `let y = copy(x)` | Creates an independent value | Independent mutation | No shared lifetime |
| `&x` | Creates a shared borrow | No | Borrow ends before `x` is moved or mutated |
| `&mut x` | Creates an exclusive borrow | Yes, through the borrow | No other access during the borrow |
| `return x` | Moves or copies according to type policy | Caller receives the value | Returned value outlives the callee |

## 2. Value categories and copy policy

Vāk should distinguish **copyable scalar values** from **move-only resource values**. `Bool`, `I32`, `U32`, `U64`, and `F64` should initially be implicitly copyable because their values have no destructor and have a fixed representation. `String`, arrays containing non-copy values, and structs containing non-copy fields should initially be move-only.

A type is copyable only when every field is copyable and the type has no destructor or runtime-managed resource. Copying a move-only value must require an explicit operation, such as `clone`, once the runtime provides that operation. The compiler must never silently duplicate a resource-bearing value.

| Type family | Initial policy | Rationale |
|---|---|---|
| Boolean and fixed-width integers | Copy | Trivial value semantics |
| Floating-point values | Copy | Trivial value semantics, subject to documented NaN rules |
| String | Move-only initially | Future representation may own heap storage |
| Fixed arrays | Copy if all elements are copyable; otherwise move-only | Structural rule is predictable |
| Structs | Copy if all fields are copyable; otherwise move-only | Prevents accidental resource duplication |
| References | Copy as aliases, never owners | Copying a borrow does not copy the referent |

## 3. Binding states

The semantic analyzer should extend each binding with an ownership state. The current `initialized` and `mutable` fields are useful foundations, but they are not sufficient for ownership checking.

```text
Binding {
    type: SemanticType,
    mutable: bool,
    initialized: bool,
    ownership: Owned | Moved | SharedBorrowed(count) | MutablyBorrowed,
    move_kind: Copy | Move,
}
```

The implementation should use an internal state machine rather than expose this representation to the language. A moved binding remains in the symbol table so diagnostics can identify the original declaration and report that the value was moved.

The permitted transitions are:

```text
Uninitialized -> Owned
Owned -> Moved
Owned -> SharedBorrowed
Owned -> MutablyBorrowed
SharedBorrowed -> Owned       when the last shared borrow ends
MutablyBorrowed -> Owned      when the mutable borrow ends
```

The following transitions must be rejected:

```text
Moved -> read or move
SharedBorrowed -> mutable borrow
SharedBorrowed -> mutation through the owner
MutablyBorrowed -> any other read, move, or borrow
Owned -> second mutable borrow
```

## 4. Borrow checking rules

A shared borrow may coexist with other shared borrows. A mutable borrow must be unique. The checker should reject the following examples:

```vak
let mut value: I32 = 1;
let first: &I32 = &value;
let second: &mut I32 = &mut value; // error: shared borrow is active
```

```vak
let mut value: I32 = 1;
let first: &mut I32 = &mut value;
let second: &mut I32 = &mut value; // error: second mutable borrow
```

```vak
let value: String = "owned";
let moved: String = value;
print(value); // error: use after move
```

The first implementation should use **lexical borrow regions**. A borrow ends at the end of the smallest enclosing statement or block that contains its last use. A later refinement may use non-lexical lifetime inference, but the checker should not attempt that optimization before the basic region model is sound.

The checker must distinguish a reference's lifetime from the referent's storage duration. A reference may not escape a function if it points to a local variable whose lifetime ends at function return. Returning a reference to a parameter may be permitted only when the signature explicitly relates the output lifetime to the input lifetime. Explicit lifetime parameters should be deferred until basic local and argument borrowing is stable.

## 5. Functions and calls

Function signatures should record ownership modes for parameters and return values:

```text
Owned(T)       // callee receives ownership
SharedRef(T)   // callee receives a shared borrow
MutRef(T)      // callee receives an exclusive mutable borrow
```

An owned argument moves unless its type is copyable. A shared reference argument creates an immutable borrow for the duration of the call. A mutable reference argument creates an exclusive borrow for the duration of the call.

The first milestone should prohibit returning references. It should support owned returns and copyable scalar returns. This avoids lifetime-parameter syntax while still enabling useful ownership checking. A later milestone can add returned borrows with explicit lifetime relationships.

## 6. Aggregates and fields

For structs and arrays, the checker should support field- and element-level borrowing only after whole-value moves are correct. The initial safe rule should be conservative:

- Moving any field marks the whole parent aggregate as partially moved.
- Reading the whole aggregate after a field move is rejected.
- Reading a different, unaffected field may be allowed only after a precise place-analysis implementation exists.
- Borrowing a field borrows the parent storage for the borrow region.
- A mutable borrow of one field must not overlap a shared or mutable borrow of the same parent when the implementation cannot prove disjointness.

The first implementation may reject some disjoint field patterns conservatively. False positives are preferable to unsound acceptance during the initial milestone.

## 7. Heap allocation and destruction

Heap allocation should not be added until ownership states and move checking are stable. When introduced, an owned heap value must have one destruction path. The compiler should lower destruction at scope exits and on early returns. It should not use reference counting as the default model because reference counting does not provide deterministic destruction or prevent all ownership cycles.

A future explicit `Drop` or destructor mechanism must satisfy these rules:

1. Destruction runs exactly once for every owned value.
2. Moved values are not destroyed at the original binding.
3. Partially moved aggregates destroy only their remaining initialized fields.
4. Destruction order is deterministic and documented.
5. Destructors cannot observe already-destroyed fields.
6. Destructors cannot implicitly resurrect an owner.

## 8. Unsafe and FFI boundary

Pointers, raw addresses, pointer arithmetic, and C ABI declarations should require an explicit `unsafe` block or module. The safe checker should treat raw pointers as opaque capabilities that cannot be dereferenced without an unsafe context.

The FFI boundary must specify:

- Calling convention.
- Primitive type widths.
- Struct layout and alignment.
- String encoding and ownership transfer.
- Which side allocates and which side frees.
- Nullability.
- Error propagation.
- Thread-safety obligations.

The current `print(String)` implementation through `puts` is a bootstrap runtime convenience. It is not yet the final FFI model.

## 9. Compiler architecture changes

Ownership checking should occur after name resolution and ordinary type checking, but before LLVM generation. The planned pipeline is:

```text
source
  -> lexer
  -> parser
  -> AST
  -> name resolution
  -> type checking
  -> ownership and borrow checking
  -> lowered ownership-aware IR
  -> LLVM IR
  -> native linker
```

The ownership-aware IR should make moves, borrows, drops, and storage locations explicit. Directly inferring all ownership actions during textual LLVM emission would make diagnostics and cleanup correctness difficult to test.

The IR should eventually contain operations resembling:

```text
StorageLive(local)
Move(place)
Copy(place)
BorrowShared(place, region)
BorrowMut(place, region)
Use(place)
Drop(place)
StorageDead(local)
```

This IR does not need to be public. Its purpose is to separate language semantics from LLVM syntax and to make ownership verification independently testable.

## 10. Milestone sequence

| Phase | Deliverable | Acceptance criteria |
|---|---|---|
| A | Move-only binding states | Use-after-move and double-move diagnostics pass |
| B | Copy classification | Scalars copy; resource values move; explicit clone is required |
| C | Shared and mutable local borrows | Alias rules and exclusive mutation tests pass |
| D | Borrow regions | Borrows end deterministically; invalid escapes are rejected |
| E | Ownership-aware IR | Move, borrow, and drop operations are explicit and testable |
| F | Aggregate places | Conservative field/element move and borrow checks pass |
| G | Heap runtime | Allocation and destruction execute exactly once |
| H | Unsafe/FFI | Safe code cannot dereference raw pointers or violate ABI contracts |

## 11. Required diagnostics

Diagnostics should identify both the current use and the original binding or conflicting borrow. At minimum, the checker should report:

| Error | Required information |
|---|---|
| Use after move | Moved binding, move site, current use |
| Second mutable borrow | Existing mutable borrow, requested borrow |
| Mutation during shared borrow | Shared borrow site and mutation site |
| Reference escapes scope | Referent declaration and escape location |
| Double destruction | Ownership state and cleanup path |
| Invalid FFI ownership | Transfer direction and expected contract |

The current compiler uses placeholder spans for many semantic diagnostics. Ownership work must first preserve real source spans through AST nodes so these diagnostics are actionable.

## 12. Non-goals for the first ownership milestone

The first ownership milestone should not attempt generics, async tasks, threads, interior mutability, arbitrary self-referential structs, full lifetime parameters, pointer arithmetic, automatic reference counting, or complete C++ interoperability. Each of those features changes the proof obligations and should follow a stable local ownership core.

## References

[1]: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html "The Rust Programming Language: Understanding Ownership"

[2]: https://llvm.org/docs/LangRef.html "LLVM Language Reference Manual"

[3]: https://doc.rust-lang.org/nomicon/ "The Rustonomicon"
