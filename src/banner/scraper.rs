use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

pub fn get_reader() -> Box<dyn Read> {
    let file_path = PathBuf::from("japan").join("test.json");
    let file = File::open(file_path).expect("File did not open");
    Box::new(BufReader::new(file))
}
