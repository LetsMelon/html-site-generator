use std::io::Write;

use crate::html::IntoHtmlNode;

macro_rules! generate_holder_struct {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            data: Vec<u8>,
        }

        impl $name {
            pub(crate) fn new() -> Self {
                Self {
                    data: Default::default(),
                }
            }

            pub fn data(&self) -> &[u8] {
                &self.data
            }
        }

        impl std::io::Write for $name {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                let len = buf.len();

                self.data.extend_from_slice(buf);

                Ok(len)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                // TODO I'm not quit sure if this method should do something
                Ok(())
            }
        }
    };
}

generate_holder_struct!(HtmlWriter);
generate_holder_struct!(CssWriter);
generate_holder_struct!(JsWriter);

#[derive(Debug)]
pub struct RawWriter {
    data_html: HtmlWriter,
    data_css: CssWriter,
    data_js: JsWriter,
}

impl RawWriter {
    pub fn new() -> Self {
        RawWriter {
            data_html: HtmlWriter::new(),
            data_css: CssWriter::new(),
            data_js: JsWriter::new(),
        }
    }

    pub fn writers(&mut self) -> (&mut HtmlWriter, &mut CssWriter, &mut JsWriter) {
        (&mut self.data_html, &mut self.data_css, &mut self.data_js)
    }

    pub fn html_writer(&mut self) -> &mut HtmlWriter {
        &mut self.data_html
    }

    pub fn css_writer(&mut self) -> &mut CssWriter {
        &mut self.data_css
    }

    pub fn js_writer(&mut self) -> &mut JsWriter {
        &mut self.data_js
    }

    pub fn transform_into_single_document<W: Write>(self, buffer: &mut W) -> anyhow::Result<()> {
        writeln!(buffer, "<!DOCTYPE html>")?;
        writeln!(buffer, "<html>")?;
        writeln!(buffer, "<head>")?;

        // TODO implement writer for head

        if !self.data_css.data.is_empty() {
            writeln!(buffer, "<style>")?;
            buffer.write_all(&self.data_css.data)?;
            writeln!(buffer, "</style>")?;
        }

        if !self.data_js.data.is_empty() {
            writeln!(buffer, "<script>")?;
            buffer.write_all(&self.data_js.data)?;
            writeln!(buffer, "</script>")?;
        }

        writeln!(buffer, "</head>")?;

        if !self.data_html.data.is_empty() {
            // TODO check if `self.data_html` contains a `body` already
            writeln!(buffer, "<body>")?;
            buffer.write_all(&self.data_html.data)?;
            writeln!(buffer, "</body>")?;
        }

        writeln!(buffer, "</html>")?;

        Ok(())
    }
}

impl IntoHtmlNode for RawWriter {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> anyhow::Result<()> {
        buffer.write_all(&self.data_html.data)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> anyhow::Result<()> {
        buffer.write_all(&self.data_css.data)?;

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> anyhow::Result<()> {
        buffer.write_all(&self.data_js.data)?;

        Ok(())
    }
}
