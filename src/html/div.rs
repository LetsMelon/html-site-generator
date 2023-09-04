use std::io::Write;

use html_site_generator_macro::{add_attributes_field, DeriveSetHtmlAttributes};

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

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
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        write!(buffer, "<div")?;
        self._attributes.transform_into_raw_html(buffer)?;
        writeln!(buffer, ">")?;

        for element in &self.elements {
            element.transform_into_raw_html(buffer)?;
        }

        writeln!(buffer, "</div>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self._attributes.transform_into_raw_css(buffer)?;

        for element in &self.elements {
            element.transform_into_raw_css(buffer)?;
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self._attributes.transform_into_raw_js(buffer)?;

        for element in &self.elements {
            element.transform_into_raw_js(buffer)?;
        }

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
