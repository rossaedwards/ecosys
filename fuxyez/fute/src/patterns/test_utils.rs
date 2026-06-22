//! Pattern Test Utilities (Macros + Fuzzing) - Fuxyez
//!
//! Fast macros for coverage: regression, property-based, challenge/response, 
//! and continuous transformation pipeline QA.

#[macro_export]
macro_rules! assert_pattern_match {
    ($library:expr, $node:expr, $pattern:expr) => {{
        let matches = $library.match_node(&$node);
        assert!(
            matches.iter().any(|p| p.name == $pattern),
            "Pattern '{}' did not match the node; got: {:?}", $pattern, matches
        );
    }};
}

#[macro_export]
macro_rules! pattern_property_check {
    ($library:expr, $node:expr, $pred:expr) => {{
        let matches = $library.match_node(&$node);
        assert!(
            matches.iter().any($pred),
            "No match satisfied the property predicate."
        );
    }};
}

// Bonus: MQ fuzz macro for property-based tests.
#[macro_export]
macro_rules! fuzz_for_any_pattern_match {
    ($library:expr, $generator:expr, $runs:expr) => {{
        for _ in 0..$runs {
            let test_node = $generator();
            let _ = $library.match_node(&test_node); // Just exercise matcher; result not asserted
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::library::PatternLibrary;
    use crate::ast::{AstNode, Type};

    #[test]
    fn assert_macro_catches_matching_pattern() {
        let library = PatternLibrary::new();
        let node = AstNode::VarDecl {
            name: "q".into(),
            ty: Some(Type::Named("Qubit".into())),
            value: None,
            is_mutable: false,
            symbol_id: None,
        };
        assert_pattern_match!(library, node, "QubitDeclaration");
    }

    #[test]
    fn property_check_macro_works() {
        let library = PatternLibrary::new();
        let node = AstNode::VarDecl {
            name: "q".into(),
            ty: Some(Type::Named("Qubit".into())),
            value: None,
            is_mutable: false,
            symbol_id: None,
        };
        pattern_property_check!(library, node, |p: &_| p.domain.to_string().contains("Quantum"));
    }
}