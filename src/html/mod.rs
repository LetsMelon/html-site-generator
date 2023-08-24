use std::fmt::Debug;
use std::io::Write;

use anyhow::Result;

use self::paragraph::Paragraph;

pub mod address;
pub mod body;
pub mod div;
pub mod document;
pub mod footer;
pub mod head;
pub mod hyperlink;
pub mod line_break;
pub mod link;
pub mod list;
pub mod meta;
pub mod paragraph;
pub mod text;

pub trait IntoHtmlNode: Debug {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()>;
}

impl<S: AsRef<str> + Debug> IntoHtmlNode for S {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        let s = self.as_ref().to_string();

        let mut p = Paragraph::new();
        p.add_element(s);
        p.transform_into_html_node(buffer)?;

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
