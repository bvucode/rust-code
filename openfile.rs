use std::io::Read;

fn main() {
    // required file data.txt in root directoty of project
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}