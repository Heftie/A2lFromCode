use regex::Regex;
use tree_sitter::{Node, Parser};
use tree_sitter_c::LANGUAGE;
use a2lfile::*;

pub struct A2lCommentGenerator{

}


impl A2lCommentGenerator {
    pub fn new() -> Self {
        A2lCommentGenerator {}
    }
    // match c variable types to a2l types
    fn match_c_type_to_a2l_type(&self, c_type: &str) -> DataType {
        // Konvertiere in Kleinbuchstaben und erhalte einen String-Slice
        match c_type.to_lowercase().as_str() {
            // Byte-Variablen
            "uint8_t" => DataType::Ubyte,
            "int8_t" => DataType::Sbyte,
            "uint8" => DataType::Ubyte,
            "int8" => DataType::Sbyte,
            "unsigned char" => DataType::Ubyte,
            "char" => DataType::Sbyte,
            // Word-Variablen 16 Bit
            "unsigned short" => DataType::Uword,
            "short" => DataType::Sword,
            "uint16_t" => DataType::Uword,
            "int16_t" => DataType::Sword,
            "uint16" => DataType::Uword,
            "int16" => DataType::Sword,
            // Word-Variablen 32 Bit
            "int" => DataType::Slong,
            "unsigned int" => DataType::Ulong,
            "uint32_t" => DataType::Ulong,
            "int32_t" => DataType::Slong,
            "uint32" => DataType::Ulong,
            "int32" => DataType::Slong,
            // Long-Variablen 64 Bit
            "long long" => DataType::AInt64,
            "unsigned long long" => DataType::AUint64,
            "uint64_t" => DataType::AUint64,
            "int64_t" => DataType::AInt64,
            "uint64" => DataType::AUint64,
            "int64" => DataType::AInt64,
            // floating point variables
            "float" => DataType::Float32Ieee,
            "double" => DataType::Float64Ieee,
            "long" => DataType::Slong,
            // throw an error if the type is not found
            _ => {
                println!("Error: Type {} not found", c_type);
                // return a default type
                DataType::Ubyte
            }
        }
    }

    fn create_characteristic(
        &self,
        name: &str,
        long_identifier: &str,
        characteristic_type: CharacteristicType,
        deposit : &str,
        conversion: &str,
        min: f64,
        max: f64,

    ) -> Characteristic {
        Characteristic::new(
            name.to_string(),
            long_identifier.to_string(),
            characteristic_type,
            0,
            deposit.to_string(),
            0.0,
            conversion.to_string(),
            min,
            max
        )
    }    

    fn create_measuremnt(
        &self,
        name: &str,
        long_identifier: &str,
        datatype: DataType,
        conversion: &str,
        resolution: u16,
        min: f64,
        max: f64,
    ) -> Measurement {
        Measurement::new(
            name.to_string(),
            long_identifier.to_string(),
            datatype,
            conversion.to_string(),
            resolution,
            0.0,
            min,
            max,
        )
    }
}
