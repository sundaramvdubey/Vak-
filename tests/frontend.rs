use vak::{analyze, generate_llvm, lex, parse_source, TokenKind};

#[test]
fn comments_and_literals_lex() {
    let tokens = lex("// comment\nlet value = 12.5;").unwrap();
    assert!(matches!(tokens[0].kind, TokenKind::Let));
    assert!(matches!(tokens[4].kind, TokenKind::Semicolon));
    assert_eq!(tokens[0].span.line, 2);
}

#[test]
fn representative_program_parses() {
    let source = std::fs::read_to_string("examples/hello.vak").unwrap();
    let program = parse_source(&source).unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn malformed_input_is_reported_without_panic() {
    let errors = parse_source("fn broken( { let x = ;").unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].span.line >= 1);
}

#[test]
fn illegal_character_is_a_lex_error() {
    assert!(lex("let x = @;").is_err());
}

#[test]
fn scalar_program_lowers_to_llvm() {
    let source = std::fs::read_to_string("examples/native.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("define i32 @main"));
    assert!(ir.contains("call i32 @add"));
}

#[test]
fn control_flow_lowers_to_llvm() {
    let source = std::fs::read_to_string("examples/control.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("while.cond"));
    assert!(ir.contains("br i1"));
}

#[test]
fn aggregates_lower_to_llvm() {
    let source = std::fs::read_to_string("examples/aggregates.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("getelementptr inbounds [3 x i32]"));
    assert!(ir.contains("%struct.Pair = type"));
}

#[test]
fn for_range_and_strings_lower_to_llvm() {
    let source = std::fs::read_to_string("examples/for_range.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("for.cond"));

    let source = std::fs::read_to_string("examples/strings.vak").unwrap();
    let program = parse_source(&source).unwrap();
    analyze(&program).unwrap();
    let ir = generate_llvm(&program).unwrap();
    assert!(ir.contains("declare i32 @puts(ptr)"));
    assert!(ir.contains("call i32 @puts"));
}
