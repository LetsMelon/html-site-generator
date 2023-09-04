use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Body {
    elements: Vec<Box<dyn IntoHtmlNode>>,
}

impl Body {
    pub fn new() -> Self {
        Body {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for Body {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<body>")?;

        for element in &self.elements {
            element.transform_into_raw_html(buffer)?;
        }

        writeln!(buffer, "</body>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> Result<()> {
        for element in &self.elements {
            element.transform_into_raw_css(buffer)?;
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> Result<()> {
        for element in &self.elements {
            element.transform_into_raw_js(buffer)?;
        }

        Ok(())
    }
}
