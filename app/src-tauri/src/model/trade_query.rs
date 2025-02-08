use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::trade_filter::TradeFilters;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeQuery {
    pub query: Value,
    pub sort: Value,
}

impl TradeQuery {
    pub fn new(query: Value) -> Self {
        Self {
            query,
            sort: json!({ "price": "asc" }),
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
                        "disabled": false,
                        "value": {
                            "option": true
                        }
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
                "option": if filters.online_only { "online" } else { "any" }
            },
            "stats": [{
                "type": "and",
                "filters": stat_filters,
                "disabled": false
            }],
            "filters": {
                "type_filters": {
                    "filters": {
                        "category": {
                            "option": filters.item_category.as_ref()
                                .map(|c| c.text.to_lowercase())
                                .unwrap_or_default()
                        }
                    },
                    "disabled": !filters.item_category.as_ref().map_or(false, |c| c.enabled)
                }
            }
        });

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

        if has_equipment_filters {
            query["filters"]["equipment_filters"] = json!({
                "filters": equipment_filters,
                "disabled": false
            });
        }

        if filters.price.enabled && !filters.price.option.is_empty() {
            query["filters"]["trade_filters"] = json!({
                "filters": {
                    "price": {
                        "option": filters.price.option,
                        "min": filters.price.min,
                        "max": filters.price.max
                    }
                },
                "disabled": false
            });
        }

        Self::new(query)
    }
}

#[cfg(test)]
mod test {
    use crate::model::trade_filter::{
        RangeFilter, StatFilter, StatValue, TextFilter, TradeFilters,
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
        filters.online_only = true;

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
}
