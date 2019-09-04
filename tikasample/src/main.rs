use std::process::Command;
use walkdir::WalkDir;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    for entry in WalkDir::new("testdata") {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = path.metadata().unwrap();
        if metadata.is_file() {
            let vec_u8 = extract_pdf_contents_by_using_tika(path.to_str().unwrap());
            let texts = extract_texts_from_xml_data(&vec_u8[..]);
            println!("{:?}", texts);
        }
    }
}

fn extract_pdf_contents_by_using_tika(path: &str) -> Vec<u8> {
    Command::new("tika")
        .arg(path)
        .output()
        .expect("failed to execute process")
        .stdout
}

fn extract_texts_from_xml_data(source: &[u8]) -> Vec<String> {
    let mut texts = vec![];

    for e in EventReader::new(source) {
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
