use std::env;
use std::fs::File;
use std::io::BufReader;
use std::string::String;

const ERR_FILENAME_ARGUMENT: &str = "filename is required for argument.";
const ERR_FILE_OPEN: &str = "failed to open the file with the specified name.";

use wordcount::calculate_frequency;

fn main() {
    let filename = get_filename();
    let file = File::open(filename).expect(ERR_FILE_OPEN);
    let reader = BufReader::new(&file);
    let frequency = calculate_frequency(reader);
    println!("{:?}", frequency);
}

fn get_filename() -> String {
    let mut args: env::Args = env::args();
    let filename: Option<String> = args.nth(1);
    return filename.expect(ERR_FILENAME_ARGUMENT);
}
