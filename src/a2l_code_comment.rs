use regex::Regex;
use tree_sitter::{Node, Parser};
use tree_sitter_c::LANGUAGE;
use a2lfile::*;

#[derive(Debug, PartialEq)]
pub enum A2lType {
    Measurement,
    Characteristic,
    Unknown, // Fallback for unsupported or missing types
}

impl A2lType {
    // Function to parse a string into an A2lType
    pub fn from_str(type_str: &str) -> Self {
        if type_str.to_lowercase().contains("measurement") {
            A2lType::Measurement
        } else if type_str.to_lowercase().contains("characteristic") {
            A2lType::Characteristic
        } else {
            A2lType::Unknown
        }
    }
}


pub struct A2lCodeComment {
    pub a2l_on: bool,
    pub a2l_type: A2lType, // Use the enum here
    pub a2l_characteristic_type: CharacteristicType, // Fully qualified path
    pub a2l_description: String,
    pub a2l_min: f64,
    pub a2l_max: f64,
    pub a2l_linear_coeffs: String,
    pub a2l_rat_func_coeffs: String,
    pub a2l_display_identifier: String,
    pub a2l_group: String,
    pub a2l_max_refresh: String,
    pub a2l_read_only: bool,
    pub a2l_read_write: bool,
    pub a2l_unit: String,
}

impl A2lCodeComment {
    pub fn new() -> Self {
        A2lCodeComment {
            a2l_on: false,
            a2l_type: A2lType::Unknown,
            a2l_characteristic_type: CharacteristicType::Value,
            a2l_description: String::new(),
            a2l_min: 0.0,
            a2l_max: 0.0,
            a2l_linear_coeffs: String::new(),
            a2l_rat_func_coeffs: String::new(),
            a2l_display_identifier: String::new(),
            a2l_group: String::new(),
            a2l_max_refresh: String::new(),
            a2l_read_only: false,
            a2l_read_write: false,
            a2l_unit: String::new(),
        }
    }

