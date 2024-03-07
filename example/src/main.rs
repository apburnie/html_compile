use std::fs;

fn main() {
    use html_compile::compile::*;
    use html_compile::types::*;

    // Example application reads from the file input.html to get the contents and writes to file output.html

    let read_html_path = "./input.html";

    let mut contents = fs::read_to_string(read_html_path).expect("Can read the file");

    // Example Component - change this to change content inserted into the HTML file

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
                tag: "p",
                meta: None,
                child: Child::Text("The list begins after the following line"),
            }),
            Box::new(Component {
                tag: "hr",
                meta: None,
                child: Child::NoChild,
            }),
            Box::new(Component {
                tag: "ul",
                meta: None,
                child: Child::ComponentVec(item_components),
            }),
        ]),
    };

    contents = insert_components(contents, input_component);

    // Example application writes to the file output.html
    let write_output_path = "./output.html";
    fs::write(write_output_path, contents).expect("Unable to write to file");
}
