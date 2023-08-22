use std::collections::HashMap;

use anyhow::Result;

use crate::datetime::DateTime;
use crate::strukt::paragraph::Paragraph;
use crate::transform::TransformAsHtml;

#[derive(Debug)]
pub struct Document {
    pub title: String,
    pub keywords: Vec<String>,
    pub publish_date: DateTime,
    pub paragraphs: Vec<Paragraph>,
}

impl TransformAsHtml for Document {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../templates/document.html"));

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        let mut map = HashMap::new();

        let body = self.paragraphs.render()?;

        map.insert("title", self.title.clone());
        map.insert("body", body);
        map.insert(
            "style",
            include_str!("../templates/document.css").to_string(),
        );

        Ok(map)
    }
}
