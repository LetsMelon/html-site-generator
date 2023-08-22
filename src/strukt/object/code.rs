use std::collections::HashMap;

use anyhow::Result;

use crate::transform::TransformAsHtml;

#[derive(Debug, Clone)]
pub enum CodeLanguage {
    Rust,
}

impl CodeLanguage {
    fn into_class(&self) -> String {
        let suffix = match self {
            CodeLanguage::Rust => "rust",
        };

        format!("language-{}", suffix)
    }
}

#[derive(Debug, Clone)]
pub struct CodeObject {
    pub language: CodeLanguage,
    pub code: String,
}

impl TransformAsHtml for CodeObject {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../../templates/code.html"));

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        let mut map = HashMap::new();

        map.insert("language", self.language.into_class());
        map.insert("body", self.code.clone());

        Ok(map)
    }
}
