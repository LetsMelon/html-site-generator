use std::io::Write;

use anyhow::Result;
use derive_builder::Builder;
use html_site_generator_macro::DeriveSetHtmlAttributes;

use super::hyperlink::ReferrerPolicy;
use super::link::CrossOrigin;
use super::{IntoHtmlNode, IsParagraph};
use crate::attributes::HtmlAttributes;

/// Specifies whether a browser should load an image immediately or to defer loading of images until some conditions are met
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Loading {
    Eager,
    Lazy,
}

impl Loading {
    fn to_html_string(&self) -> &'static str {
        match self {
            Loading::Eager => "eager",
            Loading::Lazy => "lazy",
        }
    }
}

/// HTML Attribute: [`<img>`](https://www.w3schools.com/tags/tag_img.asp)
#[derive(Debug, Builder, DeriveSetHtmlAttributes)]
pub struct Image {
    /// Specifies an alternate text for an image
    #[builder(setter(strip_option, into), default)]
    alt: Option<String>,

    /// Allow images from third-party sites that allow cross-origin access to be used with canvas
    #[builder(setter(strip_option), default)]
    cross_origin: Option<CrossOrigin>,

    /// Specifies the height of an image
    #[builder(setter(strip_option), default)]
    height: Option<u32>,

    /// Specifies an image as a server-side image map
    #[builder(setter(strip_option), default = "false")]
    ismap: bool,

    /// Specifies whether a browser should load an image immediately or to defer loading of images until some conditions are met
    #[builder(setter(strip_option), default)]
    loading: Option<Loading>,

    // TODO not really supported https://www.w3schools.com/tags/att_img_longdesc.asp
    /// Specifies a URL to a detailed description of an image
    // #[builder(setter(strip_option, into), default)]
    // long_desc: Option<String>,

    /// Specifies which referrer information to use when fetching an image
    #[builder(setter(strip_option), default)]
    referrer_policy: Option<ReferrerPolicy>,

    // TODO add logic to check with `src_set`, https://www.dofactory.com/html/img/sizes
    /// Specifies image sizes for different page layouts
    // #[builder(setter(strip_option, into), default)]
    // sizes: Option<String>,

    /// Specifies the path to the image
    #[builder(setter(strip_option, into), default)]
    src: Option<String>,

    // TODO add logic to check with `sizes`, https://www.dofactory.com/html/img/sizes
    /// Specifies a list of image files to use in different situations
    // #[builder(setter(strip_option, into), default)]
    // src_set: Option<Vec<String>>,

    /// Specifies an image as a client-side image map
    // TODO implement html tag `map` before implementing this here in `Image`
    // #[builder(setter(strip_option, into), default)]
    // usemap: Option<String>,

    /// Specifies the width of an image
    #[builder(setter(strip_option), default)]
    width: Option<u32>,

    #[builder(setter(skip))]
    _attributes: HtmlAttributes,
}

impl IntoHtmlNode for Image {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<img")?;
        self._attributes.transform_into_html_node(buffer)?;

        if let Some(value) = &self.alt {
            write!(buffer, " alt=\"{}\"", value)?;
        }
        if let Some(value) = &self.cross_origin {
            write!(buffer, " crossorigin=\"{}\"", value.to_html_string())?;
        }
        if let Some(value) = &self.height {
            write!(buffer, " height=\"{}\"", value)?;
        }
        if let Some(value) = &self.loading {
            write!(buffer, " loading=\"{}\"", value.to_html_string())?;
        }
        if let Some(value) = &self.referrer_policy {
            write!(buffer, " referrerpolicy=\"{}\"", value.to_html_string())?;
        }
        if let Some(value) = &self.src {
            write!(buffer, " src=\"{}\"", value)?;
        }
        if let Some(value) = &self.width {
            write!(buffer, " width=\"{}\"", value)?;
        }

        if self.ismap {
            write!(buffer, " ismap")?;
        }

        write!(buffer, ">")?;

        Ok(())
    }
}

// TODO I'm not quit sure if `Image` implements the trait `IsParagraph`.
// But at the moment this is the only way to use `Hyperlink` in a `Paragraph`
// Maybe there is another, and better way, to do this?
// same as `Hyperlink`, `LineBreak`
impl IsParagraph for Image {
    fn to_raw(&self) -> String {
        let mut vec = Vec::new();

        self.transform_into_html_node(&mut vec).unwrap();

        String::from_utf8(vec).unwrap()
    }
}
