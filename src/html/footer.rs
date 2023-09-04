use std::io::Write;

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

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
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        writeln!(buffer, "<footer>")?;

        for element in &self.elements {
            element.transform_into_raw_html(buffer)?;
        }

        writeln!(buffer, "</footer>")?;

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

#[cfg(test)]
mod tests {
    use super::Footer;
    use crate::html::test_harness::test_generates_correct_html;

    test_generates_correct_html!({
        let mut f = Footer::new();

        f.add_element("Hihi, I'm a footer!");

        f
    });
}
