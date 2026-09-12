use vak::{analyze_with_source, parse_source};

#[test]
fn semantic_diagnostics_include_source_location_and_code() {
    let source = "fn main() {\n  let value: String = \"owned\";\n  let moved: String = value;\n  print(value);\n}\n";
    let program = parse_source(source).unwrap();
    let diagnostics = analyze_with_source(&program, source).unwrap_err();
    let diagnostic = diagnostics
        .iter()
        .find(|d| d.message.contains("use after move"))
        .unwrap();
    assert_eq!(diagnostic.code, Some("E0401"));
    assert!(diagnostic.span.line >= 3);
    assert!(diagnostic.span.column >= 1);
}
