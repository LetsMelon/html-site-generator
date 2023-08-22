use std::io::Write;

use anyhow::Result;

use crate::strukt::object::code::CodeObject;
use crate::strukt::object::image::ImageObject;
use crate::strukt::object::list::ListObject;
use crate::strukt::object::text::TextObject;
use crate::transform::TransformAsHtml;

pub mod code;
pub mod image;
pub mod list;
pub mod text;

#[derive(Debug, Clone)]
pub enum Object {
    Text(TextObject),
    Image(ImageObject),
    List(ListObject),
    Code(CodeObject),
}

impl TransformAsHtml for Object {
    const TEMPLATE: Option<&'static str> = None;

    fn transform<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Object::Text(content) => content.transform(writer)?,
            Object::Image(img) => img.transform(writer)?,
            Object::List(list) => list.transform(writer)?,
            Object::Code(code) => code.transform(writer)?,
        }

        Ok(())
    }
}
