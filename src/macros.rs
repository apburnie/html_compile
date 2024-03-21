/// Builds a Rust struct that represents an HTML component.
///
/// See documentation on `html!` for more details on the syntax to use
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

/// Generates an HTML string using the inputted syntax
///### Creating a Text Element
///#### Using a String Literal
///
/// ```text
/// html!(div (style = "border: 1px solid black;" class = "Hello") "Hello World");
/// ```
///
/// Will create the following string of HTML that consists of a `div` with a `style` attribute that creates a black border, a `class` attribute set to `Hello` and text set to `"Hello World"`
///
/// ```text
/// "<div style=\"border: 1px solid black;\" class=\"Hello\">Hello World</div>"
/// ```
/// This renders as follows in the browser:
///
/// <div style="border: 1px solid black;" class="Hello">Hello World</div>
///
///
/// The first token (here `div`) specifies the name of the element.
///
///The set of parentheses `()` contains the attributes for the element. The attribute name comes before the `=` and the attribute value after and is in double quotation marks. Different attributes are separated by whitespace.
///
///The text in double quotation marks at the end of the content specifies the text content `"Hello World"`.
///
///#### Using a Variable
///The text can also be derived from a variable. In this case surround the variable with curly brackets `{}`
///
///```text
///let example_text = "Hello World";
///
///html!(div (style = "border: 1px solid black;" class = "Hello") {example_text});
///```
///
///gives the same result as before:
///
///```text
///"<div style=\"border: 1px solid black;\" class=\"Hello\">Hello World</div>"
///```
///
///### Creating an Element with No Attributes or Content
///
///Both `html!(div)` and `html!(div ())` will create a string of HTML consisting of an empty div with no styling
///```text
///"<div></div>"
///```
///
///Void elements should not have end tags and this is handled accordingly. For example `html!(hr)` will return `"<hr />"`
///
///### Creating Elements that Contain other Elements
///
///```text
///html!(ul () [el!(li () "First Sibling")] [el!(li () "Second Sibling")])
///```
///
///Will create the following string of HTML that consists of an unordered list with two items
///
///```text
///"<ul><li>First Sibling</li><li>Second Sibling</li></ul>"
///```
///
///In the browser this renders as
///
///<ul><li>First Sibling</li><li>Second Sibling</li></ul>
///
///Each child component is surrounded by square brackets `[]` and is inputted into the macro `el!` which creates the component. Multiple specified child components are treated as siblings.
///
///The result from `el!(li () "Second Sibling")` could have been stored in a variable and the variable used instead as follows:
///
///```text
///let second_sibling = el!(li () "Second Sibling");
///let result = html!(ul()[el!(li () "First Sibling")][second_sibling]);
///```
/// This would return the same HTML String.
///### Where Child Elements are Specified through a Vector or an Array
///
///```text
///let example_list = [
///  el!(li () "First Sibling"),
///  el!(li () "Second Sibling")
///];
///
///html!(ul () vec[example_list])
///```
///
///Will create the following string of HTML that consists of an unordered list with two items
///
///```text
///"<ul><li>First Sibling</li><li>Second Sibling</li></ul>"
///```
///
///In the browser this renders as
///
///<ul><li>First Sibling</li><li>Second Sibling</li></ul>
///
///
///Inserting the text `vec` before the square brackets `[]` tells the macro to expect a vector or array.
///
#[macro_export]
macro_rules! html {
    ($($x:tt)*) => {
        {
            let component = el!($($x)*);

            build_component(&component)
        }
    }
}

/// Takes a String and swaps the placeholder text `{COMPONENT}` for the HTML String built using the provided syntax
///
/// First specify the String containing the placeholder text `{COMPONENT}`. This string should be inserted in curly brackets `{}` and have a comma inserted after the brackets.
/// Second specify the syntax describing the element to be inserted.
/// See documentation on `html!` for more details on the syntax to use.
/// ```text
/// let test_contents = String::from("<div>{COMPONENT}</div>");
///
/// let result = insert_html!({test_contents}, span () "Hello World");
///
/// assert_eq!(result, "<div><span>Hello World</span></div>");
///
/// ```
#[macro_export]
macro_rules! insert_html {
    ({$contents:expr}, $($x:tt)*) => {
        {
            let component = el!($($x)*);

            insert_components($contents, component)
        }
    }
}
