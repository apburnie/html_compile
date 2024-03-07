use crate::types::*;

fn build_attribute(attribute_vec: &Vec<Attribute>) -> String {
    let mut attribute_string = String::from("");

    for attribute in attribute_vec {
        attribute_string = format!(
            "{} {}=\"{}\"",
            attribute_string, attribute.label, attribute.value
        );
    }

    attribute_string
}

fn build_start_tag(component: &Component) -> String {
    let mut start_tag = format!("<{}", component.tag);

    match &component.meta {
        Some(meta) => {
            let attribute_string = build_attribute(&meta);

            start_tag = format!("{}{}>", start_tag, attribute_string);
        }
        None => {
            start_tag = format!("{}>", start_tag);
        }
    }

    start_tag
}

fn build_body(component: &Component) -> String {
    let mut markup = "".to_string();

    match &component.child {
        Child::NoChild => {}
        Child::Text(value) => {
            markup = format!("{}", value);
        }
        Child::ComponentVec(value) => {
            for single in value {
                markup = format!("{}{}", markup, build_component(single));
            }
        }
    }

    markup
}

/// Builds HTML component using the provided struct `component`
///
/// # Example
///
/// ```
/// use html_compile::compile::*;
/// use html_compile::types::*;
///
/// let input_component = Component {
///     tag: "section",
///     meta: Some(vec![
///         Attribute {
///             label: "style",
///             value: "border: 1px solid black; padding: 10px;",
///         },
///         Attribute {
///             label: "class",
///             value: "Example",
///         },
///     ]),
///     child: Child::ComponentVec(vec![
///         Box::new(Component {
///             tag: "h2",
///             meta: None,
///             child: Child::Text("Text of a Heading"),
///         }),
///         Box::new(Component {
///             tag: "p",
///             meta: None,
///             child: Child::Text("Text of a Paragraph"),
///         }),
///     ]),
/// };
///
/// let output = build_component(&input_component);  
///
/// assert_eq!(output, String::from("<section style=\"border: 1px solid black; padding: 10px;\" class=\"Example\"><h2>Text of a Heading</h2><p>Text of a Paragraph</p></section>"));
///
/// ```
/// In a browser the string will display as follows:
///
/// <section style="border: 1px solid black; padding: 10px;" class="Example">
///     <h2>Text of a Heading</h2>
///     <p>Text of a Paragraph</p>
/// </section>
pub fn build_component(component: &Component) -> String {
    let start_tag = build_start_tag(component);

    let middle = build_body(component);

    let end_tag = format!("</{}>", component.tag);

    return format!("{}{}{}", start_tag, middle, end_tag);
}

/// Takes the provided HTML string `contents` and swaps the string `{COMPONENT}` for the HTML component built using the provided struct `component`
///
/// # Example
///
/// ```
/// use html_compile::compile::*;
/// use html_compile::types::*;
///
/// let contents = String::from("<html lang=\"en\"><head><title>Test Data</title></head><body>{COMPONENT}</body></html>");
///
/// let input_component = Component {
///     tag: "section",
///     meta: Some(vec![
///         Attribute {
///             label: "style",
///             value: "border: 1px solid black; padding: 10px;",
///         },
///         Attribute {
///             label: "class",
///             value: "Example",
///         },
///     ]),
///     child: Child::ComponentVec(vec![
///         Box::new(Component {
///             tag: "h2",
///             meta: None,
///             child: Child::Text("Text of a Heading"),
///         }),
///         Box::new(Component {
///             tag: "p",
///             meta: None,
///             child: Child::Text("Text of a Paragraph"),
///         }),
///     ]),
/// };
///
/// let output = insert_components(contents, input_component);  
///
/// assert_eq!(output, String::from("<html lang=\"en\"><head><title>Test Data</title></head><body><section style=\"border: 1px solid black; padding: 10px;\" class=\"Example\"><h2>Text of a Heading</h2><p>Text of a Paragraph</p></section></body></html>"));
/// ```
/// In a browser the string will display as follows:
///
/// <html lang="en">
///     <head>
///         <title>Test Data</title>
///     </head>
/// <body>
///     <section style="border: 1px solid black; padding: 10px;" class="Example">
///         <h2>Text of a Heading</h2>
///         <p>Text of a Paragraph</p>
///     </section>
///</body>
///</html>
///
pub fn insert_components(contents: String, component: Component) -> String {
    const COMPONENT_PLACEHOLDER: &str = "{COMPONENT}";

    let component = build_component(&component);

    return contents.trim().replace(COMPONENT_PLACEHOLDER, &component);
}
