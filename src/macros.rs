// el creates components as rust structs

#[macro_export]
macro_rules! el {
// Just specify element name
    ($tag:tt) => {
        Component {
            tag: stringify!($tag),
            meta: None,
            child: Child::NoChild,
        }
    };

// Specify element name and attributes
($tag:tt ($( $label:tt=$value:literal )*)) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push(
                Attribute {
                    label: stringify!($label),
                    value: $value
                }
            );
          )*

        Component {
            tag: stringify!($tag),
            meta: Some(temp_vec),
            child: Child::NoChild,
        }
    }
    };

// Specify element name, attributes and text

($tag:tt ($( $label:tt=$value:literal )*) $content:literal) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push(
                Attribute {
                    label: stringify!($label),
                    value: $value
                }
            );
          )*

        Component {
            tag: stringify!($tag),
            meta: Some(temp_vec),
            child: Child::Text($content),
        }
    }
    };

// Specify element name, attributes and text from a variable
($tag:tt ($( $label:tt=$value:literal )*) {$content:expr}) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push(
                Attribute {
                    label: stringify!($label),
                    value: $value
                }
            );
          )*

        Component {
            tag: stringify!($tag),
            meta: Some(temp_vec),
            child: Child::Text($content),
        }
    }
    };

// Specify element name, attributes and at least one child component
($tag:tt ($( $label:tt=$value:literal )*)  $([$component:expr])* ) => {
    {
        let mut attribute_vec = Vec::new();
        $(
            attribute_vec.push(
                Attribute {
                    label: stringify!($label),
                    value: $value
                }
            );
          )*

        let mut component_vec = Vec::new();

       		$(

        component_vec.push(Box::new($component));
	)*


        Component {
            tag: stringify!($tag),
            meta: Some(attribute_vec),
            child: Child::ComponentVec(component_vec),
        }
    }
    };

// Specify element name, attributes and child components specified in a vector or array
($tag:tt ($( $label:tt=$value:literal )*) vec[$component:expr] ) => {
    {
        let mut attribute_vec = Vec::new();
        $(
            attribute_vec.push(
                Attribute {
                    label: stringify!($label),
                    value: $value
                }
            );
          )*

       let component_vec = $component.into_iter().map(|single| Box::new(single)).collect();

        Component {
            tag: stringify!($tag),
            meta: Some(attribute_vec),
            child: Child::ComponentVec(component_vec),
        }
    }
    };
}

// html directly creates html strings

#[macro_export]
macro_rules! html {
    ($($x:tt)*) => {
        {
            let component = el!($($x)*);

            build_component(&component)
        }
    }
}

// insert html into existing html string

#[macro_export]
macro_rules! insert_html {
    ({$contents:expr}, $($x:tt)*) => {
        {
            let component = el!($($x)*);

            insert_components($contents, component)
        }
    }
}
