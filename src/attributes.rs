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
    #[builder(setter(into), default)]
    id: Vec<String>,

    /// Specifies whether the content of an element is editable or not
    #[builder(setter, default = "false")]
    content_editable: bool,

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

// TODO add docs
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

    fn add_id<S: Into<String>>(&mut self, id: S) {
        let attributes = self.get_attributes_mut();

        attributes.id.push(id.into())
    }

    fn clear_id(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.id = Vec::new()
    }

    fn set_content_editable(&mut self, value: bool) {
        let attributes = self.get_attributes_mut();
        attributes.content_editable = value;
    }

    fn set_hidden(&mut self, value: bool) {
        let attributes = self.get_attributes_mut();
        attributes.hidden = value;
    }

    fn set_style<S: Into<String>>(&mut self, value: S) {
        let attributes = self.get_attributes_mut();
        attributes.style = Some(value.into())
    }

    fn clear_style(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.style = None;
    }

    fn set_tab_index(&mut self, value: usize) {
        let attributes = self.get_attributes_mut();
        attributes.tab_index = Some(value);
    }

    fn clear_tab_index(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.tab_index = None;
    }

    fn set_title<S: Into<String>>(&mut self, value: S) {
        let attributes = self.get_attributes_mut();
        attributes.title = Some(value.into());
    }

    fn clear_title(&mut self) {
        let attributes = self.get_attributes_mut();
        attributes.title = None;
    }
}

impl IntoHtmlNode for HtmlAttributes {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> Result<()> {
        if !self.class.is_empty() {
            write!(buffer, " class=\"{}\"", self.class.join(" "))?;
        }

        if !self.id.is_empty() {
            write!(buffer, " id=\"{}\"", self.id.join(" "))?;
        }

        if self.content_editable {
            write!(buffer, " contenteditable=\"{}\"", self.content_editable)?;
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

    fn transform_into_raw_css(&self, _buffer: &mut dyn Write) -> Result<()> {
        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
