use crate::values::{ValuePos, Values};
use std::str::Chars;

#[derive(Debug)]
enum Token {
    WhiteSpace,
    LeftCroc,
    RightCroc,
    Equalsign,
    Slash,
    Tick,
    NewLine,
    Char,
}

impl Token {
    fn from(c: char) -> Token {
        use Token::*;
        match c {
            ' ' => WhiteSpace, // needs more testing if we need: _ if c.is_whitespace()
            '\n'  => NewLine,
            '<' => LeftCroc,
            '>' => RightCroc,
            '=' => Equalsign,
            '/' => Slash,
            '"' => Tick,
            _ => Char,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ParseState {
    InsideOpenBracket,
    InsideCloseBracket,
    OutsideBracket,
    Value,
}

/// The main entry point. Provide it with a XML-file contents as a `&str`.
#[derive(Debug)]
pub struct XmlStringParser<'a> {
    original: &'a str,
    input: Chars<'a>,
    state: ParseState,
    values: Values<'a>,
    charcount: usize,
    start_pos: Option<usize>,
    end_pos: usize,
    bytepos: usize,
}

impl<'a> XmlStringParser<'a> {
    pub fn new(input: &'a str) -> Self {
        XmlStringParser {
            original: input,
            input: input.chars(),
            state: ParseState::OutsideBracket,
            values: Values::new(input),
            charcount: 0,
            start_pos: None,
            end_pos: 0,
            bytepos: 0,
        }
    }

    /// Parses the document returning an iterator over the contained values.
    pub fn parse(mut self) -> Values<'a> {

        while let Some(ch) = self.input.next() {
            self.step(ch);
        }
        self.values
    }

    fn extract(&mut self) {
        let start = self.start_pos.expect("No start pos set.");
        let end = self.end_pos + 1;

        let pos = ValuePos::new(start, end);
        self.values.add(pos);
        // Reset start position
        self.start_pos = None
    }

    fn step(&mut self, ch: char) {
        use ParseState::*;

            match Token::from(ch) {
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
                },

                Token::RightCroc => match self.state {
                    InsideOpenBracket => self.state = Value,
                    InsideCloseBracket => self.state = OutsideBracket,
                    _ => panic!("unexpeced double >>"),
                },

                Token::Char => match self.state {
                    ParseState::Value => {
                        match self.start_pos {
                            Some(_) => (),
                            None => {
                                self.start_pos = Some(self.bytepos);
                            }
                        }
                        self.end_pos = self.bytepos;
                        self.charcount += 1;
                    }

                    _ => (),
                },

                Token::WhiteSpace => match self.state {
                    ParseState::Value => {
                        // if we already have one character
                        if let Some(_) = self.start_pos {
                            self.charcount += 1;
                        }
                    }

                    _ => (),
                },
                _ => (),
            
            
        }

        self.bytepos += ch.len_utf8();
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

    let expected = [
        "SOMEMULTIBYTETEXTÆØÅ👍.8-LEV.E: 10.00.",
        "MAX PALL: 1,20",
        "OSLO",
    ];

    for (got, exp) in text.zip(&expected) {
        assert_eq!(*exp, got);
    }
}
