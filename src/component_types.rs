pub struct Attribute<'a> {
    pub label: &'a str,
    pub value: &'a str,
}

pub struct Component<'a> {
    pub tag: &'a str,
    pub meta: Option<Vec<Attribute<'a>>>,
}
