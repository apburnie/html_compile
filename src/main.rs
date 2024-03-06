use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    use mantis::compile_html::*;
    use mantis::component_types::*;

    let args: Vec<String> = env::args().collect();

    let read_html_path: &String = &format!("{}/src/index.html", &args[1]);

    let write_output_path = &format!("{}/index.html", &args[2]);

    let mut contents = fs::read_to_string(read_html_path).expect("Can read the file");

    let input_component = Component {
        tag: "section",
        meta: Some(vec![
Attribute{
label: "style",
value: "border: 1px solid black; height: 50px; width: 50px;"
},
Attribute{
label: "class",
value: "Example"
}


]),
    };

    contents = insert_components(contents, input_component);

    fs::write(write_output_path, contents).expect("Unable to write to file");
}
