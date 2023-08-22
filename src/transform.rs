use std::collections::HashMap;
use std::io::{Cursor, Write};

use anyhow::{bail, Result};
use new_string_template::template::Template;

pub trait TransformAsHtml {
    const TEMPLATE: Option<&'static str>;

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        Ok(HashMap::default())
    }

    fn transform<W: Write>(&self, writer: &mut W) -> Result<()>
    where
        Self: Sized,
    {
        let rendered = self.render()?;
        writer.write(rendered.as_bytes())?;

        Ok(())
    }

    fn render(&self) -> Result<String> {
        if Self::TEMPLATE.is_none() {
            bail!("Can't render this item without having a template");
        }

        let templ = Template::new(Self::TEMPLATE.unwrap());

        let data = self.as_map()?;
        let rendered = templ.render(&data)?;

        Ok(rendered)
    }
}

impl<T: TransformAsHtml> TransformAsHtml for Vec<T> {
    const TEMPLATE: Option<&'static str> = None;

    fn render(&self) -> Result<String> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        for item in self {
            item.transform(&mut cursor)?;
        }

        Ok(String::from_utf8(buffer)?)
    }
}
