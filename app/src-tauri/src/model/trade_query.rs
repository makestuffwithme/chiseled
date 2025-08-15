use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::trade_filter::TradeFilters;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeQuery {
    pub query: Value,
    pub sort: Value,
    pub league: String,
}

impl TradeQuery {
    pub fn new(query: Value, league: String) -> Self {
        Self {
            query,
            sort: json!({ "price": "asc" }),
            league,
        }
    }

    pub fn from_trade_filters(filters: &TradeFilters) -> Self {
        let mut stat_filters = Vec::new();

        // Add explicit mods
        for stat in &filters.explicit_mods {
            if stat.enabled {
                if stat.value.min.is_some() || stat.value.max.is_some() {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false,
                        "value": {
                            "min": stat.value.min,
                            "max": stat.value.max
                        }
                    }));
                } else {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false
                    }));
                }
            }
        }

        // Add implicit mods
        for stat in &filters.implicit_mods {
            if stat.enabled {
                if stat.value.min.is_some() || stat.value.max.is_some() {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false,
                        "value": {
                            "min": stat.value.min,
                            "max": stat.value.max
                        }
                    }));
                } else {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false,
                        "value": {
                            "option": true
                        }
                    }));
                }
            }
        }

        // Add rune mods
        for stat in &filters.rune_mods {
            if stat.enabled {
                if stat.value.min.is_some() || stat.value.max.is_some() {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false,
                        "value": {
                            "min": stat.value.min,
                            "max": stat.value.max
                        }
                    }));
                } else {
                    stat_filters.push(json!({
                        "id": stat.id,
                        "disabled": false,
                        "value": {
                            "option": true
                        }
                    }));
                }
            }
        }

        // Build the main query
        let mut query = json!({
            "status": {
                "option": if filters.online_only.enabled { "online" } else { "any" }
            },
            "stats": [{
                "type": "and",
                "filters": stat_filters,
                "disabled": false
            }]
        });

        // Only add type filters if we have an enabled category
        if filters.item_category.as_ref().map_or(false, |c| c.enabled) {
            query["filters"]["type_filters"] = json!({
                "filters": {
                    "category": {
                        "option": filters.item_category.as_ref()
                            .map(|c| c.text.to_lowercase())
                            .unwrap_or_default()
                    }
                },
                "disabled": false
            });
        }

        // Add name filter if enabled (for unique items)
        if let Some(name) = &filters.item_name {
            if name.enabled {
                query["name"] = json!(name.text);
            }
        }
        // Add type filter if enabled (for non-unique items)
        else if let Some(base_type) = &filters.item_base_type {
            if base_type.enabled {
                query["type"] = json!(base_type.text);
            }
        }

        // Add item level filter if present and enabled
        if let Some(ilvl) = &filters.item_level {
            if ilvl.enabled {
                query["filters"]["misc_filters"] = json!({
                    "filters": {
                        "ilvl": {
                            "min": ilvl.min,
                            "max": ilvl.max
                        }
                    },
                    "disabled": false
                });
            }
        }

        // Add equipment filters if any are present and enabled
        let mut equipment_filters = json!({});
        let mut has_equipment_filters = false;

        if let Some(rarity) = &filters.rarity {
            if rarity.enabled && rarity.text != "" {
                equipment_filters["rarity"] = json!(rarity.text);
                has_equipment_filters = true;
            }
        }

        if let Some(pdps) = &filters.physical_dps {
            if pdps.enabled {
                equipment_filters["pdps"] = json!({"min": pdps.min, "max": pdps.max});
                has_equipment_filters = true;
            }
        }
        if let Some(edps) = &filters.elemental_dps {
            if edps.enabled {
                equipment_filters["edps"] = json!({"min": edps.min, "max": edps.max});
                has_equipment_filters = true;
            }
        }
        if let Some(dps) = &filters.total_dps {
            if dps.enabled {
                equipment_filters["dps"] = json!({"min": dps.min, "max": dps.max});
                has_equipment_filters = true;
            }
        }
        if let Some(aps) = &filters.attack_speed {
            if aps.enabled {
                equipment_filters["aps"] = json!({"min": aps.min, "max": aps.max});
                has_equipment_filters = true;
            }
        }
        if let Some(sockets) = &filters.socket_count {
            if sockets.enabled {
                equipment_filters["rune_sockets"] = json!({"min": sockets.min, "max": sockets.max});
                has_equipment_filters = true;
            }
        }

        // Add armour filters
        if let Some(armour) = &filters.armour {
            if armour.enabled {
                equipment_filters["ar"] = json!({"min": armour.min, "max": armour.max});
                has_equipment_filters = true;
            }
        }
        if let Some(es) = &filters.energy_shield {
            if es.enabled {
                equipment_filters["es"] = json!({"min": es.min, "max": es.max});
                has_equipment_filters = true;
            }
        }
        if let Some(evasion) = &filters.evasion {
            if evasion.enabled {
                equipment_filters["ev"] = json!({"min": evasion.min, "max": evasion.max});
                has_equipment_filters = true;
            }
        }
        if let Some(spirit) = &filters.spirit {
            if spirit.enabled {
                equipment_filters["spirit"] = json!({"min": spirit.min, "max": spirit.max});
                has_equipment_filters = true;
            }
        }
        if let Some(block) = &filters.block_chance {
            if block.enabled {
                equipment_filters["block"] = json!({"min": block.min, "max": block.max});
                has_equipment_filters = true;
            }
        }

        if has_equipment_filters {
            query["filters"]["equipment_filters"] = json!({
                "filters": equipment_filters,
                "disabled": false
            });
        }

        if let Some(drop_chance) = &filters.waystone_drop_chance {
            if drop_chance.enabled {
                query["filters"]["map_filters"] = json!({
                    "filters": {
                        "map_bonus": {
                            "min": drop_chance.min,
                            "max": drop_chance.max
                        }
                    },
                    "disabled": false
                });
            }
        }

        let mut trade_filters = json!({});
        trade_filters["collapse"] = json!({ "option": true });

        if filters.price.enabled && !filters.price.option.is_empty() {
            trade_filters["price"] = json!({
                "option": filters.price.option,
                "min": filters.price.min,
                "max": filters.price.max
            });
        }

        query["filters"]["trade_filters"] = json!({
            "filters": trade_filters,
            "disabled": false
        });

        let league = filters.league.as_ref()
            .map(|l| l.text.clone())
            .unwrap_or_else(|| "Standard".to_string());
        Self::new(query, league)
    }
}

