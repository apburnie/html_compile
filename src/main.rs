use std::env;
use std::fs;
use std::io;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();

    let read_html_path: &String = &format!("{}/src/index.html", &args[1]);

    let write_output_path = &format!("{}/index.html", &args[2]);

    let contents = fs::read_to_string(read_html_path).expect("Can read the file");

    fs::write(write_output_path, contents).expect("Unable to write to file");
}
