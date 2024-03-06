#[cfg(test)]
mod tests {
    use mantis::compile_html::*;
    use mantis::component_types::*;

    #[test]
    fn tag() {
        const test_component: Component = Component {
            tag: "section",
            meta: None,
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
        };
        let result_one = build_component(&test_component);

        assert_eq!(
            result_one,
            "<section style=\"border: 1px solid black;\" class=\"Test\"></section>"
        );
    }
}
