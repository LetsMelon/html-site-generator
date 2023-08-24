use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Footer {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Footer {
    pub fn new() -> Self {
        Footer {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Footer {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<footer>")?;

        for element in &self.elements {
            element.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</footer>")?;

        Ok(())
    }
}