# Next Session Plan — Reference LLVM Lowering

This file is a ready-to-paste prompt for continuing Vāk in an environment that
actually has Rust + LLVM + Clang installed and network access (your Omarchy
machine, or Claude Code running locally). It picks ONE milestone out of the
larger remaining-work list, sized to be finishable and independently
verifiable, per the project's own scope-control rules.

## Why this milestone first

`OWNERSHIP_BORROWING_PLAN.md` and `PROJECT_STATUS.md` agree: the borrow
checker already accepts `&T` / `&mut T` at the semantic-analysis level, but
`LlvmCodegen` in `src/lib.rs` deliberately rejects references. Closing that
gap is the single change that unblocks the most future work (aggregate
ownership, heap allocation, unsafe/pointers all assume references already
lower to real addresses).

## Scope (do ONLY this)

1. Reference representation in codegen: a reference is a plain LLVM pointer
   to the referent's existing storage — no new runtime type, no boxing.
2. Function signatures: parameters of type `&T` / `&mut T` lower to pointer
   parameters instead of by-value parameters.
3. Taking a reference (`&x`, `&mut x`): emit the address of `x`'s existing
   alloca — do NOT copy the value.
4. Reading through a reference (`*value`): emit a load from the pointer.
5. Writing through a mutable reference (`*value = expr`): emit a store to
   the pointer.
6. Calling a function with a reference argument: pass the pointer.
7. Explicitly PROHIBIT returning a reference for now (compile-time error,
   not a codegen crash) — lifetime-safe reference returns are out of scope
   until the region/lifetime work in `OWNERSHIP_BORROWING_PLAN.md` lands.

## Explicitly out of scope for this milestone

- Full lifetime/region tracking (already documented separately)
- Borrowing struct fields or array elements
- Nested borrows, loop borrow-state analysis
- Heap allocation, Drop, ownership IR
- Raw pointers / unsafe

If you notice the change wanting to grow into any of the above, stop and
write a new Design Decision Record instead of expanding scope mid-milestone.

## Acceptance criteria

- [ ] `cargo build` succeeds with no warnings introduced
- [ ] All 23 existing tests still pass (`cargo test`)
- [ ] New test: `update(value: &mut I32)` compiles to native code and
      actually mutates the caller's variable (add an example + end-to-end
      test asserting the process exit/return value)
- [ ] New test: `inspect(value: &String)` compiles and reads correctly
- [ ] New negative test: attempting to return a reference produces a clean
      diagnostic, not a panic
- [ ] `PROJECT_STATUS.md` completion percentages are updated to reflect
      reality, with the same honest tone as the current version — no
      rounding up

## How to run it locally

```bash
# one-time setup (Arch/Omarchy)
sudo pacman -S rust llvm clang lld

cargo build
cargo test
cargo run -- check examples/hello.vak
cargo run -- build examples/hello.vak && ./hello
```

## Suggested prompt to paste into Claude Code

> Continue the Vāk compiler in this repository. Read
> OWNERSHIP_BORROWING_PLAN.md, PROJECT_STATUS.md, and NEXT_SESSION.md first.
> Implement ONLY the "Reference LLVM Lowering" milestone described in
> NEXT_SESSION.md. Do not start any other milestone. Run `cargo build` and
> `cargo test` after every change and only report success if they actually
> pass. Update PROJECT_STATUS.md honestly when done, including anything
> that turned out harder than expected.
