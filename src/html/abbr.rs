use html_site_generator_macro::{add_attributes_field, DeriveSetHtmlAttributes};

use super::{IntoHtmlNode, IsParagraph};

#[add_attributes_field]
#[derive(Debug, DeriveSetHtmlAttributes)]
pub struct Abbr {
    value: String,
}

impl Abbr {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Abbr {
            value: text.into(),
            _attributes: Default::default(),
        }
    }
}

impl IsParagraph for Abbr {
    fn to_raw(&self) -> String {
        let mut vec = Vec::new();
        self._attributes.transform_into_html_node(&mut vec).unwrap();

        format!(
            "<abbr{}>{}</abbr>",
            String::from_utf8(vec).unwrap(),
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Abbr;
    use crate::attributes::SetHtmlAttributes;
    use crate::html::paragraph::Paragraph;
    use crate::html::test_harness::test_generates_correct_html;
    use crate::html::IsParagraph;

    #[test]
    fn can_be_inside_paragraph() {
        let mut p = Paragraph::new();
        p.add_element("The next word is a ");
        p.add_element(Abbr::new("Abbr"));
        p.add_element("!");

        assert_eq!(p.to_raw(), "The next word is a <abbr>Abbr</abbr>!");
    }

    #[test]
    fn attributes_can_be_set() {
        let mut p = Paragraph::new();
        p.add_element("The next word is a ");
        p.add_element({
            let mut a = Abbr::new("Abbr");
            a.set_title("Some html element");
            a
        });
        p.add_element("!");

        assert_eq!(
            p.to_raw(),
            "The next word is a <abbr title=\"Some html element\">Abbr</abbr>!"
        );
    }

    test_generates_correct_html!({
        let mut p = Paragraph::new();
        p.add_element("The next word is a ");
        p.add_element({
            let mut a = Abbr::new("Abbr");
            a.set_title("Some html element");
            a
        });
        p.add_element("!");
        p
    });
}
