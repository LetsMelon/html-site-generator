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
