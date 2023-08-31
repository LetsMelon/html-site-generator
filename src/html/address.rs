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

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::html::test_harness::test_generates_correct_html;

    test_generates_correct_html!(generates_correct_html_empty, { Address::new() });

    test_generates_correct_html!({
        let mut a = Address::new();

        a.add_element("Cou can write mails to: PRIVATE");

        a
    });
}
