use std::collections::HashMap;
use std::io::Write;

use anyhow::Result;

use crate::html::IntoHtmlNode;

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
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<meta")?;

        for (key, value) in self.values.iter() {
            write!(buffer, " {}=\"{}\"", key, value)?;
        }

        writeln!(buffer, "/>")?;

        Ok(())
    }
}
