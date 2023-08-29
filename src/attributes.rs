use std::io::Write;

use anyhow::Result;
use derive_builder::Builder;

use crate::html::IntoHtmlNode;

/// HTML Global Attributes: [link](https://www.w3schools.com/tags/ref_standardattributes.asp)
#[derive(Debug, Default, Builder)]
pub struct HtmlAttributes {
    /// Specifies one or more classnames for an element (refers to a class in a style sheet)
    #[builder(setter(into), default)]
    class: Vec<String>,

    /// Specifies a unique id for an element
    #[builder(setter(strip_option, into), default)]
    id: Option<String>,

    /// Specifies whether the content of an element is editable or not
    #[builder(setter, default = "false")]
    contenteditable: bool,

    /// Specifies that an element is not yet, or is no longer, relevant
    #[builder(setter, default = "false")]
    hidden: bool,

    /// Specifies an inline CSS style for an element
    #[builder(setter(strip_option, into), default)]
    style: Option<String>,

    /// Specifies the tabbing order of an element
    #[builder(setter(strip_option), default)]
    tab_index: Option<usize>,

    /// Specifies extra information about an element
    #[builder(setter(strip_option, into), default)]
    title: Option<String>,
}

// TODO add missing 'get/setter' methods
pub trait SetHtmlAttributes {
    fn get_attributes(&self) -> &HtmlAttributes;
    fn get_attributes_mut(&mut self) -> &mut HtmlAttributes;

    fn add_class<S: Into<String>>(&mut self, class: S) {
        let attributes = self.get_attributes_mut();

        attributes.class.push(class.into());
    }

    fn clear_class(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.class = Vec::new()
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
        write!(buffer, " class=\"{}\"", self.class.join(" "))?;

        if let Some(id) = &self.id {
            write!(buffer, " id=\"{}\"", id)?;
        }

        if self.contenteditable {
            write!(buffer, " contenteditable=\"{}\"", self.contenteditable)?;
        }

        if self.hidden {
            write!(buffer, " hidden")?;
        }

        if let Some(style) = &self.style {
            write!(buffer, " style=\"{}\"", style)?;
        }

        if let Some(index) = &self.tab_index {
            write!(buffer, " tabindex=\"{}\"", index)?;
        }

        if let Some(index) = &self.title {
            write!(buffer, " title=\"{}\"", index)?;
        }

        Ok(())
    }
}
