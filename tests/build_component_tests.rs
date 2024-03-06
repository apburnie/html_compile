#[cfg(test)]
mod tests {
    use mantis::compile_html::*;
    use mantis::component_types::*;

    const test_component: Component = Component { tag: "section" };

    #[test]
    fn tag() {
        let result_one = build_component(&test_component);

        assert_eq!(result_one, "<section></section>");
    }
}
