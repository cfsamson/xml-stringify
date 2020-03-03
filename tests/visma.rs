extern crate xml_stringify;

use xml_stringify::XmlStringParser;

#[test]
fn name() {
    let input = include_str!("./visma.xml");

    let mut parser = XmlStringParser::new(input);

    let values = parser.parse();
    let text = values.map(|v| v.to_string()).collect::<Vec<String>>();
    let formatted = text.join("\n");

    println!("{}", formatted);

    // let mut tokens = res.iter();
    // // Example
    // let mut text = vec![];
    // let mut start = None;
    // let mut end = 0;
    // while let Some(token) = tokens.next() {
    //     match token {
    //         Token::Char(index) => {
    //             if start.is_none() {
    //                 start = Some(*index);
    //             } else {
    //                 end = *index;
    //             }
    //         }

    //         _ => match start {
    //             Some(start_index) => {
    //                 text.push(input[start_index..end+1].to_string());
    //                 start = None;
    //             }

    //             None => (),
    //         },
    //     }
    // }

    // for t in text {
    //     println!("{}", t);
    // }
}