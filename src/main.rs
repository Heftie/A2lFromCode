use regex::Regex;
use tree_sitter::{Node, Parser};
use tree_sitter_c::LANGUAGE;
use a2lfile::*;

mod a2l_code_comment;
mod a2l_comment_generator;
mod code_parser;

use a2l_code_comment::*;
use a2l_comment_generator::*;
use code_parser::*;

fn main() {
    let comment = r#"
    a2l on
    a2l-type InvalidType
    a2l-characteristic-type InvalidType
    a2l-description This is a test description
    a2l-min invalid_value
    a2l-max invalid_value
    a2l-linear-coeffs invalid_value
    a2l-rat-func-coeffs invalid_value
    a2l-display-identifier TestIdentifier
    a2l-group TestGroup
    a2l-max-refresh 50ms
    a2l-read-only
    a2l-unit °C
    "#;

    let a2l_code_comment = A2lCodeComment::from_comment(comment);

    // Assertions for invalid values
    assert_eq!(a2l_code_comment.a2l_type, A2lType::Unknown);
    assert_eq!(a2l_code_comment.a2l_characteristic_type, CharacteristicType::Value);
    // Check the vaild values
    assert!(a2l_code_comment.a2l_on);
    assert_eq!(a2l_code_comment.a2l_description, "This is a test description");
    assert_eq!(a2l_code_comment.a2l_min, 0.0); // Default value
    assert_eq!(a2l_code_comment.a2l_max, 0.0); // Default value
    assert_eq!(a2l_code_comment.a2l_linear_coeffs, "invalid_value");
    assert_eq!(a2l_code_comment.a2l_rat_func_coeffs, "invalid_value");
    assert_eq!(a2l_code_comment.a2l_display_identifier, "TestIdentifier");
    assert_eq!(a2l_code_comment.a2l_group, "TestGroup");
    assert_eq!(a2l_code_comment.a2l_max_refresh, "50ms");
    assert!(a2l_code_comment.a2l_read_only);
    assert!(!a2l_code_comment.a2l_read_write); // Not set in the comment
    assert_eq!(a2l_code_comment.a2l_unit, "°C");
}
