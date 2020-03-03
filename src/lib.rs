
mod parser;
mod values;

pub use parser::XmlStringParser;

use std::ffi::{c_void, CStr};

/// Extracts the values of the xml by reading a fat pointer (*const T, usize)
/// to the zero terminated C-string. Will error if the data is not valid UTF8.
#[no_mangle]
pub extern fn extract_values(chars: &[u8]) -> Result<Vec<String>, String> {
    let input = CStr::from_bytes_with_nul(chars).map_err(|e| e.to_string())?;
    let input = input.to_str().map_err(|e| e.to_string())?;
    let parser = XmlStringParser::new(input);
    let values = parser.parse();
    
    let strings: Vec<String> = values.map(|v| v.to_string()).collect();
    Ok(strings)
} 

/// Extracts the values of the xml by getting a pointer to the byte array and
/// the length of the array. Will error if the data is not valid UTF8.
#[no_mangle]
pub extern fn extract_values_from_raw_parts(data: *const u8, len: usize) -> Result<Vec<String>, String> {
    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let input = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let parser = XmlStringParser::new(input);
    let values = parser.parse();
    
    let strings: Vec<String> = values.map(|v| v.to_string()).collect();
    Ok(strings)
} 