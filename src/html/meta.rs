use std::collections::HashMap;
use std::io::Write;

use crate::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug)]
pub struct Meta {
    values: HashMap<String, String>,
}

impl Meta {
    pub fn new() -> Self {
        Meta {
            values: HashMap::new(),
        }
    }

    pub fn add_pair<S: Into<String>>(&mut self, key: S, value: S) {
        self.values.insert(key.into(), value.into());
    }
}

impl IntoHtmlNode for Meta {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        write!(buffer, "<meta")?;

        for (key, value) in self.values.iter() {
            write!(buffer, " {}=\"{}\"", key, value)?;
        }

        write!(buffer, "/>")?;

        Ok(())
    }

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }
}
