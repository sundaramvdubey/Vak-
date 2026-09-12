#![no_main]

use libfuzzer_sys::fuzz_target;
use vak::{analyze, generate_llvm, parse_source};

fuzz_target!(|source: String| {
    let Ok(program) = parse_source(&source) else { return };
    let Ok(()) = analyze(&program) else { return };
    let _ = generate_llvm(&program);
});
