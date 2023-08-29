use std::io::Write;

use anyhow::Result;

use crate::html::{IntoHtmlNode, IsParagraph};

#[derive(Debug)]
pub struct LineBreak;

impl LineBreak {
    pub fn new() -> Self {
        LineBreak
    }
}

impl IntoHtmlNode for LineBreak {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<br />")?;

        Ok(())
    }
}

// TODO I'm not quit sure if `LineBreak` should implement the trait `IsParagraph`.
// But at the moment this is the only way to use `Hyperlink` in a `Paragraph`
// Maybe there is another, and better way, to do this?
// same as `Image`, `Hyperlink`
impl IsParagraph for LineBreak {
    fn to_raw(&self) -> String {
        let mut vec = Vec::new();

        self.transform_into_html_node(&mut vec).unwrap();

        String::from_utf8(vec).unwrap()
    }
}
