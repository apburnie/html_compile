use crate::component_types::*;

pub fn insert_components(contents: String, component: Component) -> String {
    const COMPONENT_PLACEHOLDER: &str = "{MANTIS_COMPONENT}";

    let component = build_component(&component);

    return contents.trim().replace(COMPONENT_PLACEHOLDER, &component);
}

pub fn build_component(component: &Component) -> String {
    let start_tag = format!("<{}>", component.tag);

    let middle = "";

    let end_tag = format!("</{}>", component.tag);

    return format!("{}{}{}", start_tag, middle, end_tag);
}
