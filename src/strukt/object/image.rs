use std::collections::HashMap;

use anyhow::Result;

use crate::transform::TransformAsHtml;

#[derive(Debug, Clone)]
pub struct ImageObject {
    pub path: String,
    pub alt_text: String,
    pub width: u32,
}

impl TransformAsHtml for ImageObject {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../../templates/image.html"));

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        let mut map = HashMap::new();

        map.insert("path", self.path.clone());
        map.insert("alt_text", self.alt_text.clone());
        map.insert("width", format!("{}px", self.width));

        Ok(map)
    }
}
