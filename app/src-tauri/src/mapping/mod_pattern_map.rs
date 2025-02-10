use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ModPatternMapping {
    result: Vec<ModPatternSection>,
}


#[derive(Debug, Serialize, Deserialize)]
struct ModPatternSection {
    entries: Vec<ModPatternEntry>,
}


#[derive(Debug, Serialize, Deserialize)]
struct ModPatternEntry {
    id: String,
    text: String,
}

pub struct ModPatternMap {
    mapping: HashMap<String, Vec<String>>,
}

impl ModPatternMap {
    pub fn new(json: Value) -> Result<Self, String> {
        let mapping: ModPatternMapping = serde_json::from_value(json)
            .map_err(|e| format!("Failed to parse stats mapping: {}", e))?;

        let mut pattern_map = HashMap::new();
        for section in mapping.result {
            for entry in section.entries {
                let pattern = normalize_pattern(&entry.text);
                pattern_map
                    .entry(pattern)
                    .or_insert_with(Vec::new)
                    .push(entry.id);
            }
        }

        Ok(Self {
            mapping: pattern_map,
        })
    }

    pub fn mod_pattern_to_trade_stat(&self, pattern: &str, prefix: &str) -> Option<String> {
        if let Some(trade_stats) = self.mapping.get(pattern) {
            for trade_stat in trade_stats {
                if trade_stat.starts_with(prefix) {
                    return Some(trade_stat.clone());
                }
            }
        }
        None
    }
}

fn normalize_pattern(text: &str) -> String {
    let mut normalized_words = Vec::new();
    let words = text.split(" ").collect::<Vec<_>>();
    for word in words {
        let clean_word = word
            .replace("[", "")
            .replace("]", "")
            .split("|")
            .last()
            .unwrap()
            .to_string();
        normalized_words.push(clean_word);
    }

    normalized_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pattern_mapping() {
        // Create test JSON
        let json = serde_json::json!({
            "result": [{
                "entries": [
                    { "id": "explicit.stat_803737631", "text": "# to Accuracy Rating" },
                    { "id": "explicit.stat_210067635", "text": "#% increased Attack Speed" },
                    { "id": "explicit.stat_1509134228", "text": "#% increased Physical Damage" },
                    { "id": "explicit.stat_1940865751", "text": "Adds # to # Physical Damage" },
                    { "id": "explicit.stat_3885405204", "text": "Bow Attacks fire # additional Arrows" },
                    { "id": "explicit.stat_669069897", "text": "Leeches #% of Physical Damage as Mana" },
                    { "id": "rune.stat_1509134228", "text": "#% increased Physical Damage" },
                    { "id": "implicit.stat_1980802737", "text": "Grenade Skills Fire an additional Projectile" }
                ]
            }]
        });

        let pattern_map = ModPatternMap::new(json).expect("Failed to create mod pattern map");

        // Test exact pattern matching
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("# to Accuracy Rating", "explicit"),
            Some("explicit.stat_803737631".to_string())
        );

        // Test pattern with multiple numbers
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("Adds # to # Physical Damage", "explicit"),
            Some("explicit.stat_1940865751".to_string())
        );

        // Test pattern with percentage
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("#% increased Attack Speed", "explicit"),
            Some("explicit.stat_210067635".to_string())
        );

        // Test same pattern with different prefix (explicit vs rune)
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("#% increased Physical Damage", "explicit"),
            Some("explicit.stat_1509134228".to_string())
        );
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("#% increased Physical Damage", "rune"),
            Some("rune.stat_1509134228".to_string())
        );

        // Test pattern with "additional" modifier
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("Bow Attacks fire # additional Arrows", "explicit"),
            Some("explicit.stat_3885405204".to_string())
        );

        // Test non-existent pattern
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("This mod doesn't exist", "explicit"),
            None
        );

        // Test existing pattern with wrong prefix
        assert_eq!(
            pattern_map.mod_pattern_to_trade_stat("#% increased Attack Speed", "implicit"),
            None
        );
    }
}
