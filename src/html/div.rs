use std::io::Write;

use anyhow::Result;
use html_site_generator_macro::{add_attributes_field, DeriveSetHtmlAttributes};

use crate::html::IntoHtmlNode;

#[add_attributes_field]
#[derive(Debug, DeriveSetHtmlAttributes)]
pub struct Div {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Div {
    pub fn new() -> Self {
        Div {
            _attributes: Default::default(),
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Div {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<div")?;
        self._attributes.transform_into_html_node(buffer)?;
        writeln!(buffer, ">")?;

        for element in &self.elements {
            element.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</div>")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Div;
    use crate::html::test_harness::test_generates_correct_html;

    test_generates_correct_html!(generates_correct_html_empty, { Div::new() });

    test_generates_correct_html!({
        let mut d = Div::new();
        d.add_element("My text inside a div!");

        d
    });
}
