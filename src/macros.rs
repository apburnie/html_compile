#[macro_export]
macro_rules! html {
    ($tag:tt) => {
        Component {
            tag: stringify!($tag),
            meta: None,
            child: Child::NoChild,
        }
    };

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
