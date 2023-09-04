use std::io::Write;

use html_site_generator_macro::{add_attributes_field, DeriveSetHtmlAttributes};

use crate::attributes::HtmlAttributes;
use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug, Clone, Default)]
pub enum ListType {
    Ordered,
    #[default]
    Unordered,
}

impl ListType {
    fn get_tags(&self) -> &str {
        match self {
            ListType::Ordered => "ol",
            ListType::Unordered => "ul",
        }
    }
}

#[add_attributes_field]
#[derive(Debug, DeriveSetHtmlAttributes)]
pub struct List {
    elements: Vec<(Box<dyn IntoHtmlNode>, HtmlAttributes)>,
    list_type: ListType,
}

impl List {
    pub fn new() -> Self {
        Self::new_with_ordering(ListType::default())
    }

    pub fn new_with_ordering(ordering: ListType) -> Self {
        List {
            elements: Vec::new(),
            list_type: ordering,
            _attributes: Default::default(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.add_element_with_attributes(item, HtmlAttributes::default())
    }

    pub fn add_element_with_attributes(
        &mut self,
        item: impl IntoHtmlNode + 'static,
        attributes: HtmlAttributes,
    ) {
        self.elements.push((Box::new(item), attributes))
    }
}

impl IntoHtmlNode for List {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        let symbol = self.list_type.get_tags();

        write!(buffer, "<{}", symbol)?;
        self._attributes.transform_into_raw_html(buffer)?;
        writeln!(buffer, ">")?;

        for (element, attribute) in &self.elements {
            write!(buffer, "<li")?;
            attribute.transform_into_raw_html(buffer)?;
            writeln!(buffer, ">")?;

            element.transform_into_raw_html(buffer)?;

            writeln!(buffer, "</li>")?;
        }

        writeln!(buffer, "</{}>", symbol)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        for (element, _) in &self.elements {
            element.transform_into_raw_css(buffer)?;
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        for (element, _) in &self.elements {
            element.transform_into_raw_js(buffer)?;
        }

        Ok(())
    }
}
