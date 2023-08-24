use std::io::Write;

use anyhow::Result;
use derive_builder::Builder;

use crate::html::IntoHtmlNode;

#[derive(Debug, Default, Builder)]
pub struct HtmlAttributes {
    #[builder(setter(strip_option, into), default)]
    class: Option<String>,
    #[builder(setter(strip_option, into), default)]
    id: Option<String>,
}

pub trait SetHtmlAttributes {
    fn get_attributes(&self) -> &HtmlAttributes;
    fn get_attributes_mut(&mut self) -> &mut HtmlAttributes;

    fn set_class<S: Into<String>>(&mut self, class: S) {
        let attributes = self.get_attributes_mut();
        attributes.class = Some(class.into())
    }

    fn clear_class(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.class = None
    }

    fn set_id<S: Into<String>>(&mut self, id: S) {
        let attributes = self.get_attributes_mut();
        attributes.id = Some(id.into())
    }

    fn clear_id(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.id = None
    }
}

impl IntoHtmlNode for HtmlAttributes {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        if let Some(class) = &self.class {
            write!(buffer, " class=\"{}\"", class)?;
        }

        if let Some(id) = &self.id {
            write!(buffer, " id=\"{}\"", id)?;
        }

        Ok(())
    }
}
