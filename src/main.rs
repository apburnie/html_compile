use std::env;
use std::fs;

fn main() {
    use html_compile::compile_html::*;
    use html_compile::component_types::*;

    let args: Vec<String> = env::args().collect();

    let read_html_path: &String = &format!("{}/src/index.html", &args[1]);

    let write_output_path = &format!("{}/index.html", &args[2]);

    let mut contents = fs::read_to_string(read_html_path).expect("Can read the file");

    // Example Component - change this to change content inserted in the HTML file

    let item_list: Vec<String> = vec![1, 2, 3].iter().map(|x| format!("{}", x)).collect();

    let item_components: Vec<Box<Component>> = item_list
        .iter()
        .map(|item| {
            Box::new(Component {
                tag: "li",
                meta: None,
                child: Child::Text(item),
            })
        })
        .collect();

    let input_component = Component {
        tag: "section",
        meta: Some(vec![
            Attribute {
                label: "style",
                value: "border: 1px solid black; padding: 10px;",
            },
            Attribute {
                label: "class",
                value: "Example",
            },
        ]),
        child: Child::ComponentVec(vec![
            Box::new(Component {
                tag: "h2",
                meta: None,
                child: Child::Text("A List of Items"),
            }),
            Box::new(Component {
                tag: "ul",
                meta: None,
                child: Child::ComponentVec(item_components),
            }),
        ]),
    };

    contents = insert_components(contents, input_component);

    fs::write(write_output_path, contents).expect("Unable to write to file");
}
