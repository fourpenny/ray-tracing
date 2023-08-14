use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    let data = "Test!";
    let f = fs::File::create("./test.txt").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write to file");
}
