use serde_json::Value;
use std::collections::HashMap;

pub struct AffixMap {
    mapping: HashMap<String, Vec<String>>,
}

impl AffixMap {
    pub fn new(json: Value) -> Result<Self, String> {
        let result = json
            .get("result")
            .and_then(|v| v.as_array())
            .ok_or("Missing result array in stats data")?;

        let mut pattern_map = HashMap::new();
        for section in result {
            if let Some(entries) = section.get("entries").and_then(|v| v.as_array()) {
                for entry in entries {
                    if let (Some(id), Some(text)) = (
                        entry.get("id").and_then(|v| v.as_str()),
                        entry.get("text").and_then(|v| v.as_str()),
                    ) {
                        let pattern = normalize_pattern(text);
                        pattern_map
                            .entry(pattern)
                            .or_insert_with(Vec::new)
                            .push(id.to_string());
                    }
                }
            }
        }

        Ok(Self {
            mapping: pattern_map,
        })
    }

    pub fn affix_to_trade_stat(&self, text: &str, prefix: &str) -> Option<(String, Vec<f32>)> {
        let (normalized_text, values) = normalize_mod_text(text);

        if let Some(ids) = self.mapping.get(&normalized_text) {
            for id in ids {
                if id.starts_with(prefix) {
                    return Some((id.clone(), values));
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

fn normalize_mod_text(text: &str) -> (String, Vec<f32>) {
    let mut values = Vec::new();
    let mut normalized = text.to_string();

    // replace numbers (2, +105, 12%, 5.85%) with #
    let number_pattern = regex::Regex::new(r"(\+?\-?\d+\.?\d*)").unwrap();
    while let Some(caps) = number_pattern.captures(&normalized) {
        let full_match = caps.get(0).unwrap();
        values.push(full_match.as_str().parse::<f32>().unwrap_or(0.0));
        normalized.replace_range(full_match.range(), "#");
    }

    // replace "an additional (singular-word)" with "# additional (plural-word)"
    let an_additional_pattern =
        regex::Regex::new(r" an additional (Projectile|Arrow|Curse)").unwrap();
    if let Some(caps) = an_additional_pattern.captures(&normalized) {
        values.push(1.0);
        // ggg handles the same mod for grenade projectiles differently lol
        if !normalized.starts_with("Grenade") {
            let full_match = caps.get(0).unwrap();
            normalized = normalized.replace(
                full_match.as_str(),
                &(full_match.as_str().to_string() + "s"),
            );
            normalized = normalized.replace(" an additional ", " # additional ");
        }
    }

    let ailment_seconds_pattern = regex::Regex::new(r"You cannot be (Chilled|Frozen|Shocked|Ignited) for # seconds").unwrap();
    if let Some(_caps) = ailment_seconds_pattern.captures(&normalized) {
        // remove the s from seconds
        normalized = normalized.replace("for # seconds", "for # second");
    }

    (normalized, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_affix_to_trade_stat() {
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
                    { "id": "explicit.stat_2306924373", "text": "You cannot be Chilled for # second after being Chilled" },
                    { "id": "rune.stat_1509134228", "text": "#% increased Physical Damage" },
                    { "id": "implicit.stat_1980802737", "text": "Grenade Skills Fire an additional Projectile" }
                ]
            }]
        });

        let affix_map = AffixMap::new(json).expect("Failed to create affix map");

        // Test cases
        let test_cases = vec![
            (
                "42% increased Physical Damage",
                "explicit",
                "explicit.stat_1509134228",
                vec![42.0],
            ),
            (
                "Adds 6 to 11 Physical Damage",
                "explicit",
                "explicit.stat_1940865751",
                vec![6.0, 11.0],
            ),
            (
                "+80 to Accuracy Rating",
                "explicit",
                "explicit.stat_803737631",
                vec![80.0],
            ),
            (
                "12% increased Attack Speed",
                "explicit",
                "explicit.stat_210067635",
                vec![12.0],
            ),
            (
                "Bow Attacks fire an additional Arrow",
                "explicit",
                "explicit.stat_3885405204",
                vec![1.0],
            ),
            (
                "Leeches 5.85% of Physical Damage as Mana",
                "explicit",
                "explicit.stat_669069897",
                vec![5.85],
            ),
            (
                "40% increased Physical Damage",
                "rune",
                "rune.stat_1509134228",
                vec![40.0],
            ),
            (
                "Grenade Skills Fire an additional Projectile",
                "implicit",
                "implicit.stat_1980802737",
                vec![1.0],
            ),
            (
                "You cannot be Chilled for 6 seconds after being Chilled",
                "explicit",
                "explicit.stat_2306924373",
                vec![6.0],
            ),
        ];

        for (text, prefix, expected_id, expected_values) in test_cases {
            if let Some((id, values)) = affix_map.affix_to_trade_stat(text, prefix) {
                assert_eq!(id, expected_id, "Failed for mod: {}", text);
                assert_eq!(values, expected_values, "Failed for mod: {}", text);
            } else {
                panic!("Failed to parse mod: {}", text);
            }
        }
    }
}
