use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug, Clone, Default)]
pub enum ListType {
    Ordered,
    #[default]
    Unordered,
}

impl ListType {
    fn get_tags(&self) -> [&str; 2] {
        match self {
            ListType::Ordered => ["<ol>", "</ol>"],
            ListType::Unordered => ["<ul>", "</ul>"],
        }
    }
}

#[derive(Debug)]
pub struct List {
    elements: Vec<Box<dyn IntoHtmlNode>>,
    list_type: ListType,
}

impl List {
    pub fn new() -> Self {
        List {
            elements: Vec::new(),
            list_type: ListType::default(),
        }
    }

    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.elements.push(Box::new(item))
    }
}

impl IntoHtmlNode for List {
    fn transform_into_html_node(&self, buffer: &mut Box<dyn Write>) -> Result<()> {
        let [suffix, prefix] = self.list_type.get_tags();

        writeln!(buffer, "{}", suffix)?;

        for element in &self.elements {
            writeln!(buffer, "<li>")?;

            element.transform_into_html_node(buffer)?;

            writeln!(buffer, "</li>")?;
        }

        writeln!(buffer, "{}", prefix)?;

        Ok(())
    }
}
