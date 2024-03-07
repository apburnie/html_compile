#[cfg(test)]
mod tests {
    use html_compile::compile_html::*;
    use html_compile::component_types::*;

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
        let test_component : Component = Component {
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
}
