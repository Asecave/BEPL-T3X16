use std::{fs::File, io::Read};

fn main() {
    let file = read_file("test.c");
    for l in file.split('\n') {
        println!("{}", l);
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(&path).expect(format!("Could not open File: {}", &path).as_str());
    let mut content = String::new();
    file.read_to_string(&mut content).expect("msg");
    content
}
