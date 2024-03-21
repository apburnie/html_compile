use std::fs;

fn main() {
    use html_compile::compile::*;
    use html_compile::html;
    use html_compile::types::*;

    // Example application generates content and then writes to file output.html

    let item_list: Vec<String> = vec![1, 2, 3].iter().map(|x| format!("{}", x)).collect();

    let item_components: Vec<Box<Component>> = item_list
        .iter()
        .map(|item| {
            Box::new(Component {
                tag: "li",
                meta: None,
                child: Child::Text(item),
            })
        })
        .collect();

    let head = vec![
        el!(meta(charset = "utf-8")),
        el!(meta(name = "viewport" content = "width=device-width")),
        el!(title () "Explanation of html_compile syntax"),
        el!(meta(
            name = "description"
            content = "Describes how html_compile syntax works"
        )),
    ];

    let section_element_containing_text = html!(section () vec[vec![
    html!(h2() "Element containing text"),
            html!(code() "html!(div (style=&ldquo;color: blue;&rdquo;, class=&ldquo;Hello&rdquo;) &ldquo;Hello World&rdquo;)"),
            html!(p() "Will create a div with the class &ldquo;Hello&rdquo; and containing the blue text &ldquo;Hello World&rdquo; as shown below"),
            html!(div (style="color:blue;" class="Hello") "Hello World" ),
            html!(p () [html!(span () "The first token, here ")] [html!(code () "div")] [html!(span () " specifies the name of the element")]),
            html!(p () [html!(span () "The set of parentheses contains the attributes for the element. The attribute name comes before the &equals; and the attribute value after and is in double quotation marks. Different attributes are separated by whitespace. So in the example the attribute style is used to change the color to blue and the attribute class is used to assign the Hello class.")]),
            html!(p () [html!(span () "The text in double quotation marks at the end specifies the text content &ldquo;Hello World&rdquo;")] ),

    ]]);

    let section_element_containing_text_variable= html!(section () vec[vec![
    html!(h2() "Element containing text in a variable"),
            html!(code() [html!(p () "let example = &ldquo;Hello World&rdquo;;")] [html!(p () "html!(div (style=&ldquo;color: blue;&rdquo;, class=&ldquo;Hello&rdquo;) {example})")]),
            html!(p() "Will create a div with the class &ldquo;Hello&rdquo; and containing the blue text &ldquo;Hello World&rdquo; as shown below"),
            html!(div (style="color:blue;" class="Hello") "Hello World" ),
            html!(p () [html!(span () "The first token, here ")] [html!(code () "div")] [html!(span () " specifies the name of the element")]),
            html!(p () [html!(span () "The set of parentheses contains the attributes for the element. The attribute name comes before the &equals; and the attribute value after and is in double quotation marks. Different attributes are separated by whitespace. So in the example the attribute style is used to change the color to blue and the attribute class is used to assign the Hello class.")]),
            html!(p () [html!(span () "The text in double quotation marks at the end specifies the text content &ldquo;Hello World&rdquo;")] ),

    ]]);


    let content = vec![
        html!(h1 () "Guide to html_compile syntax"),
        html!(
            p()[html!(span () "html_compile provides a macro ")][html!(code (style="color:blue;") "html!(...)")]
                [html!(span () "that converts a series of tokens into a web component.")]
        ),
        section_element_containing_text,
        section_element_containing_text_variable
    ];

    let input_component =
        html!(html(lang = "en")[html!(head () vec[head])][html!(body()vec[content])]);

    let html_string = build_component(&input_component);

    let contents = format!("{}{}", "<!DOCTYPE html>", html_string);

    // Example application writes to the file output.html
    let write_output_path = "./output.html";
    fs::write(write_output_path, contents).expect("Unable to write to file");
}
