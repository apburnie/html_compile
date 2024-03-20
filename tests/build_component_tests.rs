#[cfg(test)]
mod tests {
    use html_compile::compile::*;
    use html_compile::html;
    use html_compile::types::*;

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
    fn macro_tag() {
        let test_component: Component = html!(section);

        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<section></section>");
    }

    #[test]
    fn macro_attribute() {
        let test_component: Component =
            html!(section(style = "border: 1px solid black;", class = "Test"));

        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"></section>"
        );
    }

    #[test]
    fn macro_text_no_attribute() {
        let test_component = html!(section () "Hello World");

        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<section>Hello World</section>");
    }

    #[test]
    fn macro_text_with_attribute() {
        let test_component =
            html!(section (style = "border: 1px solid black;", class = "Test") "Hello World");
        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\">Hello World</section>"
        );
    }

    #[test]
    fn macro_inline_component() {
        let test_component = html!(
            section(style = "border: 1px solid black;", class = "Test")[html!( h1 () "Heading of Section")]
        );

        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1></section>"
        );
    }

    #[test]
    fn macro_interpolate_multiple_components() {
        let heading = html!(h1 () "Heading of Section");
        let text = html!(p (style = "color:blue;") "Some interesting text");

        let test_component = html!(
            section(style = "border: 1px solid black;", class = "Test")[heading][text][html!(p () "Further text")]
        );

        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1><p style=\"color:blue;\">Some interesting text</p><p>Further text</p></section>"
        );
    }

    #[test]
    fn macro_list_components() {
        let heading = html!(h1 () "Heading of Section");

        let text_list = vec![
            heading,
            html!(p (style = "color:blue;") "text 1"),
            html!(p (style = "color:blue;") "text 2"),
        ];

        let test_component = html!(
            section(style = "border: 1px solid black;", class = "Test") vec[text_list]
        );

        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"><h1>Heading of Section</h1><p style=\"color:blue;\">text 1</p><p style=\"color:blue;\">text 2</p></section>"
        );
    }

    #[test]
    fn macro_loops() {
        // Metadata for document
        let meta = vec![
            html!(meta(charset = "utf-8")),
            html!(meta(name = "viewport", content = "width=device-width")),
            html!(title () "Test Data"),
            html!(meta(name = "description", content = "some description")),
        ];

        // Some list of items in the document

        let item_list: Vec<String> = vec![1, 2, 3].iter().map(|x| format!("{}", x)).collect();

        let li_items: Vec<Component> = item_list.iter().map(|x| html!(li () {x})).collect();

        let full_html = html!(
            html(lang = "en")[html!(head() vec[meta])][html!(
                body()[html!(
                    section(style = "border: 1px solid black;", class = "Example")[html!(h2 () "A List of Items")]
                        [html!(p () "The list begins after the following line")][html!(hr)][html!(ul () vec[li_items])]
                )]
            )]
        );

        let result_one = build_component(&full_html);

        let output_html = format!("{}{}", "<!DOCTYPE html>", result_one);

        assert_eq!(
                output_html,
                "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><meta name=\"viewport\" content=\"width=device-width\" /><title>Test Data</title><meta name=\"description\" content=\"some description\" /></head><body><section style=\"border: 1px solid black;\" class=\"Example\"><h2>A List of Items</h2><p>The list begins after the following line</p><hr /><ul><li>1</li><li>2</li><li>3</li></ul></section></body></html>"
            );
    }
}
