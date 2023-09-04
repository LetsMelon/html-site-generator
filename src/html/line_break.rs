use std::io::Write;

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult, IsParagraph};

#[derive(Debug)]
pub struct LineBreak;

impl LineBreak {
    pub fn new() -> Self {
        LineBreak
    }
}

impl IntoHtmlNode for LineBreak {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        write!(buffer, "<br />")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
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

        self.transform_into_raw_html(&mut vec).unwrap();

        String::from_utf8(vec).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::LineBreak;
    use crate::html::paragraph::Paragraph;
    use crate::html::test_harness::test_generates_correct_html;

    test_generates_correct_html!(LineBreak::new());

    test_generates_correct_html!(generates_correct_html_inside_paragraph, {
        let mut p = Paragraph::new();

        p.add_element("Hey!");
        p.add_element(LineBreak::new());
        p.add_element("I'm on the next line!");

        p
    });
}
