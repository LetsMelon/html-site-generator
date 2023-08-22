use std::collections::HashMap;

use anyhow::Result;

use crate::strukt::object::text::TextObject;
use crate::transform::TransformAsHtml;

#[derive(Debug, Clone)]
pub enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub item: String,
}

impl TransformAsHtml for ListItem {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../../templates/list_item.html"));

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        let mut map = HashMap::new();

        map.insert("item", self.item.clone());

        Ok(map)
    }
}

#[derive(Debug, Clone)]
pub struct ListObject {
    pub text: TextObject,
    pub items: Vec<ListItem>,
    pub list_type: ListType,
}

impl TransformAsHtml for ListObject {
    const TEMPLATE: Option<&'static str> = Some(include_str!("../../templates/list_object.html"));

    fn as_map(&self) -> Result<HashMap<&str, String>> {
        let mut map = HashMap::new();

        map.insert("text", self.text.render()?);
        map.insert("items", self.items.render()?);
        map.insert(
            "list_type",
            match self.list_type {
                ListType::Ordered => "ol".to_string(),
                ListType::Unordered => "ul".to_string(),
            },
        );

        Ok(map)
    }
}
