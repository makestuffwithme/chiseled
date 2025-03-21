use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeFilters {
    // Base Item Properties
    pub item_category: Option<TextFilter>,
    pub item_name: Option<TextFilter>,
    pub item_base_type: Option<TextFilter>,
    pub rarity: Option<TextFilter>,
    pub item_level: Option<RangeFilter>,

    // Equipment Properties
    pub physical_dps: Option<RangeFilter>,
    pub elemental_dps: Option<RangeFilter>,
    pub total_dps: Option<RangeFilter>,
    pub attack_speed: Option<RangeFilter>,
    pub critical_chance: Option<RangeFilter>,
    pub socket_count: Option<RangeFilter>,
    pub armour: Option<RangeFilter>,
    pub energy_shield: Option<RangeFilter>,
    pub evasion: Option<RangeFilter>,
    pub spirit: Option<RangeFilter>,
    pub block_chance: Option<RangeFilter>,

    // Map Properties
    pub waystone_drop_chance: Option<RangeFilter>,

    // Stat Filters
    pub explicit_mods: Vec<StatFilter>,
    pub implicit_mods: Vec<StatFilter>,
    pub rune_mods: Vec<StatFilter>,

    pub price: PriceFilter,
    pub online_only: ToggleFilter,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TextFilter {
    pub text: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PriceFilter {
    pub enabled: bool,
    pub option: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RangeFilter {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatFilter {
    pub id: String,
    pub text: String,
    pub enabled: bool,
    pub value: StatValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StatValue {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFilter {
    pub enabled: bool,
}

impl StatValue {
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            StatValue {
                min: None,
                max: None,
            }
        } else {
            let avg = values.iter().sum::<f64>() / values.len() as f64;
            StatValue {
                min: Some(avg),
                max: None,
            }
        }
    }
}

impl TradeFilters {
    fn map_item_category(item_class: &str) -> String {
        let item_text = item_class.to_lowercase();
        match item_text.as_str() {
            // One-Handed Weapons
            "claws" => "weapon.claw",
            "daggers" => "weapon.dagger",
            "one hand swords" => "weapon.onesword",
            "one hand axes" => "weapon.oneaxe",
            "one hand maces" => "weapon.onemace",
            "spears" => "weapon.spear",
            "flails" => "weapon.flail",

            // Two-Handed Weapons
            "two hand swords" => "weapon.twosword",
            "two hand axes" => "weapon.twoaxe",
            "two hand maces" => "weapon.twomace",
            "quarterstaves" => "weapon.warstaff",

            // Ranged Weapons
            "bows" => "weapon.bow",
            "crossbows" => "weapon.crossbow",

            // Caster Weapons
            "wands" => "weapon.wand",
            "sceptres" => "weapon.sceptre",
            "staves" => "weapon.staff",

            // Armour
            "helmets" => "armour.helmet",
            "body armours" => "armour.chest",
            "gloves" => "armour.gloves",
            "boots" => "armour.boots",
            "quivers" => "armour.quiver",
            "shields" => "armour.shield",
            "focuses" => "armour.focus",
            "bucklers" => "armour.buckler",

            // Accessories
            "amulets" => "accessory.amulet",
            "belts" => "accessory.belt",
            "rings" => "accessory.ring",

            // Jewels
            "jewels" => "jewel",

            // Gems
            "active skill gems" => "gem.activegem",
            "support skill gems" => "gem.supportgem",
            "meta skill gems" => "gem.metagem",
            "skill gems" => "gem",

            // Flasks
            "life flasks" => "flask.life",
            "mana flasks" => "flask.mana",
            "flasks" => "flask",

            // Endgame Items
            "waystones" => "map.waystone",
            "map fragments" => "map.fragment",
            "logbooks" => "map.logbook",
            "breachstones" => "map.breachstone",
            "baryas" => "map.barya",
            "pinnacle keys" => "map.bosskey",
            "ultimatum keys" => "map.ultimatum",
            "tablets" => "map.tablet",

            // Misc Items
            "divination cards" => "card",
            "relics" => "sanctum.relic",
            "omens" => "currency.omen",
            "runes" => "currency.rune",
            "soul cores" => "currency.soulcore",

            // Default case
            _ => item_text.as_str(),
        }
        .to_string()
    }

    pub fn new() -> Self {
        Self {
            item_category: None,
            item_name: None,
            item_base_type: None,
            rarity: None,
            item_level: None,
            physical_dps: None,
            elemental_dps: None,
            total_dps: None,
            attack_speed: None,
            critical_chance: None,
            socket_count: None,
            armour: None,
            energy_shield: None,
            evasion: None,
            spirit: None,
            block_chance: None,
            waystone_drop_chance: None,
            explicit_mods: Vec::new(),
            implicit_mods: Vec::new(),
            rune_mods: Vec::new(),
            price: PriceFilter {
                enabled: true,
                option: String::new(),
                min: None,
                max: None,
            },
            online_only: ToggleFilter {
                enabled: true,
            },
        }
    }

    pub fn from_text(
        mod_pattern_to_trade_stat: impl Fn(&str, &str) -> Option<String>,
        item_text_to_base_type: impl Fn(&str) -> Option<(String, String)>,
        text: &str,
    ) -> Result<Self, String> {
        let mut filters = Self::new();
        let mut avg_phys_dmg: Option<f64> = None;
        let mut avg_ele_dmg: Option<f64> = None;
        let mut attack_speed: Option<f64> = None;

        // Split on first separator
        let mut parts = text.splitn(2, "--------");
        let header = parts.next().ok_or("Missing header section")?;
        let body = parts.next().ok_or("Missing body section")?;

        let mod_text_to_trade_stat_and_values = |text: &str, prefix: &str| -> Option<(String, Vec<f64>)> {
            let (mod_pattern, values) = Self::mod_text_to_pattern(text);
            mod_pattern_to_trade_stat(&mod_pattern, prefix).map(|trade_stat| (trade_stat, values))
        };

        // Parse header lines
        let header_lines: Vec<&str> = header
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        if header_lines.len() < 2 || header_lines.len() > 4 {
            return Err("Invalid item format: header should be between 2 and 4 lines".to_string());
        }

        let rarity_line = if header_lines.len() == 2 {
            header_lines[0]
        } else {
            header_lines[1]
        };

        // Parse rarity
        let rarity = rarity_line.strip_prefix("Rarity: ")
            .ok_or("Missing rarity")?;
        filters.rarity = Some(TextFilter {
            text: rarity.to_string().to_lowercase(),
            enabled: true,
        });

        // Parse item name and base type based on rarity
        match rarity {
            "Currency" => {
                filters.rarity = None;
                if header_lines.len() == 3 {
                    filters.item_base_type = Some(TextFilter {
                        text: header_lines[2].to_string(),
                        enabled: true,
                    });
                } else {
                    filters.item_base_type = Some(TextFilter {
                        text: header_lines[1].to_string(),
                        enabled: true,
                    });
                }
            }
            "Unique" => {
                // For unique items, we care about the name but not the base type or category
                filters.item_name = Some(TextFilter {
                    text: header_lines[2].to_string(),
                    enabled: true,
                });
            }
            "Rare" | "Magic" | "Normal" => {
                // Parse item category
                if let Some(class) = header_lines[0].strip_prefix("Item Class: ") {
                    filters.item_category = Some(TextFilter {
                        text: Self::map_item_category(class).to_string(),
                        enabled: true,
                    });
                }

                if header_lines.len() == 4 {
                    filters.item_base_type = Some(TextFilter {
                        text: header_lines[3].to_string(),
                        enabled: true,
                    });
                } else if let Some((base_type, _)) = item_text_to_base_type(text) {
                    filters.item_base_type = Some(TextFilter {
                        text: base_type,
                        enabled: true,
                    });
                }
            }
            _ => return Err(format!("Unsupported rarity: {}", rarity)),
        }

        // Process body lines
        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() || line == "--------" {
                continue;
            }

            if let Some(drop_chance) = line.strip_prefix("Waystone Drop Chance: ") {
                let drop_chance: f64 = drop_chance
                    .trim_start_matches('+')
                    .trim_end_matches("% (augmented)")
                    .trim_end_matches('%')
                    .trim()
                    .parse()
                    .map_err(|e| format!("Failed to parse waystone drop chance value: {}", e))?;
                filters.waystone_drop_chance = Some(RangeFilter {
                    min: Some(drop_chance),
                    max: None,
                    enabled: true,
                });
            } else if let Some(block) = line.strip_prefix("Block chance: ") {
                let block: f64 = block
                    .trim_end_matches("% (augmented)")
                    .trim_end_matches('%')
                    .trim()
                    .parse()
                    .map_err(|e| format!("Failed to parse block chance value: {}", e))?;
                filters.block_chance = Some(RangeFilter {
                    min: Some(block),
                    max: None,
                    enabled: true,
                });
            } else if let Some(spirit) = line.strip_prefix("Spirit: ") {
                let spirit: f64 = spirit
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| format!("Failed to parse spirit value: {}", e))?;
                filters.spirit = Some(RangeFilter {
                    min: Some(spirit),
                    max: None,
                    enabled: true,
                });
            } else if let Some(sockets) = line.strip_prefix("Sockets: ") {
                let socket_count = sockets.split_whitespace().count();
                filters.socket_count = Some(RangeFilter {
                    min: Some(socket_count as f64),
                    max: None,
                    enabled: true,
                });
            } else if let Some(level) = line.strip_prefix("Item Level: ") {
                let level: f64 = level
                    .parse()
                    .map_err(|e| format!("Failed to parse item level: {}", e))?;
                filters.item_level = Some(RangeFilter {
                    min: Some(level),
                    max: None,
                    enabled: true,
                });
            } else if let Some(armour) = line.strip_prefix("Armour: ") {
                let armour: f64 = armour
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| format!("Failed to parse armour value: {}", e))?;
                filters.armour = Some(RangeFilter {
                    min: Some(armour),
                    max: None,
                    enabled: true,
                });
            } else if let Some(es) = line.strip_prefix("Energy Shield: ") {
                let es: f64 = es
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| format!("Failed to parse energy shield value: {}", e))?;
                filters.energy_shield = Some(RangeFilter {
                    min: Some(es),
                    max: None,
                    enabled: true,
                });
            } else if let Some(evasion) = line.strip_prefix("Evasion Rating: ") {
                let evasion: f64 = evasion
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| format!("Failed to parse evasion rating value: {}", e))?;
                filters.evasion = Some(RangeFilter {
                    min: Some(evasion),
                    max: None,
                    enabled: true,
                });
            } else if let Some(crit) = line.strip_prefix("Critical Hit Chance: ") {
                let crit = crit
                    .trim_end_matches("% (augmented)")
                    .trim_end_matches('%')
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| format!("Failed to parse critical hit chance: {}", e))?;
                filters.critical_chance = Some(RangeFilter {
                    min: Some(crit),
                    max: None,
                    enabled: true,
                });
            } else if let Some(aps) = line.strip_prefix("Attacks per Second: ") {
                attack_speed = Some(
                    aps.split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .map_err(|e| format!("Failed to parse attack speed: {}", e))?,
                );
                filters.attack_speed = Some(RangeFilter {
                    min: attack_speed,
                    max: None,
                    enabled: true,
                });
            } else if let Some(phys_dmg) = line.strip_prefix("Physical Damage: ") {
                let parts: Vec<&str> = phys_dmg.split('-').collect();
                if parts.len() == 2 {
                    let min: f64 = parts[0]
                        .trim()
                        .parse()
                        .map_err(|e| format!("Failed to parse physical damage min: {}", e))?;
                    let max: f64 = parts[1]
                        .split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .map_err(|e| format!("Failed to parse physical damage max: {}", e))?;
                    avg_phys_dmg = Some((min + max) / 2.0);
                }
            } else if let Some(ele_dmg) = line.strip_prefix("Elemental Damage: ") {
                let parts: Vec<&str> = ele_dmg.split('-').collect();
                if parts.len() == 2 {
                    let min: f64 = parts[0]
                        .trim()
                        .parse()
                        .map_err(|e| format!("Failed to parse elemental damage min: {}", e))?;
                    let max: f64 = parts[1]
                        .split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .map_err(|e| format!("Failed to parse elemental damage max: {}", e))?;
                    avg_ele_dmg = Some((min + max) / 2.0);
                }
            } else if line.ends_with("(implicit)") {
                let mod_text = line.trim_end_matches("(implicit)").trim();
                if let Some((id, values)) = mod_text_to_trade_stat_and_values(mod_text, "implicit") {
                    filters.implicit_mods.push(StatFilter {
                        id,
                        text: mod_text.to_string(),
                        enabled: true,
                        value: StatValue::from_values(&values),
                    });
                }
            } else if line.ends_with("(rune)") {
                let mod_text = line.trim_end_matches("(rune)").trim();
                if let Some((id, values)) = mod_text_to_trade_stat_and_values(mod_text, "rune") {
                    filters.rune_mods.push(StatFilter {
                        id,
                        text: mod_text.to_string(),
                        enabled: true,
                        value: StatValue::from_values(&values),
                    });
                }
            } else if !line.starts_with("Requirements:")
                && !line.starts_with("Level:")
                && !line.starts_with("Str:")
                && !line.starts_with("Dex:")
                && !line.starts_with("Reload Time:")
                && !line.starts_with("Sockets:")
                && !line.starts_with("Quality:")
            {
                // Try to parse as explicit mod
                if let Some((id, values)) = mod_text_to_trade_stat_and_values(line, "explicit") {
                    filters.explicit_mods.push(StatFilter {
                        id,
                        text: line.to_string(),
                        enabled: true,
                        value: StatValue::from_values(&values),
                    });
                }
            }
        }

        // Calculate DPS values
        if let Some(aps) = attack_speed {
            if let Some(avg_phys) = avg_phys_dmg {
                let pdps = avg_phys * aps;
                filters.physical_dps = Some(RangeFilter {
                    min: Some(pdps),
                    max: None,
                    enabled: true,
                });
            }

            if let Some(avg_ele) = avg_ele_dmg {
                let edps = avg_ele * aps;
                filters.elemental_dps = Some(RangeFilter {
                    min: Some(edps),
                    max: None,
                    enabled: true,
                });
            }
        }

        // Calculate total DPS if we have either physical or elemental DPS
        if filters.physical_dps.is_some() || filters.elemental_dps.is_some() {
            let pdps = filters
                .physical_dps
                .as_ref()
                .and_then(|f| f.min)
                .unwrap_or(0.0);
            let edps = filters
                .elemental_dps
                .as_ref()
                .and_then(|f| f.min)
                .unwrap_or(0.0);
            filters.total_dps = Some(RangeFilter {
                min: Some(pdps + edps),
                max: None,
                enabled: true,
            });
        }

        Ok(filters)
    }

    fn mod_text_to_pattern(text: &str) -> (String, Vec<f64>) {
        let mut values = Vec::new();
        let mut normalized = text.to_string();
    
        // replace numbers (2, +105, 12%, 5.85%) with #
        let number_pattern = regex::Regex::new(r"(\+?\-?\d+\.?\d*)").unwrap();
        while let Some(caps) = number_pattern.captures(&normalized) {
            let full_match = caps.get(0).unwrap();
            values.push(full_match.as_str().parse::<f64>().unwrap_or(0.0));
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mapping::mod_pattern_map::ModPatternMap;
    use crate::mapping::base_type_map::BaseTypeMap;
    use serde_json::json;

    fn assert_json_float_eq(value: &serde_json::Value, expected: f64) {
        let actual = value.as_f64().unwrap();
        let epsilon = 0.000001;
        assert!(
            (actual - expected).abs() < epsilon,
            "Expected {} but got {}",
            expected,
            actual
        );
    }

    fn get_test_maps() -> (ModPatternMap, BaseTypeMap) {
        let stats_json = json!({
            "result": [{
                "entries": [
                    { "id": "explicit.stat_803737631", "text": "# to Accuracy Rating" },
                    { "id": "explicit.stat_210067635", "text": "#% increased Attack Speed" },
                    { "id": "explicit.stat_1509134228", "text": "#% increased Physical Damage" },
                    { "id": "explicit.stat_1940865751", "text": "Adds # to # Physical Damage" },
                    { "id": "explicit.stat_3885405204", "text": "Bow Attacks fire # additional Arrows" },
                    { "id": "explicit.stat_669069897", "text": "Leeches #% of Physical Damage as Mana" },
                    { "id": "rune.stat_1509134228", "text": "#% increased Physical Damage" },
                    { "id": "implicit.stat_1980802737", "text": "Grenade Skills Fire an additional Projectile" },
                    { "id": "implicit.stat_3885405204", "text": "Bow Attacks fire # additional Arrows" }
                ]
            }]
        });

        let items_json = json!({
            "result": [{
                "id": "weapon.crossbow",
                "label": "Crossbows",
                "entries": [
                    {
                        "type": "Bombard Crossbow",
                        "text": "Bombard Crossbow",
                        "flags": { "unique": false }
                    }
                ]
            },
            {
                "id": "weapon.bow",
                "label": "Bows",
                "entries": [
                    {
                        "type": "Advanced Dualstring Bow",
                        "text": "Advanced Dualstring Bow",
                        "flags": { "unique": false }
                    }
                ]
            }]
        });

        let mod_pattern_map = ModPatternMap::new(stats_json).expect("Failed to create mod pattern map");
        let base_type_map = BaseTypeMap::new(items_json).expect("Failed to create base type map");

        (mod_pattern_map, base_type_map)
    }

    const TEST_ITEM: &str = r#"Item Class: Crossbows
Rarity: Magic
Deliberate Bombard Crossbow of Mastery
--------
Physical Damage: 12-47
Critical Hit Chance: 5.00%
Attacks per Second: 1.85 (augmented)
Reload Time: 0.75 (augmented)
--------
Requirements:
Level: 33
Str: 43 (unmet)
Dex: 43
--------
Item Level: 35
--------
Grenade Skills Fire an additional Projectile (implicit)
--------
+105 to Accuracy Rating
12% increased Attack Speed"#;

    const TEST_ITEM_WITH_AILMENT: &str = r#"Item Class: Boots
Rarity: Magic
Deliberate Boots of the Glacier
--------
Requirements:
Level: 33
Str: 43 (unmet)
Dex: 43
--------
Item Level: 35
--------
You cannot be Chilled for 6 seconds after being Chilled"#;

    const BOW_TEST_ITEM: &str = r#"Item Class: Bows
Rarity: Rare
Horror Thunder
Advanced Dualstring Bow
--------
Physical Damage: 64-118 (augmented)
Critical Hit Chance: 5.00%
Attacks per Second: 1.36 (augmented)
--------
Requirements:
Level: 55
Dex: 126
--------
Sockets: S S 
--------
Item Level: 58
--------
40% increased Physical Damage (rune)
--------
Bow Attacks fire an additional Arrow (implicit)
--------
42% increased Physical Damage
Adds 6 to 11 Physical Damage
+80 to Accuracy Rating
13% increased Attack Speed
Bow Attacks fire an additional Arrow
Leeches 5.85% of Physical Damage as Mana"#;

    const ARMOUR_TEST_ITEM: &str = r#"Item Class: Helmets
Rarity: Rare
Empyrean Dome
Cultist Crown
--------
Armour: 44
Energy Shield: 19
Evasion Rating: 919 (augmented)
--------
Requirements:
Level: 16
Str: 18 (unmet)
Int: 18
--------
Item Level: 17
--------
9% increased Rarity of Items found
+1 to Level of all Minion Skills
+5 to Intelligence
+8% to Cold Resistance"#;

    #[test]
    fn test_trade_filters_from_text() {
        let (affix_map, base_type_map) = get_test_maps();
        let filters = TradeFilters::from_text(
            |text, prefix| affix_map.mod_pattern_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            TEST_ITEM,
        )
        .expect("Should parse successfully");

        // Check base properties
        assert_eq!(
            filters.item_category,
            Some(TextFilter {
                text: "weapon.crossbow".to_string(),
                enabled: true,
            })
        );

        assert_eq!(
            filters.rarity,
            Some(TextFilter {
                text: "Magic".to_string(),
                enabled: true,
            })
        );
        assert_eq!(
            filters.item_level,
            Some(RangeFilter {
                min: Some(35.0),
                max: None,
                enabled: true,
            })
        );

        // Check weapon properties
        assert_eq!(
            filters.attack_speed,
            Some(RangeFilter {
                min: Some(1.85),
                max: None,
                enabled: true,
            })
        );

        // check crit chance
        assert_eq!(
            filters.critical_chance,
            Some(RangeFilter {
                min: Some(5.0),
                max: None,
                enabled: true,
            })
        );

        // Check implicit mods
        assert_eq!(filters.implicit_mods.len(), 1);
        let implicit_mod = &filters.implicit_mods[0];
        assert_eq!(implicit_mod.id, "implicit.stat_1980802737");
        assert_eq!(
            implicit_mod.text,
            "Grenade Skills Fire an additional Projectile"
        );

        // Check explicit mods
        assert_eq!(filters.explicit_mods.len(), 2);

        // Accuracy mod
        let accuracy_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.id == "explicit.stat_803737631")
            .expect("Should have accuracy mod");
        assert_eq!(accuracy_mod.text, "+105 to Accuracy Rating");
        assert_json_float_eq(&json!(accuracy_mod.value.min.unwrap()), 105.0);

        // Attack speed mod
        let speed_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.id == "explicit.stat_210067635")
            .expect("Should have attack speed mod");
        assert_eq!(speed_mod.text, "12% increased Attack Speed");
        assert_json_float_eq(&json!(speed_mod.value.min.unwrap()), 12.0);
    }

    #[test]
    fn test_trade_filters_from_text_with_ailment() {
        let (affix_map, base_type_map) = get_test_maps();
        let filters = TradeFilters::from_text(
            |text, prefix| affix_map.mod_pattern_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            TEST_ITEM_WITH_AILMENT,
        )
        .expect("Should parse successfully");

        // Check explicit mods
        assert_eq!(filters.explicit_mods.len(), 1);

        // Ailment mod
        let ailment_mod = &filters.explicit_mods[0];
        assert_eq!(
            ailment_mod.text,
            "You cannot be Chilled for 6 seconds after being Chilled"
        );
        assert_json_float_eq(&json!(ailment_mod.value.min.unwrap()), 6.0);
    }

    #[test]
    fn test_normalize_mod_text() {
        let test_cases = vec![
            (
                "42% increased Physical Damage",
                ("#% increased Physical Damage", vec![42.0]),
            ),
            (
                "Adds 6 to 11 Physical Damage",
                ("Adds # to # Physical Damage", vec![6.0, 11.0]),
            ),
            (
                "+80 to Accuracy Rating",
                ("# to Accuracy Rating", vec![80.0]),
            ),
            (
                "12% increased Attack Speed",
                ("#% increased Attack Speed", vec![12.0]),
            ),
            (
                "Bow Attacks fire an additional Arrow",
                ("Bow Attacks fire # additional Arrows", vec![1.0]),
            ),
            (
                "Leeches 5.85% of Physical Damage as Mana",
                ("Leeches #% of Physical Damage as Mana", vec![5.85]),
            ),
            (
                "You cannot be Chilled for 6 seconds after being Chilled",
                ("You cannot be Chilled for # second after being Chilled", vec![6.0]),
            ),
        ];

        for (input, expected) in test_cases {
            let (normalized, values) = TradeFilters::mod_text_to_pattern(input);
            assert_eq!(normalized, expected.0, "Failed for mod: {}", input);
            assert_eq!(values, expected.1, "Failed for mod: {}", input);
        }
    }

    #[test]
    fn test_bow_with_rune_mod() {
        let (affix_map, base_type_map) = get_test_maps();
        let filters = TradeFilters::from_text(
            |text, prefix| affix_map.mod_pattern_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            BOW_TEST_ITEM,
        )
        .expect("Should parse successfully");

        // Check base properties
        assert_eq!(
            filters.item_category,
            Some(TextFilter {
                text: "weapon.bow".to_string(),
                enabled: true,
            })
        );

        // Check socket count
        assert_eq!(
            filters.socket_count,
            Some(RangeFilter {
                min: Some(2.0),
                max: None,
                enabled: true,
            })
        );

        // Check rarity
        assert_eq!(
            filters.rarity,
            Some(TextFilter {
                text: "Rare".to_string(),
                enabled: true,
            })
        );
        assert_eq!(
            filters.item_level,
            Some(RangeFilter {
                min: Some(58.0),
                max: None,
                enabled: true,
            })
        );

        // Check weapon properties
        assert_eq!(
            filters.attack_speed,
            Some(RangeFilter {
                min: Some(1.36),
                max: None,
                enabled: true,
            })
        );

        // Check rune mod
        assert_eq!(filters.rune_mods.len(), 1);
        let rune_mod = &filters.rune_mods[0];
        assert_eq!(rune_mod.text, "40% increased Physical Damage");
        assert_eq!(rune_mod.id, "rune.stat_1509134228");
        assert_json_float_eq(&json!(rune_mod.value.min.unwrap()), 40.0);

        // Check implicit mod
        assert_eq!(filters.implicit_mods.len(), 1);
        let implicit_mod = &filters.implicit_mods[0];
        assert_eq!(implicit_mod.text, "Bow Attacks fire an additional Arrow");
        assert_eq!(implicit_mod.id, "implicit.stat_3885405204");
        assert_json_float_eq(&json!(implicit_mod.value.min.unwrap()), 1.0);

        // Check explicit mods
        assert_eq!(filters.explicit_mods.len(), 6);

        // Physical Damage mod
        let phys_dmg_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "42% increased Physical Damage")
            .expect("Should have physical damage mod");
        assert_eq!(phys_dmg_mod.id, "explicit.stat_1509134228");
        assert_json_float_eq(&json!(phys_dmg_mod.value.min.unwrap()), 42.0);

        // Flat Physical Damage mod
        let flat_phys_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "Adds 6 to 11 Physical Damage")
            .expect("Should have flat physical damage mod");
        assert_eq!(flat_phys_mod.id, "explicit.stat_1940865751");

        // Accuracy mod
        let accuracy_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "+80 to Accuracy Rating")
            .expect("Should have accuracy mod");
        assert_eq!(accuracy_mod.id, "explicit.stat_803737631");
        assert_json_float_eq(&json!(accuracy_mod.value.min.unwrap()), 80.0);

        // Attack Speed mod
        let speed_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "13% increased Attack Speed")
            .expect("Should have attack speed mod");
        assert_eq!(speed_mod.id, "explicit.stat_210067635");
        assert_json_float_eq(&json!(speed_mod.value.min.unwrap()), 13.0);

        // Additional Arrow mod
        let arrow_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "Bow Attacks fire an additional Arrow")
            .expect("Should have additional arrow mod");
        assert_eq!(arrow_mod.id, "explicit.stat_3885405204");

        // Mana Leech mod
        let leech_mod = filters
            .explicit_mods
            .iter()
            .find(|m| m.text == "Leeches 5.85% of Physical Damage as Mana")
            .expect("Should have mana leech mod");
        assert_eq!(leech_mod.id, "explicit.stat_669069897");
        assert_json_float_eq(&json!(leech_mod.value.min.unwrap()), 5.85);
    }

    #[test]
    fn test_armour_with_defensive_stats() {
        let (affix_map, base_type_map) = get_test_maps();
        let filters = TradeFilters::from_text(
            |text, prefix| affix_map.mod_pattern_to_trade_stat(text, prefix),
            |text| base_type_map.item_text_to_base_type(text),
            ARMOUR_TEST_ITEM,
        )
        .expect("Should parse successfully");

        // Check base properties
        assert_eq!(
            filters.item_category,
            Some(TextFilter {
                text: "armour.helmet".to_string(),
                enabled: true,
            })
        );

        // Check defensive stats
        assert_eq!(
            filters.armour,
            Some(RangeFilter {
                min: Some(44.0),
                max: None,
                enabled: true,
            })
        );

        assert_eq!(
            filters.energy_shield,
            Some(RangeFilter {
                min: Some(19.0),
                max: None,
                enabled: true,
            })
        );

        assert_eq!(
            filters.evasion,
            Some(RangeFilter {
                min: Some(919.0),
                max: None,
                enabled: true,
            })
        );

        // Check item level
        assert_eq!(
            filters.item_level,
            Some(RangeFilter {
                min: Some(17.0),
                max: None,
                enabled: true,
            })
        );

        // Check explicit mods (just verify count as the actual mod parsing is tested elsewhere)
        assert_eq!(filters.explicit_mods.len(), 4);
    }
}
