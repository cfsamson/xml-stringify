#[derive(Debug)]
pub(crate) struct ValuePos {
    start: usize,
    end: usize,
}

impl ValuePos {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        ValuePos { start, end }
    }
}

/// An iterator over values in a parsed XML document. It will return
/// sub-slices of the original input so it doesn't force any allocations.
/// 
/// ## Example
/// ```rust
/// # extern crate xml_stringify;
/// # use xml_stringify::XmlStringParser;
/// # 
/// let parser = XmlStringParser::new("<tag>Hello world!</tag>");
/// let mut values = parser.parse();
/// let text = values.next().unwrap();
/// 
/// assert_eq!("Hello world!", text);
/// ```
#[derive(Debug)]
pub struct Values<'o> {
    original: &'o str,
    positions: Vec<ValuePos>,
    next: usize,
    len: usize,
}
impl<'o> Values<'o> {
    pub(crate) fn new(orig: &'o str) -> Self {
        Values {
            original: orig,
            positions: vec![],
            next: 0,
            len: 0,
        }
    }

    pub(crate) fn add(&mut self, pos: ValuePos) {
        self.positions.push(pos);
        self.len += 1;
    }
}

impl<'o> Iterator for Values<'o> {
    type Item = &'o str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.len {
            let pos = &self.positions[self.next];
            let res = &self.original[pos.start..pos.end];
            self.next += 1;
            Some(res)
        } else {
            None
        }
    }
}
