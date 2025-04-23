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

fn check_children(node: &Node, code: &str) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "comment" {
            let comment_text = get_node_text(&child, code);
            println!("Comment: {}", comment_text);
        } else if child.kind() == "declaration" {
            let declaration_text = get_node_text(&child, code);
            println!("Declaration: {}", declaration_text);
        }
    }
}
fn main() {

    let file_path = "test_file.c";
    // read in file
    let code = std::fs::read_to_string(file_path).expect("Unable to read file");
    // parse the code
    let mut parser = Parser::new();
    let language = tree_sitter_c::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading C parser");
    let tree = parser.parse(&code, None).unwrap(); // Pass a reference to `code`
    assert!(!tree.root_node().has_error());

    let mut cursor = tree.root_node().walk(); // Make the cursor mutable
    let mut valid_area = false;
    let mut comment_str = String::new();
    let mut vec_found: Vec<(String, String)> = Vec::new(); // Vector of tuples to store comments and declarations

    let compiler_options = vec!["TEST", "ENABLE", "ENABLE_TEST"];

    for child in tree.root_node().children(&mut cursor) {
        // Pass a mutable reference
        if child.kind() == "comment" {
            let comment_text = get_node_text(&child, &code);
            println!("Comment: {}", comment_text);
            if comment_text.contains("a2l on") {
                valid_area = true;
            }
            if valid_area {
                comment_str.push_str(&format!("{}\n", comment_text)); // Add a newline after each comment
            }
        } else if child.kind() == "declaration"
            && child.child_by_field_name("declarator").unwrap().kind() == "init_declarator"
        {
            let declaration_text = get_node_text(&child, &code);
            if valid_area {
                println!("Declaration: {}", declaration_text);
                // Save comment string and declaration string as a tuple to vec_found
                vec_found.push((comment_str.clone(), declaration_text.clone()));
                // Clear comment string
                comment_str.clear();
            }
        } else if child.kind() == "preproc_ifdef" {
            let preproc_ifdef_name = child.child_by_field_name("name");
            let preproc_ifdef_name_text = get_node_text(&preproc_ifdef_name.unwrap(), &code);
            println!("Found preprocessor directive: {}", preproc_ifdef_name_text);
            if compiler_options.contains(&preproc_ifdef_name_text.as_str()) {
                check_children(&child, &code);
            } else {
                // search for alternative
                let alternative_node = child
                    .child_by_field_name("alternative")
                    .unwrap();
                let alternative_text = get_node_text(&alternative_node, &code);
                println!("Found alternative: {}", alternative_text);
            }
        } else {
            valid_area = false;
            comment_str.clear();
        }
    }
    // Print vec_found
    println!("\nFound {} declarations with comments:\n", vec_found.len());
    for (comment, declaration) in vec_found.iter() {
        println!("Found comment: \n{}", comment);
        println!("Found declaration: \n{}", declaration);
    } 


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
