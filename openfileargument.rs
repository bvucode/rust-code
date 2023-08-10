use std::env::args;
use std::fs;

fn main() {

    let file_path = args();
    let mut c = 0;
    for i in file_path {
        if c == 1 {
            println!("In file {}", i);
            let contents = fs::read_to_string(i)
            .expect("Should have been able to read the file");
            println!("With text:\n{contents}");
        }
        c += 1;
    }
}