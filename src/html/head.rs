use std::io::Write;

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

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
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        writeln!(buffer, "<head>")?;

        for element in &self.elements {
            element.transform_into_raw_html(buffer)?;
        }

        writeln!(buffer, "</head>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        for element in &self.elements {
            element.transform_into_raw_css(buffer)?;
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        for element in &self.elements {
            element.transform_into_raw_js(buffer)?;
        }

        Ok(())
    }
}
