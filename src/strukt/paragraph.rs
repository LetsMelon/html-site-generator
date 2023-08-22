use std::io::Write;

use anyhow::Result;

use crate::strukt::object::Object;
use crate::transform::TransformAsHtml;

#[derive(Debug)]
pub struct Paragraph {
    pub objects: Vec<Object>,
}

impl TransformAsHtml for Paragraph {
    const TEMPLATE: Option<&'static str> = None;

    fn transform<W: Write>(&self, writer: &mut W) -> Result<()> {
        for object in &self.objects {
            object.transform(writer)?;
        }

        Ok(())
    }
}
