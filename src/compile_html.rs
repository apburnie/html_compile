use crate::component_types::*;

pub fn insert_components(contents: String, component: Component) -> String {
    const COMPONENT_PLACEHOLDER: &str = "{MANTIS_COMPONENT}";

    let component = build_component(&component);

    return contents.trim().replace(COMPONENT_PLACEHOLDER, &component);
}

pub fn build_attribute(attribute_vec: &Vec<Attribute>) -> String {
    let mut attribute_string = String::from("");

    for attribute in attribute_vec {
        attribute_string = format!(
            "{} {}=\"{}\"",
            attribute_string, attribute.label, attribute.value
        );
    }

    attribute_string
}

pub fn build_start_tag(component: &Component) -> String {
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

pub fn build_component(component: &Component) -> String {
    let start_tag = build_start_tag(component);

    let middle = "";

    let end_tag = format!("</{}>", component.tag);

    return format!("{}{}{}", start_tag, middle, end_tag);
}
