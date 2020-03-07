use crate::values::{Values, ValuePos};

fn lex(input: &str) -> Vec<Token> {

    let mut byteindex = 0;

    let mut chars = input.chars().peekable();
    // Remove whitespace and newlines from the start
    while let Some(ch) = chars.peek() {
        let token = Token::from(*ch, 0);
        match token {
            Token::WhiteSpace | Token::NewLine => {
                let c = chars.next().unwrap();
                byteindex += c.len_utf8();
            }

            _ => break,
        }
    }

    // We're at the first character
    let mut tokens: Vec<Token> = chars.map(|t| {
        let token = Token::from(t, byteindex);
        byteindex += t.len_utf8();
        token
    }).collect();

    // remove whitespace from the end
    while let Some(token) = tokens.pop() {
        match token {
            Token::WhiteSpace | Token::NewLine => (),
            _ => {
                // it wasn't a nl or ws so we put it back
                tokens.push(token);
                break;
            }
        }
    }

    tokens
}

#[derive(Debug)]
enum Token {
    WhiteSpace,
    LeftCroc,
    RightCroc,
    Equalsign,
    Slash,
    Tick,
    NewLine,
    Char(usize),
}

impl Token {
    fn from(c: char, index: usize) -> Token {
        use Token::*;
        match c {
            _ if c.is_whitespace() => WhiteSpace,
            _ if c.is_control() => NewLine,
            '<' => LeftCroc,
            '>' => RightCroc,
            '=' => Equalsign,
            '/' => Slash,
            '"' => Tick,
            // this in string indicies `text[start_index..end_index]`
            _ => Char(index),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ParseState  {
    InsideOpenBracket,
    InsideCloseBracket,
    OutsideBracket,
    Value,
}

/// The main entry point. Provide it with a XML-file contents as a `&str`.
#[derive(Debug)]
pub struct XmlStringParser<'a> {
    input: Vec<Token>,
    state: ParseState,
    values: Values<'a>,
    pos: usize,
    charcount: usize,
    start_pos: Option<usize>,
    end_pos: usize,
}

impl<'a> XmlStringParser<'a> {
    pub fn new(input: &'a str) -> Self {

        let tokens = lex(input);
        
        XmlStringParser {
            //original: input,
            input: tokens,
            state: ParseState::OutsideBracket,
            values: Values::new(input),
            pos: 0,
            charcount: 0,
            start_pos: None,
            end_pos: 0,
        }
    }

    /// Parses the document returning an iterator over the contained values.
    pub fn parse(mut self) -> Values<'a> {
        while self.step() { };

        self.values
    }

    fn extract(&mut self) {
        let start = self.start_pos.expect("No start pos set.");
        let end = self.end_pos + 1;

        let pos = ValuePos::new(start, end);
        self.values.add(pos);
            // let extracted = self.original[start..end].to_string();
            // self.text.push(extracted);
        // Reset start position
        self.start_pos = None
    }

    fn step(&mut self) -> bool {
        use ParseState::*;

        match self.input[self.pos] {
            Token::LeftCroc => match self.state {
                OutsideBracket => self.state = InsideOpenBracket,
                Value => {
                    // if there are no characters, we're in a nested scope
                    if self.charcount == 0 {
                        self.state = InsideOpenBracket;
                    } else {
                        self.state = InsideCloseBracket;
                        // Extract the text
                        self.extract();
                        self.charcount = 0;
                    }
                }
                _ => (),
            }

            Token::RightCroc => match self.state {
                InsideOpenBracket => self.state = Value,
                InsideCloseBracket => self.state = OutsideBracket,
                _ => panic!("unexpeced double >>"),
            }

            Token::Char(index) => match self.state {
                ParseState::Value => {
                    match self.start_pos {
                        Some(_) => (),
                        None => self.start_pos = Some(index),
                    }
                    self.end_pos = index;
                    self.charcount += 1;
                }

                _ => (),
            }

            Token::WhiteSpace => match self.state {
                ParseState::Value => {
                    // if we already have one character
                    if let Some(_) = self.start_pos {
                        self.charcount += 1;
                    }
                }

                _ => (),
            }

            _ => (),
        }

        self.pos += 1;

        if self.pos < self.input.len() {
            true
        } else {
            false
        }
    }
}







#[test]
fn edge_case() {
    let input = "
    <cac:PostalAddress>
    <cbc:StreetName>SOMEMULTIBYTETEXTÆØÅ👍.8-LEV.E: 10.00.</cbc:StreetName>
    <cbc:AdditionalStreetName>MAX PALL: 1,20</cbc:AdditionalStreetName>
    <cbc:CityName>OSLO</cbc:CityName>
";

    let parser = XmlStringParser::new(input);

    let text = parser.parse();

    let expected = ["SOMEMULTIBYTETEXTÆØÅ👍.8-LEV.E: 10.00.", "MAX PALL: 1,20", "OSLO"];

    for (got, exp) in text.zip(&expected) {
        
        assert_eq!(*exp, got);
    }
    
}
