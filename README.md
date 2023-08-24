# html-site-generator

This crate aims to help you with creating html 'code' from Rust code.

An example how easy it is to create a div and add some children to it:

```rust
let mut div = Div::new();
div.set_class("card");

div.add_element("Name: html-site-generator");
div.add_element(LineBreak::new());
div.add_element("Languages: Rust");
```
