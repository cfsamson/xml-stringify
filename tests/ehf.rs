extern crate xml_stringify;

use xml_stringify::XmlStringParser;

#[test]
fn name() {
    let input = include_str!("./ehf.xml");

    let parser = XmlStringParser::new(input);

    let values = parser.parse();
    let text = values.map(|v| v.to_string()).collect::<Vec<String>>();
    
    let expected = include_str!("./expected.txt");
    let expected: Vec<String> = expected.lines().map(|l| l.to_string()).collect();

    for (exp, got) in expected.iter().zip(text.iter()) {
        assert_eq!(exp, got);
    }
}



// let formatted = text.join("\n");
// println!("{}", formatted);