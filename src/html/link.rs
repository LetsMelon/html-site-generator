use std::io::Write;

use anyhow::Result;
use derive_builder::Builder;

use crate::html::hyperlink::ReferrerPolicy;
use crate::html::IntoHtmlNode;

/// Specifies how the element handles cross-origin requests
#[derive(Debug, Clone)]
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
}

impl CrossOrigin {
    fn to_html_string(&self) -> &'static str {
        match self {
            CrossOrigin::Anonymous => "anonymous",
            CrossOrigin::UseCredentials => "use-credentials",
        }
    }
}

/// Specifies the relationship between the current document and the linked document
#[derive(Debug, Clone, Default)]
pub enum Relationship {
    Alternate,
    Author,
    DnsPrefetch,
    Help,
    Icon,
    License,
    Next,
    Pingback,
    Preconnect,
    Prefetch,
    Preload,
    Prerender,
    Prev,
    Search,
    #[default]
    Stylesheet,
}

impl Relationship {
    fn to_html_string(&self) -> &'static str {
        match self {
            Relationship::Alternate => "alternate",
            Relationship::Author => "author",
            Relationship::DnsPrefetch => "dns-prefetch",
            Relationship::Help => "help",
            Relationship::Icon => "icon",
            Relationship::License => "license",
            Relationship::Next => "next",
            Relationship::Pingback => "pingback",
            Relationship::Preconnect => "preconnect",
            Relationship::Prefetch => "prefetch",
            Relationship::Preload => "preload",
            Relationship::Prerender => "prerender",
            Relationship::Prev => "prev",
            Relationship::Search => "search",
            Relationship::Stylesheet => "stylesheet",
        }
    }
}

/// HTML Attribute: [`<link>`](https://www.w3schools.com/tags/tag_link.asp)
#[derive(Debug, Default, Builder)]
pub struct Link {
    /// Specifies how the element handles cross-origin requests
    #[builder(setter(strip_option), default)]
    crossorigin: Option<CrossOrigin>,

    /// Specifies the location of the linked document
    #[builder(setter(strip_option, into), default)]
    href: Option<String>,

    /// Specifies the language of the text in the linked document
    #[builder(setter(strip_option, into), default)]
    href_lang: Option<String>,

    /// Specifies on what device the linked document will be displayed
    #[builder(setter(strip_option, into), default)]
    media: Option<String>,

    /// Specifies which referrer to use when fetching the resource
    #[builder(setter(strip_option), default)]
    referrer_policy: Option<ReferrerPolicy>,

    /// Specifies the relationship between the current document and the linked document
    rel: Relationship,

    /// Specifies the size of the linked resource. Only for rel="icon"
    #[builder(setter(strip_option, into), default)]
    sizes: Option<String>,

    /// Defines a preferred or an alternate stylesheet
    #[builder(setter(strip_option, into), default)]
    title: Option<String>,

    // TODO maybe use a crate with mime types
    /// Specifies the media type of the linked document
    #[builder(setter(strip_option, into), default)]
    media_type: Option<String>,
}

impl IntoHtmlNode for Link {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        write!(buffer, "<link")?;

        if let Some(value) = &self.crossorigin {
            write!(buffer, " crossorigin=\"{}\"", value.to_html_string())?;
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
        if let Some(value) = &self.referrer_policy {
            write!(buffer, " referrerpolicy=\"{}\"", value.to_html_string())?;
        }
        write!(buffer, " rel=\"{}\"", self.rel.to_html_string())?;
        if let Some(value) = &self.sizes {
            write!(buffer, " sizes=\"{}\"", value)?;
        }
        if let Some(value) = &self.title {
            write!(buffer, " title=\"{}\"", value)?;
        }
        if let Some(value) = &self.title {
            write!(buffer, " title=\"{}\"", value)?;
        }
        if let Some(value) = &self.media_type {
            write!(buffer, " type=\"{}\"", value)?;
        }

        writeln!(buffer, ">")?;

        Ok(())
    }
}
