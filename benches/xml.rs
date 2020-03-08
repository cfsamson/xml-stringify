use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};


fn tiny_xmlparser(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}

fn ehf_xmlparser(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}

fn large_xmlparser(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| {
        for t in xmlparser::Tokenizer::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}


fn tiny_xmlrs(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}

fn ehf_xmlrs(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}

fn large_xmlrs(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| {
        for event in xml::EventReader::new(text.as_bytes()) {
            let _ = event.unwrap();
        }
    })
}


fn parse_via_quick_xml(text: &str) {
    let mut r = quick_xml::Reader::from_str(text);
    r.check_comments(true);
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    loop {
        match r.read_namespaced_event(&mut buf, &mut ns_buf) {
            Ok((_, quick_xml::events::Event::Start(_))) |
            Ok((_, quick_xml::events::Event::Empty(_))) => (),
            Ok((_, quick_xml::events::Event::Text(ref e))) => {
                e.unescaped().unwrap();
                ()
            }
            Ok((_, quick_xml::events::Event::Eof)) => break,
            _ => (),
        }
        buf.clear();
    }
}

fn tiny_quick_xml(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| parse_via_quick_xml(&text))
}

fn ehf_quick_xml(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| parse_via_quick_xml(&text))
}

fn large_quick_xml(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| parse_via_quick_xml(&text))
}


fn tiny_roxmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}

fn ehf_roxmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}

fn large_roxmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| roxmltree::Document::parse(&text).unwrap())
}


fn tiny_xmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}

fn ehf_xmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}

fn large_xmltree(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| xmltree::Element::parse(text.as_bytes()).unwrap())
}


fn tiny_sdx_document(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}

fn ehf_sdx_document(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}

fn large_sdx_document(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| sxd_document::parser::parse(&text).unwrap())
}


fn tiny_minidom(bencher: &mut Bencher) {
    let data = std::fs::read_to_string("fonts.conf").unwrap();
    bencher.iter(|| {
        let _root: minidom::Element = data.parse().unwrap();
    })
}

fn ehf_minidom(bencher: &mut Bencher) {
    let data = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| {
        let _root: minidom::Element = data.parse().unwrap();
    })
}

fn large_minidom(bencher: &mut Bencher) {
    let data = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| {
        let _root: minidom::Element = data.parse().unwrap();
    })
}

/// 
/// 

fn tiny_xmlstringify(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("fonts.conf").unwrap();
    
    bencher.iter(|| {
        let parser = xml_stringify::XmlStringParser::new(&text);
        parser.parse()
    })
}

fn ehf_xmlstringify(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("ehf.xml").unwrap();
    bencher.iter(|| {
        let parser = xml_stringify::XmlStringParser::new(&text);
        parser.parse()
    })
}

fn large_xmlstringify(bencher: &mut Bencher) {
    let text = std::fs::read_to_string("large.plist").unwrap();
    bencher.iter(|| {
        let parser = xml_stringify::XmlStringParser::new(&text);
        parser.parse()
    })
}


benchmark_group!(roxmltree, tiny_roxmltree, ehf_roxmltree, large_roxmltree);
benchmark_group!(xmltree, tiny_xmltree, ehf_xmltree, large_xmltree);
benchmark_group!(sdx, tiny_sdx_document, ehf_sdx_document, large_sdx_document);
benchmark_group!(minidom, tiny_minidom, ehf_minidom, large_minidom);
benchmark_group!(xmlparser, tiny_xmlparser, ehf_xmlparser, large_xmlparser);
benchmark_group!(xmlrs, tiny_xmlrs, ehf_xmlrs, large_xmlrs);
benchmark_group!(quick_xml, tiny_quick_xml, ehf_quick_xml, large_quick_xml);
benchmark_group!(xmlstringify, tiny_xmlstringify, ehf_xmlstringify, large_xmlstringify);
benchmark_main!(xmlstringify, roxmltree, xmltree, sdx, minidom, xmlparser, xmlrs, quick_xml);
