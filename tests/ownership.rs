use vak::{analyze, parse_source};

fn errors(source: &str) -> Vec<vak::Diagnostic> {
    analyze(&parse_source(source).expect("source should parse"))
        .expect_err("source should be rejected")
}

#[test]
fn rejects_use_after_move_for_strings() {
    let diagnostics = errors(
        "fn main() { let value: String = \"owned\"; let moved: String = value; print(value); }",
    );
    assert!(diagnostics
        .iter()
        .any(|d| d.message.contains("use after move")));
}

#[test]
fn rejects_two_mutable_borrows() {
    let diagnostics = errors("fn main() { let mut value: I32 = 1; let first: &mut I32 = &mut value; let second: &mut I32 = &mut value; }");
    assert!(diagnostics
        .iter()
        .any(|d| d.message.contains("second mutable borrow")));
}

#[test]
fn rejects_mutable_borrow_during_shared_borrow() {
    let diagnostics = errors("fn main() { let mut value: I32 = 1; let first: &I32 = &value; let second: &mut I32 = &mut value; }");
    assert!(diagnostics
        .iter()
        .any(|d| d.message.contains("shared borrow is active")));
}

#[test]
fn temporary_mutable_borrow_ends_after_statement() {
    let program = parse_source("fn update(value: &mut I32) { *value = 2; } fn main() { let mut value: I32 = 1; update(&mut value); value = 3; }").unwrap();
    analyze(&program).unwrap();
}

#[test]
fn rejects_mutation_of_owner_while_shared_borrow_is_live() {
    let diagnostics =
        errors("fn main() { let mut value: I32 = 1; let view: &I32 = &value; value = 2; }");
    assert!(diagnostics
        .iter()
        .any(|d| d.message.contains("shared borrow is active")));
}

#[test]
fn owned_arguments_move_noncopy_values() {
    let diagnostics = errors("fn sink(value: String) { print(value); } fn main() { let value: String = \"owned\"; sink(value); print(value); }");
    assert!(diagnostics
        .iter()
        .any(|d| d.message.contains("use after move")));
}

#[test]
fn scalar_values_remain_copyable() {
    let program =
        parse_source("fn main() { let value: I32 = 7; let copy: I32 = value; print(\"ok\"); }")
            .unwrap();
    analyze(&program).unwrap();
}
