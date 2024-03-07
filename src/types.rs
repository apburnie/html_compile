/// **What is to be placed inside the HTML component.**
/// This could be:
/// text, e.g. `Child::Text("Hello World")` creates a `<div>Hello World</div>` component;
/// a vector of other components (`Child::ComponentVec(vec![Box::new(Component {...})])`); or
/// nothing (`Child::NoChild`)
pub enum Child<'a> {
    Text(&'a str),
    ComponentVec(Vec<Box<Component<'a>>>),
    NoChild,
}

/// **An attribute of the HTML component.**
/// Each attribute has a label and a value e.g. label could be "class" and value "Example" leading to an attribute of class="Example"
pub struct Attribute<'a> {
    pub label: &'a str,
    pub value: &'a str,
}

/// **The HTML component to be created**, where
/// `tag` describes the tag of the HTML component e.g. "div" will lead to a `<div></div>` component being built;
/// `meta` is either `None` or a list of the attributes for the component; and
/// `child` describes what is inside the component
pub struct Component<'a> {
    pub tag: &'a str,
    pub meta: Option<Vec<Attribute<'a>>>,
    pub child: Child<'a>,
}
