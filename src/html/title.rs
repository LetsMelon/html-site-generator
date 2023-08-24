use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Title {
    text: String,
}

impl Title {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Title {
            text: content.into(),
        }
    }
}

impl IntoHtmlNode for Title {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<title>{}</title>", self.text)?;

        Ok(())
    }
}
