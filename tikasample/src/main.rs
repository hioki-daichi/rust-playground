use std::env;
use std::process::Command;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let path = extract_path_from_command_args();
    let vec_u8 = extract_pdf_contents_by_using_tika(&path);
    let texts = extract_texts_from_xml_data(&vec_u8[..]);
    println!("{:?}", texts);
}

fn extract_path_from_command_args() -> String {
    env::args()
        .nth(1)
        .expect("The path to the PDF file is required.")
}

fn extract_pdf_contents_by_using_tika(path: &String) -> Vec<u8> {
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