#[cfg(test)]
mod test {
    use crate::model::trade_filter::{
        RangeFilter, StatFilter, StatValue, TextFilter, TradeFilters, ToggleFilter,
    };
    use crate::model::trade_query::TradeQuery;

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

    #[test]
    fn test_trade_filters_to_query() {
        let mut filters = TradeFilters::new();
        filters.item_category = Some(TextFilter {
            text: "weapon.crossbow".to_string(),
            enabled: true,
        });
        filters.online_only = ToggleFilter {
            enabled: true,
        };

        // Add an explicit mod with range values
        filters.explicit_mods.push(StatFilter {
            id: "explicit.stat_803737631".to_string(),
            text: "+105 to Accuracy Rating".to_string(),
            value: StatValue {
                min: Some(94.5),
                max: Some(115.5),
            },
            enabled: true,
        });

        // Add an implicit mod with boolean value
        filters.implicit_mods.push(StatFilter {
            id: "implicit.stat_1980802737".to_string(),
            text: "Grenade Skills Fire an additional Projectile".to_string(),
            value: StatValue {
                min: Some(1.0),
                max: Some(1.0),
            },
            enabled: true,
        });

        // Add some weapon properties
        filters.attack_speed = Some(RangeFilter {
            min: Some(1.85),
            max: Some(1.85),
            enabled: true,
        });

        let query = TradeQuery::from_trade_filters(&filters);

        // Verify the query structure
        assert_eq!(query.query["status"]["option"], "online");

        // Check stats section
        let stats = &query.query["stats"][0];
        assert_eq!(stats["type"], "and");
        assert_eq!(stats["disabled"], false);

        // Check filters
        let stat_filters = stats["filters"].as_array().unwrap();
        assert_eq!(stat_filters.len(), 2); // One explicit, one implicit

        // Check explicit mod filter
        let accuracy_filter = stat_filters
            .iter()
            .find(|f| f["id"] == "explicit.stat_803737631")
            .expect("Should have accuracy filter");
        assert_eq!(accuracy_filter["disabled"], false);
        assert_json_float_eq(&accuracy_filter["value"]["min"], 94.5);
        assert_json_float_eq(&accuracy_filter["value"]["max"], 115.5);

        // Check implicit mod filter
        let grenade_filter = stat_filters
            .iter()
            .find(|f| f["id"] == "implicit.stat_1980802737")
            .expect("Should have grenade filter");
        assert_eq!(grenade_filter["disabled"], false);

        // Check type filters
        let type_filters = &query.query["filters"]["type_filters"];
        assert_eq!(type_filters["disabled"], false);
        assert_eq!(
            type_filters["filters"]["category"]["option"],
            "weapon.crossbow"
        );

        // Check weapon filters
        let equipment_filters = &query.query["filters"]["equipment_filters"];
        assert_eq!(equipment_filters["disabled"], false);
        assert_json_float_eq(&equipment_filters["filters"]["aps"]["min"], 1.85);
        assert_json_float_eq(&equipment_filters["filters"]["aps"]["max"], 1.85);

        // Check sort
        assert_eq!(query.sort["price"], "asc");
    }

    #[test]
    fn test_armour_filters_to_query() {
        let mut filters = TradeFilters::new();
        filters.item_category = Some(TextFilter {
            text: "armour.helmet".to_string(),
            enabled: true,
        });
        filters.online_only = ToggleFilter {
            enabled: true,
        };

        // Add armour properties
        filters.armour = Some(RangeFilter {
            min: Some(44.0),
            max: None,
            enabled: true,
        });
        filters.energy_shield = Some(RangeFilter {
            min: Some(19.0),
            max: None,
            enabled: true,
        });
        filters.evasion = Some(RangeFilter {
            min: Some(919.0),
            max: None,
            enabled: true,
        });

        let query = TradeQuery::from_trade_filters(&filters);

        // Verify the query structure
        assert_eq!(query.query["status"]["option"], "online");

        // Check type filters
        let type_filters = &query.query["filters"]["type_filters"];
        assert_eq!(type_filters["disabled"], false);
        assert_eq!(
            type_filters["filters"]["category"]["option"],
            "armour.helmet"
        );

        // Check equipment filters
        let equipment_filters = &query.query["filters"]["equipment_filters"];
        assert_eq!(equipment_filters["disabled"], false);

        // Check armour values
        assert_json_float_eq(&equipment_filters["filters"]["ar"]["min"], 44.0);
        assert!(equipment_filters["filters"]["ar"]["max"].is_null());

        // Check energy shield values
        assert_json_float_eq(&equipment_filters["filters"]["es"]["min"], 19.0);
        assert!(equipment_filters["filters"]["es"]["max"].is_null());

        // Check evasion values
        assert_json_float_eq(&equipment_filters["filters"]["ev"]["min"], 919.0);
        assert!(equipment_filters["filters"]["ev"]["max"].is_null());
    }
}
