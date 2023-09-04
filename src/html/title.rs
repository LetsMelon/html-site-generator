use std::io::Write;

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

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
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        writeln!(buffer, "<title>{}</title>", self.text)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }
}
