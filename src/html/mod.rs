use std::fmt::Debug;
use std::io::Write;

use crate::error::IntoHtmlNodeError;
use crate::raw_writer::RawWriter;

pub mod abbr;
pub mod address;
pub mod body;
pub mod div;
pub mod document;
pub mod footer;
pub mod head;
pub mod hyperlink;
pub mod image;
pub mod line_break;
pub mod link;
pub mod list;
pub mod meta;
pub mod paragraph;
pub mod text;
pub mod title;

pub type IntoHtmlNodeResult<T> = Result<T, IntoHtmlNodeError>;

pub trait IntoHtmlNode: Debug {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()>;
    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()>;
    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()>;

    fn transform_into_html_node_with_css_and_js(
        &self,
        writer: &mut RawWriter,
    ) -> IntoHtmlNodeResult<()> {
        let (mut html_writer, mut css_writer, mut js_writer) = writer.writers();

        self.transform_into_raw_html(&mut html_writer)?;
        self.transform_into_raw_css(&mut css_writer)?;
        self.transform_into_raw_js(&mut js_writer)?;

        Ok(())
    }
}

impl<S: AsRef<str> + Debug> IntoHtmlNode for S {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        let s = self.as_ref().to_string();

        // let mut p = Paragraph::new();
        // p.add_element(s);
        // p.transform_into_html_node(buffer)?;
        buffer.write_all(s.as_bytes())?;

        Ok(())
    }

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }
}

pub trait IsParagraph: Debug {
    fn to_raw(&self) -> String;
}

impl<S: AsRef<str> + Debug> IsParagraph for S {
    fn to_raw(&self) -> String {
        self.as_ref().to_string()
    }
}

// TODO I'm not quit sure if that is the correct way to have a macro only in the crate, I think I have to redo this sometime in the future. PR's or welcome
#[cfg(test)]
mod test_harness {
    // TODO add some docs about the usage of this macro
    macro_rules! test_generates_correct_html {
        ($name:ident, $code:expr) => {
            #[test]
            fn $name() {
                use html_parser::Dom;

                use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

                let item = $code;

                let mut buffer = Vec::new();
                item.transform_into_raw_html(&mut buffer).unwrap();
                let html = String::from_utf8(buffer).unwrap();
                assert!(Dom::parse(&html).is_ok());
            }
        };
        ($code:expr) => {
            test_generates_correct_html!(generates_correct_html, { $code });
        };
    }

    pub(crate) use test_generates_correct_html;
}
