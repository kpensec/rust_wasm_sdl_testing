
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
pub use std::io::BufRead;

/**
 *
 */
fn fetch_file(filepath: &str) -> BufReader<File> {
    let path = Path::new(filepath);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}",
                           display, why.description()),
        Ok(file) => BufReader::new(file)
    };
    file
}

/**
 * apply function <func> on each line of the given file
 */
pub fn load_asset<F>(filepath: &str, mut func : F) -> ()
    where F : FnMut(&String) -> () {

    let file = fetch_file(filepath);
    for line in file.lines() {
        func(&line.unwrap());
    }
}

// read the content of an entire file and return it into a string buffer
pub fn read_file(filepath: &str) -> String {
    use std::io::Read;

    let mut contents = String::new();
    let mut file = File::open(filepath)
        .expect("file not found");

    file.read_to_string(&mut contents)
        .expect("something went wrong reading file!");

    contents
}

pub fn read_bfile(filepath: &str) -> Vec<u8> {
    use std::io::Read;

    let mut contents = Vec::<u8>::new();
    let mut file = File::open(filepath)
        .expect("file not found");

    file.read_to_end(&mut contents)
        .expect("something went wrong reading file!");

    contents
}

