#[cfg(test)]
mod tests {
    use mantis::compile_html::*;
    use mantis::component_types::*;

    #[test]
    fn tag() {
        const test_component: Component = Component {
            tag: "section",
            meta: None,
            child: Child::NoChild,
        };
        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<section></section>");
    }

    #[test]
    fn attribute() {
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
}
