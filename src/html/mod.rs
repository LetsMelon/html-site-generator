use std::fmt::Debug;
use std::io::Write;

use anyhow::Result;

pub mod div;
pub mod list;
pub mod paragraph;
pub mod text;

pub trait IntoHtmlNode: Debug {
    fn transform_into_html_node(&self, buffer: &mut Box<dyn Write>) -> Result<()>;
}

impl<S: AsRef<str> + Debug> IntoHtmlNode for S {
    fn transform_into_html_node(&self, buffer: &mut Box<dyn Write>) -> Result<()> {
        let s = self.as_ref();

        writeln!(buffer, "<p>{}</p>", s)?;

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
