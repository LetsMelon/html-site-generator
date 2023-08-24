use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Head {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Head {
    pub fn new() -> Self {
        Head {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Head {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<head>")?;

        for element in &self.elements {
            element.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</head>")?;

        Ok(())
    }
}
