use std::io::Write;

use anyhow::Result;
use derive_builder::Builder;
use html_site_generator_macro::DeriveSetHtmlAttributes;

use crate::attributes::HtmlAttributes;
use crate::html::{IntoHtmlNode, IsParagraph};

/// Specifies which referrer information to send with the link
#[derive(Debug, Clone)]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl ReferrerPolicy {
    pub(crate) fn to_html_string(&self) -> &'static str {
        match self {
            ReferrerPolicy::NoReferrer => "no-referrer",
            ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicy::Origin => "origin",
            ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicy::SameOrigin => "same-origin",
            ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            ReferrerPolicy::UnsafeUrl => "unsafe-url",
        }
    }
}

/// Specifies the relationship between the current document and the linked document
#[derive(Debug, Clone)]
pub enum Relationship {
    Alternate,
    Author,
    Bookmark,
    External,
    Help,
    License,
    Next,
    NoFollow,
    NoReferrer,
    NoOpener,
    Prev,
    Search,
    Tag,
}

impl Relationship {
    fn to_html_string(&self) -> &'static str {
        match self {
            Relationship::Alternate => "alternate",
            Relationship::Author => "author",
            Relationship::Bookmark => "bookmark",
            Relationship::External => "external",
            Relationship::Help => "help",
            Relationship::License => "license",
            Relationship::Next => "next",
            Relationship::NoFollow => "nofollow",
            Relationship::NoReferrer => "noreferrer",
            Relationship::NoOpener => "noopener",
            Relationship::Prev => "prev",
            Relationship::Search => "search",
            Relationship::Tag => "tag",
        }
    }
}

/// Specifies where to open the linked document
#[derive(Debug, Clone)]
pub enum Target {
    Blank,
    Parent,
    /// => `Self`
    SSelf,
    Top,
}

impl Target {
    fn to_html_string(&self) -> &'static str {
        match self {
            Target::Blank => "_blank",
            Target::Parent => "_parent",
            Target::SSelf => "_self",
            Target::Top => "_top",
        }
    }
}

/// HTML Attribute: [`<a>`](https://www.w3schools.com/tags/tag_a.asp)
#[derive(Debug, Builder, DeriveSetHtmlAttributes)]
pub struct Hyperlink {
    /// Specifies that the target will be downloaded when a user clicks on the hyperlink
    #[builder(setter(strip_option, into), default)]
    download: Option<String>,

    /// Specifies the URL of the page the link goes to
    #[builder(setter(strip_option, into), default)]
    href: Option<String>,

    /// Specifies the language of the linked document
    #[builder(setter(strip_option, into), default)]
    href_lang: Option<String>,

    /// Specifies what media/device the linked document is optimized for
    #[builder(setter(strip_option, into), default)]
    media: Option<String>,

    /// Specifies a space-separated list of URLs to which, when the link is followed,
    /// post requests with the body ping will be sent by the browser (in the background).
    /// Typically used for tracking.
    #[builder(setter(strip_option, into), default)]
    ping: Option<String>,

    /// Specifies which referrer information to send with the link
    #[builder(setter(strip_option, into), default)]
    referrer_policy: Option<ReferrerPolicy>,

    /// Specifies the relationship between the current document and the linked document
    #[builder(default = "Some(vec![Relationship::NoOpener, Relationship::NoReferrer])")]
    rel: Option<Vec<Relationship>>,

    /// Specifies where to open the linked document
    #[builder(default = "Some(Target::Blank)")]
    target: Option<Target>,

    // TODO maybe use a crate with mime types
    /// Specifies the media type of the linked document
    #[builder(setter(strip_option, into), default)]
    mime_type: Option<String>,

    #[builder(setter(skip))]
    children: Vec<Box<dyn IntoHtmlNode>>,

    #[builder(setter(skip))]
    _attributes: HtmlAttributes,
}

impl Hyperlink {
    pub fn add_element(&mut self, item: impl IntoHtmlNode + 'static) {
        self.children.push(Box::new(item))
    }
}

impl IntoHtmlNode for Hyperlink {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<a")?;

        if let Some(value) = &self.download {
            write!(buffer, " download=\"{}\"", value)?;
        }
        if let Some(value) = &self.href {
            write!(buffer, " href=\"{}\"", value)?;
        }
        if let Some(value) = &self.href_lang {
            write!(buffer, " hreflang=\"{}\"", value)?;
        }
        if let Some(value) = &self.media {
            write!(buffer, " media=\"{}\"", value)?;
        }
        if let Some(value) = &self.ping {
            write!(buffer, " ping=\"{}\"", value)?;
        }
        if let Some(value) = &self.referrer_policy {
            write!(buffer, "  referrerpolicy=\"{}\"", value.to_html_string())?;
        }
        if let Some(value) = &self.rel {
            let raw = value
                .iter()
                .map(|item| item.to_html_string())
                .enumerate()
                .fold("".to_string(), |acc, (i, item)| {
                    acc + if i == 0 { "" } else { " " } + item
                });

            write!(buffer, " rel=\"{}\"", raw)?;
        }
        if let Some(value) = &self.target {
            write!(buffer, " target=\"{}\"", value.to_html_string())?;
        }
        if let Some(value) = &self.mime_type {
            write!(buffer, " type=\"{}\"", value)?;
        }

        write!(buffer, ">")?;

        for child in &self.children {
            child.transform_into_html_node(buffer)?;
        }

        writeln!(buffer, "</a>")?;

        Ok(())
    }
}

// TODO I'm not quit sure that `Hyperlink` implements the trait `IsParagraph`.
// Maybe there is another, and better way, to do this?
impl IsParagraph for Hyperlink {
    fn to_raw(&self) -> String {
        let mut vec = Vec::new();

        self.transform_into_html_node(&mut vec).unwrap();

        String::from_utf8(vec).unwrap()
    }
}
