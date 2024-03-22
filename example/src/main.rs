use std::fs;

fn main() {
    use html_compile::compile::*;
    use html_compile::types::*;
    use html_compile::{el, insert_html};

    // Example application generates content and then writes to file output.html

    // Head of output.html
    let head = vec![
        el!(meta(charset = "utf-8")),
        el!(meta(name = "viewport" content = "width=device-width")),
        el!(title () "Explanation of html_compile syntax"),
        el!(meta(
            name = "description"
            content = "Describes how html_compile syntax works"
        )),
        el!(style () "code {background: #DEDEDE;} section {border : 1px solid black; width: min(100rem, 90vw); border-radius: 5px; padding: 1rem; margin-bottom: 1rem;}"),
    ];

    // Sections
    let example_text = "Hello World";

    let section_element_containing_text = el!(
        section()[el!(h2 () "Creating a Text Element")][el!(code () "html!(div (style=&ldquo;color: blue;&rdquo; class=&ldquo;Hello&rdquo;) &ldquo;Hello World&rdquo;)")]
            [el!(p() "Will create the following string of HTML that consists of a div with a style attribute that renders the text blue, \
		    a class attribute set to &ldquo;Hello&rdquo; and text set to &ldquo;Hello World&rdquo; as shown below")]
            [el!(code () "&lt;div style=&ldquo;color:blue;&rdquo; class=&ldquo;Hello&rdquo;&gt;Hello World&lt;/div&gt;")]
            [el!(p() "In the browser this renders as")][el!(div (style="color: blue;" class="Hello") "Hello World" )]
            [el!(
                p()[el!(span () "The first token (here ")][el!(code () "div")][el!(span () ") specifies the name of the element")]
            )][el!(span () "The set of parentheses contains the attributes for the element. \
  The attribute name comes before the &equals; and the attribute value after and is in double quotation marks. Different attributes are separated by whitespace.\
 So in the example the attribute style is used to change the color to blue and the attribute class is used to assign the Hello class.")]
            [el!(p () "The text in double quotation marks at the end specifies the text content &ldquo;Hello World&rdquo;")]
            [el!(p () "The text can also be derived from a variable. In this case surround the variable with curly brackets &lcub;&rcub;")]
            [el!(div()[el!( code () "let example_text = &ldquo;Hello World&rdquo;;")])][el!(
            div()[el!(code () "html!(div (style=&ldquo;color: blue;&rdquo; class=&ldquo;Hello&rdquo;) &lcub;example_text&rcub;)")]
        )][el!(p () "gives the same result as before")][el!(div (style="color:blue;" class="Hello") {example_text} )]
    );

    let section_element_no_content = el!(
        section()[el!(h2 () "Creating an Element with No Attributes or Content")][el!(
            p()[el!(span () "Both ")][el!(code () "html!(div)")][el!(span () " and ")][el!(code () "html!(div ())")]
                [el!(span () "will create a string of HTML consisting of an empty div with no styling")]
                [el!(code () "&lt;div&gt;&lt;/div&gt;")]
        )][el!(
            p()[el!(span () "Void elements should not have end tags and this is handled accordingly. For example ")]
                [el!(code () "html!(hr)")][el!(span () " will return ")][el!(code () "&lt;hr/&gt;")]
        )]
    );

    let section_nested_elements = el!(
        section()[el!(h2 () "Creating Elements that Contain other Elements")][el!(code () "html!(ul () [el!(li () &ldquo;First Sibling&rdquo;)] [el!(li () &ldquo;Second Sibling&rdquo;)]")]
            [el!(p() "Will create the following string of HTML that consists of an unordered list with two items")]
            [el!(code () "&lt;ul&gt;&lt;li&gt;First Sibling&lt;/li&gt;&lt;li&gt;Second Sibling&lt;/li&gt;&lt;/ul&gt;")]
            [el!(p() "In the browser this renders as")]
            [el!(ul()[el!(li () "First Sibling")][el!(li () "Second Sibling")])][el!(p() "Each child component is surrounded by square brackets &lsqb;&rsqb; and is inputted into the macro el! which creates the component. Different child components that will be treated as siblings.")]
    );

    let example_list = [el!(li () "First Sibling"), el!(li () "Second Sibling")];

    let section_vector_elements = el!(
        section()[el!(h2 () "Where Child Elements are Specified through a Vector or an Array")][el!(
            div()[el!(
                pre()[el!( code () "let example_list = [\n  el!(li () &ldquo;First Sibling&rdquo;),\n  el!(li () &ldquo;Second Sibling&rdquo;)\n];")]
            )]
        )][el!(code () "html!(ul () vec[example_list]")][el!(p() "Will create the following string of HTML that consists of an unordered list with two items")]
            [el!(code () "&lt;ul&gt;&lt;li&gt;First Sibling&lt;/li&gt;&lt;li&gt;Second Sibling&lt;/li&gt;&lt;/ul&gt;")]
            [el!(p() "In the browser this renders as")][el!(ul () vec[example_list])][el!(p() "Inserting the text vec before the square brackets &lsqb;&rsqb; tells the macro to expect a vector or array.")]
    );

    // Creating the body from the sections
    let body = [
        el!(h1 () "Guide to html_compile&rsquo;s macros"),
        el!(p () "html_compile provides macros that enable a more concise representation of HTML data. This guide explains how these macros work through examples"),
        section_element_containing_text,
        section_element_no_content,
        section_nested_elements,
        section_vector_elements,
    ];

    let contents = insert_html!(
        { String::from("<!DOCTYPE html>{COMPONENT}") },
        html(lang = "en")[el!(head () vec[head])][el!(body()vec[body])]
    );

    // Example application writes to the file output.html
    let write_output_path = "./output.html";
    fs::write(write_output_path, contents).expect("Unable to write to file");
}
