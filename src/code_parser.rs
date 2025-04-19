use regex::Regex;
use tree_sitter::{Node, Parser};
use tree_sitter_c::LANGUAGE;
use a2lfile::*;


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
        // get the root node
        let root_node = tree.root_node();

        // create a TreeCursor and iterate over the nodes
        let mut cursor = root_node.walk();
        for node in root_node.children(&mut cursor) {
            // get the node type
            let node_type = node.kind();
            // get the node text
            let start = node.range().start_byte;
            let end = node.range().end_byte;
            let node_text = &code[start..end]; // Borrow the slice
            // print the node type and text
            println!("Node type: {}, Node text: {}", node_type, node_text);
        }
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