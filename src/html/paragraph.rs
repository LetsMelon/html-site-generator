use std::io::Write;

use anyhow::Result;
use html_site_generator_macro::{add_attributes_field, DeriveSetHtmlAttributes};

use crate::html::{IntoHtmlNode, IsParagraph};

#[add_attributes_field]
#[derive(Debug, DeriveSetHtmlAttributes)]
pub struct Paragraph {
    elements: Vec<Box<dyn IsParagraph>>,
}

impl Paragraph {
    pub fn new() -> Self {
        Paragraph {
            _attributes: Default::default(),
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, item: impl IsParagraph + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IsParagraph for Paragraph {
    fn to_raw(&self) -> String {
        let mut s = String::new();

        // TODO maybe use something like https://crates.io/crates/string-builder, if a this (or another crate) is faster than this method.
        for element in &self.elements {
            s.push_str(&element.to_raw());
        }

        s
    }
}

impl IntoHtmlNode for Paragraph {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<p>")?;

        writeln!(buffer, "{}", self.to_raw())?;

        writeln!(buffer, "</p>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> Result<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
