use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f1 = File::open("foo.txt").expect("Failed to open hello.txt");
    let y = Some(f1);

    println!("hello world")
}

fn get_cluster_info() -> Result<String> {
    let config = fs::read_to_string("hello.txt")?;
    Ok(config)
}
