use crate::html::body::Body;
use crate::html::head::Head;
use crate::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Document {
    head: Head,
    body: Body,
}

impl Document {
    pub fn new(head: Head, body: Body) -> Self {
        Document { head, body }
    }
}

impl IntoHtmlNode for Document {
    fn transform_into_html_node(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        writeln!(buffer, "<!DOCTYPE html>")?;
        writeln!(buffer, "<html>")?;

        self.head.transform_into_html_node(buffer)?;
        self.body.transform_into_html_node(buffer)?;

        writeln!(buffer, "</html>")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Document;
    use crate::attributes::SetHtmlAttributes;
    use crate::html::body::Body;
    use crate::html::div::Div;
    use crate::html::head::Head;
    use crate::html::meta::Meta;
    use crate::html::test_harness::test_generates_correct_html;
    use crate::html::title::Title;

    test_generates_correct_html!(generates_correct_html_empty, {
        Document::new(Head::new(), Body::new())
    });

    test_generates_correct_html!({
        let mut head = Head::new();
        head.add_element(Title::new("test_page"));
        head.add_element({
            let mut m = Meta::new();
            m.add_pair("CHARSET", "UTF-8");
            m
        });

        let mut body = Body::new();
        body.add_element({
            let mut d = Div::new();
            d.add_class("test_div");

            d.add_element("Hello, I'm a text inside a div inside a document inside a test");

            d
        });

        Document::new(head, body)
    });
}
