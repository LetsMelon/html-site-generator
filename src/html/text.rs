use crate::html::IsParagraph;

#[derive(Debug, Default)]
pub enum TextElementStyling {
    #[default]
    Normal,
    Bold,
    Emphasized,
    Important,
    Highlighted,
}

impl TextElementStyling {
    fn get_tags(&self) -> [&str; 2] {
        match self {
            TextElementStyling::Normal => [""; 2],
            TextElementStyling::Bold => ["<b>", "</b>"],
            TextElementStyling::Emphasized => ["<em>", "</em>"],
            TextElementStyling::Important => ["<strong>", "</strong>"],
            TextElementStyling::Highlighted => ["<mark>", "</mark>"],
        }
    }
}

#[derive(Debug)]
pub struct TextElement {
    raw: String,
    tag: TextElementStyling,
}

impl TextElement {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self::new_with_styling(text, TextElementStyling::default())
    }

    pub fn new_with_styling<S: Into<String>>(text: S, styling: TextElementStyling) -> Self {
        TextElement {
            raw: text.into(),
            tag: styling,
        }
    }
}

impl IsParagraph for TextElement {
    fn to_raw(&self) -> String {
        let [suffix, prefix] = self.tag.get_tags();

        format!("{}{}{}", suffix, self.raw, prefix)
    }
}