    pub fn from_comment(comment: &str) -> Self {
        // a comment is multiple lines
        let mut a2l_code_comment = A2lCodeComment::new();
        for line in comment.lines() {
            // check for a2l on or off search witch regex
            let re_on = Regex::new(r"a2l\s+on").unwrap();
            if re_on.is_match(line) {
                a2l_code_comment.a2l_on = true;
            }
            let re_off = Regex::new(r"a2l\s+off").unwrap();
            if re_off.is_match(line) {
                a2l_code_comment.a2l_on = false;
            }
            // check for a2l type measurement or characteristic
            if line.contains("a2l-type") {
                a2l_code_comment.a2l_type = A2lType::from_str(&line);
            }
            // check for a2l characteristic type
            // Todo: add more types Maps, Curves,etc
            if line.contains("a2l-characteristic-type") {
                let re = Regex::new(r"a2l-characteristic-type\s+(\w+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_characteristic_type = match captures[1].to_lowercase().as_str() {
                        "ascii" => CharacteristicType::Ascii,
                        "value" => CharacteristicType::Value,
                        "valblk" => CharacteristicType::ValBlk,
                        _ => CharacteristicType::Value, // Default case
                    };
                }
            }
            // check for a2l description
            if line.contains("a2l-description") {
                let re = Regex::new(r"a2l-description\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_description = captures[1].to_string();
                }               
            }
            // check for a2l min (float or integer)
            if line.contains("a2l-min") {
                let re = Regex::new(r"a2l-min\s+([-+]?\d*\.?\d+([eE][-+]?\d+)?)").unwrap();
                if let Some(captures) = re.captures(line) {
                    if let Ok(value) = captures[1].parse::<f64>() {
                        a2l_code_comment.a2l_min = value;
                    } else {
                        eprintln!("Failed to parse a2l-min value");
                    }
                }
            }
            // check for a2l max (float or integer)
            if line.contains("a2l-max") {
                let re = Regex::new(r"a2l-max\s+([-+]?\d*\.?\d+([eE][-+]?\d+)?)").unwrap();
                if let Some(captures) = re.captures(line) {
                    if let Ok(value) = captures[1].parse::<f64>() {
                        a2l_code_comment.a2l_max = value;
                    } else {
                        eprintln!("Failed to parse a2l-max value");
                    }
                }
            }
            // check for a2l linear coeffs
            if line.contains("a2l-linear-coeffs") {
                let re = Regex::new(r"a2l-linear-coeffs\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_linear_coeffs = captures[1].to_string();
                }
            }
            // check for a2l rat func coeffs
            if line.contains("a2l-rat-func-coeffs") {
                let re = Regex::new(r"a2l-rat-func-coeffs\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_rat_func_coeffs = captures[1].to_string();
                }
            }
            // check for a2l display identifier
            if line.contains("a2l-display-identifier") {
                let re = Regex::new(r"a2l-display-identifier\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_display_identifier = captures[1].to_string();
                }
            }
            // check for a2l group
            if line.contains("a2l-group") {
                let re = Regex::new(r"a2l-group\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_group = captures[1].to_string();
                }
            }
            // check for a2l max refresh
            if line.contains("a2l-max-refresh") {
                let re = Regex::new(r"a2l-max-refresh\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_max_refresh = captures[1].to_string();
                }
            }
            // check for a2l read only
            if line.contains("a2l-read-only") {
                a2l_code_comment.a2l_read_only = true;
            }
            // check for a2l read write
            if line.contains("a2l-read-write") {
                a2l_code_comment.a2l_read_write = true;
            }
            // check for a2l unit
            if line.contains("a2l-unit") {
                let re = Regex::new(r"a2l-unit\s+(.+)").unwrap();
                if let Some(captures) = re.captures(line) {
                    a2l_code_comment.a2l_unit = captures[1].to_string();
                }
            }


        }
        // return the a2l code comment
        a2l_code_comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2l_code_comment_from_comment() {
        let comment = r#"
        a2l on
        a2l-type Measurement
        a2l-characteristic-type Ascii
        a2l-description This is a test description
        a2l-min -10.5
        a2l-max 1000
        a2l-linear-coeffs 1.23
        a2l-rat-func-coeffs 4.56
        a2l-display-identifier TestIdentifier
        a2l-group TestGroup
        a2l-max-refresh 50ms
        a2l-read-only
        a2l-unit m/s
        "#;

        let a2l_code_comment = A2lCodeComment::from_comment(comment);

        // Assertions
        assert!(a2l_code_comment.a2l_on);
        assert_eq!(a2l_code_comment.a2l_type, A2lType::Measurement);
        assert_eq!(a2l_code_comment.a2l_characteristic_type, CharacteristicType::Ascii);
        assert_eq!(a2l_code_comment.a2l_description, "This is a test description");
        assert_eq!(a2l_code_comment.a2l_min, -10.5);
        assert_eq!(a2l_code_comment.a2l_max, 1000.0);
        assert_eq!(a2l_code_comment.a2l_linear_coeffs, "1.23");
        assert_eq!(a2l_code_comment.a2l_rat_func_coeffs, "4.56");
        assert_eq!(a2l_code_comment.a2l_display_identifier, "TestIdentifier");
        assert_eq!(a2l_code_comment.a2l_group, "TestGroup");
        assert_eq!(a2l_code_comment.a2l_max_refresh, "50ms");
        assert!(a2l_code_comment.a2l_read_only);
        assert!(!a2l_code_comment.a2l_read_write); // Not set in the comment
        assert_eq!(a2l_code_comment.a2l_unit, "m/s");
    }

    #[test]
    fn test_a2l_code_comment_from_comment_invalid() {
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

    #[test]
    fn test_a2l_code_comment_defaults() {
        let a2l_code_comment = A2lCodeComment::new();

        // Assertions for default values
        assert!(!a2l_code_comment.a2l_on);
        assert_eq!(a2l_code_comment.a2l_type, A2lType::Unknown);
        assert_eq!(a2l_code_comment.a2l_characteristic_type, CharacteristicType::Value);
        assert_eq!(a2l_code_comment.a2l_description, "");
        assert_eq!(a2l_code_comment.a2l_min, 0.0);
        assert_eq!(a2l_code_comment.a2l_max, 0.0);
        assert_eq!(a2l_code_comment.a2l_linear_coeffs, "");
        assert_eq!(a2l_code_comment.a2l_rat_func_coeffs, "");
        assert_eq!(a2l_code_comment.a2l_display_identifier, "");
        assert_eq!(a2l_code_comment.a2l_group, "");
        assert_eq!(a2l_code_comment.a2l_max_refresh, "");
        assert!(!a2l_code_comment.a2l_read_only);
        assert!(!a2l_code_comment.a2l_read_write);
        assert_eq!(a2l_code_comment.a2l_unit, "");
    }

    #[test]
    fn test_a2l_code_comment_with_scientific_notation() {
        let comment = r#"
        a2l on
        a2l-type Measurement
        a2l-min -1.23e4
        a2l-max 5.67E-3
        "#;

        let a2l_code_comment = A2lCodeComment::from_comment(comment);

        // Assertions
        assert!(a2l_code_comment.a2l_on);
        assert_eq!(a2l_code_comment.a2l_type, A2lType::Measurement);
        assert_eq!(a2l_code_comment.a2l_min, -12300.0); // Parsed scientific notation
        assert_eq!(a2l_code_comment.a2l_max, 0.00567);  // Parsed scientific notation
    }
}