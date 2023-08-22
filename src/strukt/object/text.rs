use std::collections::HashMap;

use crate::transform::TransformAsHtml;

#[derive(Debug, Clone)]
pub struct TextObject {
    pub content: String,
}

impl TransformAsHtml for TextObject {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../../templates/text.html"));

    fn as_map(&self) -> anyhow::Result<std::collections::HashMap<&str, String>> {
        let mut map = HashMap::new();

        map.insert("content", self.content.clone());

        Ok(map)
    }
}
