use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Div {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Div {
    pub fn new() -> Self {
        Div {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Div {
    fn transform_into_html_node(&self, buffer: &mut Box<dyn Write>) -> Result<()> {
        writeln!(buffer, "<div>")?;

        for element in &self.elements {
            element.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</div>")?;

        Ok(())
    }
}
