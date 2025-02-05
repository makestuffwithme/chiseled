use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ItemMapping {
    result: Vec<CategoryMapping>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryMapping {
    id: String,
    label: String,
    entries: Vec<ItemEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemEntry {
    #[serde(rename = "type")]
    base_type: String,
    text: Option<String>,
    name: Option<String>,
    flags: Option<ItemFlags>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemFlags {
    unique: Option<bool>,
}

pub struct BaseTypeMap {
    mapping: HashMap<String, String>,
}

impl BaseTypeMap {
    pub fn new(json: Value) -> Result<Self, String> {
        let mapping: ItemMapping = serde_json::from_value(json)
            .map_err(|e| format!("Failed to parse item mapping: {}", e))?;

        let mut map = HashMap::new();
        for category in mapping.result {
            for entry in category.entries {
                // Only add non-unique items as base types
                if entry.flags.is_none() || !entry.flags.unwrap().unique.unwrap_or(false) {
                    map.insert(entry.base_type, category.id.clone());
                }
            }
        }

        Ok(Self { mapping: map })
    }

    pub fn item_text_to_base_type(&self, item_text: &str) -> Option<(String, String)> {
        let words: Vec<&str> = item_text.split_whitespace().collect();

        // Try all possible contiguous combinations, starting with longer ones
        for start in 0..words.len() {
            for len in (1..=words.len() - start).rev() {
                let combination = words[start..start + len].join(" ");
                if let Some(category) = self.mapping.get(&combination) {
                    return Some((combination.clone(), category.clone()));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_item_text_to_base_type() {
        // Create test JSON
        let json = serde_json::json!({
            "result": [{
                "id": "weapon",
                "label": "Weapons",
                "entries": [
                    {
                        "type": "Bombard Crossbow",
                        "text": "Bombard Crossbow",
                        "flags": { "unique": false }
                    },
                    {
                        "type": "Advanced Dualstring Bow",
                        "text": "Advanced Dualstring Bow",
                        "flags": { "unique": false }
                    }
                ]
            }]
        });

        let base_type_map = BaseTypeMap::new(json).unwrap();

        const MAGIC_ITEM_NAME: &str = "Deliberate Bombard Crossbow of Mastery";
        let (base_type, category) = base_type_map
            .item_text_to_base_type(MAGIC_ITEM_NAME)
            .unwrap();
        assert_eq!(base_type, "Bombard Crossbow");
        assert_eq!(category, "weapon");

        const RARE_ITEM_NAME: &str = "Advanced Dualstring Bow";
        let (base_type, category) = base_type_map
            .item_text_to_base_type(RARE_ITEM_NAME)
            .unwrap();
        assert_eq!(base_type, "Advanced Dualstring Bow");
        assert_eq!(category, "weapon");
    }
}
