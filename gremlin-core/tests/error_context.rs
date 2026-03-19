use gremlin_core::error::GeneratorError;

#[test]
fn generator_error_contains_context() {
    let err = GeneratorError::InvalidUrl {
        base: "url".to_string(),
        input: "%%%".into(),
        source: url::ParseError::EmptyHost,
    };

    let msg = err.to_string();

    assert!(msg.contains("invalid generated url"));
    assert!(msg.contains("%%%"));
}
