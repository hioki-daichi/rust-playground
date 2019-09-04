use std::process::Command;

fn main() {
    let output = Command::new("/usr/local/bin/tika")
        .arg("/Users/daichi/Downloads/sample.pdf")
        .output()
        .expect("failed to execute process");
    let s = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", s);
}
