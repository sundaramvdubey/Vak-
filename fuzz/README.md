# Vāk fuzzing preparation

The compiler frontend is intentionally exposed through `parse_source(&str)`, making it suitable for a future `cargo-fuzz` harness. A production fuzzing setup should add a separate fuzz crate with targets for lexer/parser totality and semantic-analysis stability.

Recommended first targets:

1. `parse_source` must never panic for arbitrary UTF-8 input.
2. Lexing must return diagnostics for malformed strings and illegal characters.
3. Parsing must terminate on unbalanced delimiters and incomplete expressions.
4. Semantic analysis must not panic after a successful parse.

The initial corpus should include the checked examples under `examples/`, malformed delimiters, unterminated strings, invalid array lengths, nested control flow, and random Unicode. Fuzzing is not enabled as a normal dependency yet so the core repository remains dependency-free.
