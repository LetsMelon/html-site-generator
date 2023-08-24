use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct LineBreak;

impl LineBreak {
    pub fn new() -> Self {
        LineBreak
    }
}

impl IntoHtmlNode for LineBreak {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "</br>")?;

        Ok(())
    }
}
