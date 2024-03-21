#[cfg(test)]
mod tests {
    use html_compile::compile::*;
    use html_compile::types::*;
    use html_compile::{el, html, insert_html};

    #[test]
    fn tag() {
        let test_component: Component = Component {
            tag: "section",
            meta: None,
            child: Child::NoChild,
        };
        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<section></section>");
    }

    #[test]
    fn attribute() {
        let test_component: Component = Component {
            tag: "section",
            meta: Some(vec![
                Attribute {
                    label: "style",
                    value: "border: 1px solid black;",
                },
                Attribute {
                    label: "class",
                    value: "Test",
                },
            ]),
            child: Child::NoChild,
        };
        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"></section>"
        );
    }

    #[test]
    fn text() {
        let test_component = Component {
            tag: "section",
            meta: Some(vec![
                Attribute {
                    label: "style",
                    value: "border: 1px solid black;",
                },
                Attribute {
                    label: "class",
                    value: "Test",
                },
            ]),
            child: Child::Text("Hello World"),
        };
        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\">Hello World</section>"
        );
    }

    #[test]
    fn component() {
        let test_component = Component {
            tag: "section",
            meta: Some(vec![
                Attribute {
                    label: "style",
                    value: "border: 1px solid black;",
                },
                Attribute {
                    label: "class",
                    value: "Test",
                },
            ]),
            child: Child::ComponentVec(vec![
                Box::new(Component {
                    tag: "h1",
                    meta: None,
                    child: Child::Text("Heading of Section"),
                }),
                Box::new(Component {
                    tag: "p",
                    meta: Some(vec![Attribute {
                        label: "style",
                        value: "color: blue;",
                    }]),
                    child: Child::Text("Some interesting text"),
                }),
            ]),
        };
        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1><p style=\"color: blue;\">Some interesting text</p></section>"
        );
    }

    #[test]
    fn void_element() {
        let test_component = Component {
            tag: "hr",
            meta: None,
            child: Child::NoChild,
        };
        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<hr />");
    }

    #[test]
    #[should_panic]
    fn panic_if_child_of_void() {
        let test_component = Component {
            tag: "hr",
            meta: None,
            child: Child::Text("Some Forbidden Text"),
        };
        build_component(&test_component);
    }

    #[test]
    fn insert() {
        let test_component: Component = Component {
            tag: "div",
            meta: None,
            child: Child::Text("Hello World"),
        };

        let test_contents = String::from("<div>{COMPONENT}</div>");

        let result = insert_components(test_contents, test_component);

        assert_eq!(result, "<div><div>Hello World</div></div>");
    }
    #[test]
    fn macro_tag() {
        let result = html!(section);

        assert_eq!(result, "<section></section>");
    }

    #[test]
    fn macro_attribute() {
        let result = html!(section(style = "border: 1px solid black;" class = "Test"));

        assert_eq!(
            result,
            "<section style=\"border: 1px solid black;\" class=\"Test\"></section>"
        );
    }

    #[test]
    fn macro_text_no_attribute() {
        let result = html!(section () "Hello World");

        assert_eq!(result, "<section>Hello World</section>");
    }

    #[test]
    fn macro_text_with_attribute() {
        let result =
            html!(section (style = "border: 1px solid black;" class = "Test") "Hello World");

        assert_eq!(
            result,
            "<section style=\"border: 1px solid black;\" class=\"Test\">Hello World</section>"
        );
    }

    #[test]
    fn macro_inline_component() {
        let result = html!(
            section(style = "border: 1px solid black;" class = "Test")[el!( h1 () "Heading of Section")]
        );

        assert_eq!(
            result,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1></section>"
        );
    }

    #[test]
    fn macro_interpolate_multiple_components() {
        let heading = el!(h1 () "Heading of Section");
        let text = el!(p (style = "color:blue;") "Some interesting text");

        let result = html!(
            section(style = "border: 1px solid black;" class = "Test")[heading][text][el!(p () "Further text")]
        );

        assert_eq!(
            result,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1><p style=\"color:blue;\">Some interesting text</p><p>Further text</p></section>"
        );
    }

    #[test]
    fn macro_list_components() {
        let heading = el!(h1 () "Heading of Section");

        let text_list = vec![
            heading,
            el!(p (style = "color:blue;") "text 1"),
            el!(p (style = "color:blue;") "text 2"),
        ];

        let result = html!(
            section(style = "border: 1px solid black;" class = "Test") vec[text_list]
        );

        assert_eq!(
            result,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1><p style=\"color:blue;\">text 1</p><p style=\"color:blue;\">text 2</p></section>"
        );
    }

    #[test]
    fn macro_loops() {
        // Metadata for document
        let meta = vec![
            el!(meta(charset = "utf-8")),
            el!(meta(name = "viewport" content = "width=device-width")),
            el!(title () "Test Data"),
            el!(meta(name = "description" content = "some description")),
        ];

        // Some list of items in the document

        let item_list: Vec<String> = vec![1, 2, 3].iter().map(|x| format!("{}", x)).collect();

        let li_items: Vec<Component> = item_list.iter().map(|x| el!(li () {x})).collect();

        let full_html = html!(
            html(lang = "en")[el!(head() vec[meta])][el!(body()[el!(
                section(style = "border: 1px solid black;" class = "Example")[el!(h2 () "A List of Items")]
                    [el!(p () "The list begins after the following line")][el!(hr)][el!(ul () vec[li_items])]
            )])]
        );

        let output_html = format!("{}{}", "<!DOCTYPE html>", full_html);

        assert_eq!(
                output_html,
                "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><meta name=\"viewport\" content=\"width=device-width\" /><title>Test Data</title><meta name=\"description\" content=\"some description\" /></head><body><section style=\"border: 1px solid black;\" class=\"Example\"><h2>A List of Items</h2><p>The list begins after the following line</p><hr /><ul><li>1</li><li>2</li><li>3</li></ul></section></body></html>"
            );
    }

    #[test]
    fn macro_insert() {
        let test_contents = String::from("<!DOCTYPE html>{COMPONENT}");

        let result = insert_html!({test_contents}, body () "Hello World");

        assert_eq!(result, "<!DOCTYPE html><body>Hello World</body>")
    }
}
