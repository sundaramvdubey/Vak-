# Vāk Runtime Contract

The current runtime is intentionally minimal. Vāk `String` values lower to pointers to null-terminated UTF-8 byte sequences. The built-in `print(String)` operation lowers to the platform C ABI function `puts`. Fixed-array bounds failures lower to the LLVM `llvm.trap` intrinsic. These contracts are sufficient for the current examples but are not a stable long-term ABI.

Future runtime work must introduce an explicit Vāk runtime library with versioned symbols, allocation and deallocation rules, string length and comparison operations, error reporting, and platform portability. Direct libc calls should then be isolated behind that runtime rather than emitted by the compiler backend.
