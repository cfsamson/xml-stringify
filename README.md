# Xml-stringify

A library that parses XML-files and is only interested in values in the 
xml document. It tries to do a minimal amount of work to achieve this and be
as fast as possible.

This can be beneficial if you want to search only the values in an XML file.

## Example

 ```rust
# extern crate xml_stringify;
# use xml_stringify::XmlStringParser;

let xml = r#"
<outertag attribute1="hello">
    <innertag>Hello world</innertag>
<\outertag>"#;

let parser = XmlStringParser::new(xml);
let mut values = parser.parse();
let first = values.next();
let second = values.next();

assert_eq!(Some("Hello world"), first);
assert_eq!(None, second);
```

## Performace

It's pretty fast at what it does. A comparison with other XML-parsing libraries
looks like this:

```
test ehf_xmlstringify   ... bench:      28,581 ns/iter (+/- 3,324)
test ehf_minidom        ... bench:     455,908 ns/iter (+/- 34,557)
test ehf_quick_xml      ... bench:      72,714 ns/iter (+/- 5,028)
test ehf_roxmltree      ... bench:     208,962 ns/iter (+/- 35,456)
test ehf_sdx_document   ... bench:     529,165 ns/iter (+/- 15,944)
test ehf_xmlparser      ... bench:      71,252 ns/iter (+/- 8,515)
test ehf_xmlrs          ... bench:   1,980,872 ns/iter (+/- 118,392)
test ehf_xmltree        ... bench:   2,137,147 ns/iter (+/- 71,397)

test large_xmlstringify ... bench:     728,460 ns/iter (+/- 125,536)
test large_minidom      ... bench:   6,286,425 ns/iter (+/- 276,356)
test large_quick_xml    ... bench:   1,184,321 ns/iter (+/- 48,501)
test large_roxmltree    ... bench:   5,257,315 ns/iter (+/- 168,505)
test large_sdx_document ... bench:   7,163,660 ns/iter (+/- 326,862)
test large_xmlparser    ... bench:   1,437,787 ns/iter (+/- 61,632)
test large_xmlrs        ... bench:  28,742,580 ns/iter (+/- 490,208)
test large_xmltree      ... bench:  31,086,030 ns/iter (+/- 617,315)

test tiny_xmlstringify  ... bench:         797 ns/iter (+/- 217)
test tiny_minidom       ... bench:      12,217 ns/iter (+/- 983)
test tiny_quick_xml     ... bench:       2,576 ns/iter (+/- 151)
test tiny_roxmltree     ... bench:       4,793 ns/iter (+/- 125)
test tiny_sdx_document  ... bench:      20,037 ns/iter (+/- 839)
test tiny_xmlparser     ... bench:       2,069 ns/iter (+/- 324)
test tiny_xmlrs         ... bench:      46,106 ns/iter (+/- 5,887)
test tiny_xmltree       ... bench:      48,970 ns/iter (+/- 1,385)
```

> The benchmark suite is taken and adapted from [roxmltree](https://github.com/RazrFalcon/roxmltree)

The EHF-benchmark is the one this library was created for. Searching for text
in an EHF invoice to later get processed by rules requiring a text search in the
keys of the EHF invoice.

## Notes on usage

Bear in mind that we _only_ care about the key-fields of the XML-file. This
library will be lenient on parsing even invalid XML-files and will only
reject ones that are malformed in a way in which we can not reliably parse them.
