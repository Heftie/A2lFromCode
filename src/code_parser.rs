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


}