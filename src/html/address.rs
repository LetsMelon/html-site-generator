use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Address {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Address {
    pub fn new() -> Self {
        Address {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Address {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<address>")?;

        for element in &self.elements {
            element.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</address>")?;

        Ok(())
    }
}
