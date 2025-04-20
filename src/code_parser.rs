use a2lfile::*;
use regex::Regex;
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator};
use tree_sitter_c::LANGUAGE;

pub struct CodeParser {
    files_paths: Vec<String>,
}

impl CodeParser {
    pub fn new() -> Self {
        CodeParser {
            files_paths: Vec::new(),
        }
    }

    pub fn add_file_path(&mut self, file_path: String) {
        self.files_paths.push(file_path);
    }

    pub fn parse_file(&self, file_path: &str) {
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
        // walk through the code
        self.walk_through_code(&tree, &code).unwrap();
    }

    fn walk_through_code(&self, tree: &tree_sitter::Tree, code: &str) -> Result<(), tree_sitter::QueryError> {
        let cursor = tree.root_node().walk();
        // Define the Query
        let query_source = r#"
        (translation_unit
            (declaration
                declarator: (init_declarator
                    declarator: (identifier) @variable))
        )
        "#;
        let language = tree_sitter_c::LANGUAGE; // Use the `language()` function
        let query = Query::new(&language.into(), query_source)?; // Use `?` operator safely
        let mut query_cursor = QueryCursor::new();

        // Execute the Query
        let mut captures = query_cursor.captures(&query, tree.root_node(), code.as_bytes());

        // Iterate over the captures
        while let Some((query_match, _)) = captures.next() {
            for capture in query_match.captures {
                let node = capture.node; // Access the node from the capture
                let node_text = self.get_node_text(&node, code);
                println!("Found variable: {}", node_text);
            }
        }

        Ok(()) // Return success
    }

    fn get_node_text(&self, node: &Node, code: &str) -> String {
        let start = node.range().start_byte;
        let end = node.range().end_byte;
        code[start..end].to_string() // Return a new String
    }
}

mod test {
    use super::*;

    #[test]
    fn test_code_parser() {
        let mut code_parser = CodeParser::new();
        code_parser.add_file_path("test_file.c".to_string());
        code_parser.parse_file("test_file.c");
    }
}
