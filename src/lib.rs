//! # Xml-stringify
//! 
//! A library that parses XML-files and is only interested in values in the 
//! xml document. It tries to do a minimal amount of work to achieve this and be
//! as fast as possible.
//! 
//! This can be beneficial if you want to search only the values in an XML file.
//! 
//! ## Example
//! 
//!  ```rust
//! # extern crate xml_stringify;
//! # use xml_stringify::XmlStringParser;
//! 
//! let xml = r#"
//! <outertag attribute1="hello">
//!     <innertag>Hello world</innertag>
//! <\outertag>"#;
//! 
//! let parser = XmlStringParser::new(xml);
//! let mut values = parser.parse();
//! let first = values.next();
//! let second = values.next();
//! 
//! assert_eq!(Some("Hello world"), first);
//! assert_eq!(None, second);
//! ```

mod parser;
mod values;

pub use parser::XmlStringParser;
pub use values::Values;