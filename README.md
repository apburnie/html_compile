# HTML Compile

A Rust tool for converting Rust structs into web components that can be inserted into a HTML file for the building of static websites. This requires having Rust set up.

# How does it work?

Run the following command in the command line:

```
cargo run -- arg1 arg2
```

* `arg1` is the file path to a directory containing `src/index.html`. The file `index.html` will contain the value `{COMPONENT}` which is where the components will be inserted.
* `arg2` is the file path to where the created HTML file will be inserted

## How do I change the components being inserted?

In `src/main.rs` there is a variable called `input_component`. Change the value of this to change the components created. The structure the variable `input_component` should follow is described in `src/component_types.rs`. What each field does is described below:

```
Component {
  tag: describes the tag for the HTML component e.g. "div" will lead to a <div></div> component being created,
  meta: lists the attributes for the component. This can be used to change the "style" or "class" of the component,
  child: describes either the string being inserted (e.g. "Hello World" creates a <div>Hello World</div> component) or another component that will act a as child component
}
```
