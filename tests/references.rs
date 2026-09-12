use vak::{analyze, generate_llvm, parse_source};

#[test]
fn references_lower_to_native_llvm_shapes() {
    let source = std::fs::read_to_string("examples/references.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("define void @update(ptr %arg_value)"));
    assert!(ir.contains("define i32 @read(ptr %arg_value)"));
    assert!(ir.contains("call void @update(ptr"));
    assert!(ir.contains("call i32 @read(ptr"));
    assert!(ir.contains("store i32"));
    assert!(ir.contains("load i32, ptr %arg_value"));
}

#[test]
fn shared_references_cannot_be_mutated() {
    let program = parse_source("fn bad(value: &I32) { *value = 2; }").unwrap();
    let errors = analyze(&program).unwrap_err();
    assert!(errors
        .iter()
        .any(|error| error.message.contains("shared reference")));
}

#[test]
fn reference_returns_are_explicitly_rejected() {
    let program = parse_source("fn bad(value: &I32) -> &I32 { return value; }").unwrap();
    let errors = analyze(&program).unwrap_err();
    assert!(errors
        .iter()
        .any(|error| error.message.contains("cannot return a reference")));
}
