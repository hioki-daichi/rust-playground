use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let texts = extract_texts_from_xml("foo.xml");
    println!("{:?}", texts);
}

#[allow(dead_code)]
// execute_tika("sample.pdf");
fn execute_tika(path: &str) -> String {
    let output = std::process::Command::new("/usr/local/bin/tika")
        .arg(path)
        .output()
        .expect("failed to execute process");
    String::from_utf8(output.stdout).expect("String::from_utf8 failed")
}

#[allow(dead_code)]
// extract_texts_from_xml("foo.xml");
fn extract_texts_from_xml(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut texts: Vec<String> = vec![];
    for e in parser {
        match e {
            Ok(XmlEvent::Characters(text)) => {
                texts.push(text);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    texts
}
