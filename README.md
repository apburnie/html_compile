# Builds HTML files using data in Rust structs

A Rust library for converting Rust structs into web components that can be inserted into a HTML file for the building of static websites. 

## Example

The following example reads from the file `input.html`, builds the web components to insert using `input_component` and then swaps the placeholder `{COMPONENT}` for the built components. The resulting string is written to the file `output.html`:

```
use std::fs;

fn main() {
    use html_compile::compile::*;
    use html_compile::types::*;

    let read_html_path = "./input.html";

    let mut contents = fs::read_to_string(read_html_path).expect("Can read the file");

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

    let write_output_path = "./output.html";
    fs::write(write_output_path, contents).expect("Unable to write to file");
}
```

The file `output.html` looks like the following:

<!DOCTYPE html> 
  <html lang="en">
          <head>
                  <meta charset="utf-8" />
                  <meta name="viewport" content="width=device-width" />
                  <title>Test Data</title>
                  <meta
                          name="description"
                          content="Some Description"
                          slot="head"
                  />
          </head>
          <body>
	  <section style="border: 1px solid black; padding: 10px;" class="Example"><h2>A List of Items</h2><p>The list begins after the following line</p><hr></hr><ul><li>1</li><li>2</li><li>3</li></ul></section> 
          </body>
  </html>
 
