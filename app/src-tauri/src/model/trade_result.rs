use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeResult {
    pub result: Vec<ItemListing>,
    #[serde(default)]
    pub total: usize,
    #[serde(default)]
    pub current_page: usize,
    #[serde(default)]
    pub total_pages: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemListing {
    pub id: String,
    pub listing: ListingInfo,
    pub item: ItemInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemInfo {
    pub realm: String,
    pub verified: bool,
    pub w: i32,
    pub h: i32,
    pub icon: String,
    pub league: String,
    pub id: String,
    pub sockets: Option<Vec<SocketInfo>>,
    pub name: String,
    #[serde(rename = "typeLine")]
    pub type_line: String,
    #[serde(rename = "baseType")]
    pub base_type: String,
    pub rarity: Option<String>,
    pub ilvl: Option<i32>,
    pub identified: Option<bool>,
    pub corrupted: Option<bool>,
    pub note: Option<String>,
    pub properties: Option<Vec<ItemProperty>>,
    pub requirements: Option<Vec<ItemRequirement>>,
    #[serde(rename = "implicitMods")]
    pub implicit_mods: Option<Vec<String>>,
    #[serde(rename = "explicitMods")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(rename = "runeMods")]
    pub rune_mods: Option<Vec<String>>,
    #[serde(rename = "frameType")]
    pub frame_type: i32,
    #[serde(rename = "socketedItems")]
    pub socketed_items: Option<Vec<SocketedItem>>,
    #[serde(default)]
    pub extended: ExtendedInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtendedInfo {
    Full(ItemExtendedInfo),
    Empty(Vec<String>),
}

impl Default for ExtendedInfo {
    fn default() -> Self {
        ExtendedInfo::Empty(vec![])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SocketInfo {
    pub group: i32,
    #[serde(rename = "type")]
    pub socket_type: String,
    pub item: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SocketedItem {
    pub realm: String,
    pub verified: bool,
    pub w: i32,
    pub h: i32,
    pub icon: String,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
    pub league: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "typeLine")]
    pub type_line: String,
    #[serde(rename = "baseType")]
    pub base_type: String,
    pub ilvl: i32,
    pub identified: bool,
    pub properties: Option<Vec<ItemProperty>>,
    #[serde(rename = "explicitMods")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(rename = "descrText")]
    pub description_text: Option<String>,
    #[serde(rename = "frameType")]
    pub frame_type: i32,
    pub socket: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<(String, i32)>,
    #[serde(rename = "displayMode")]
    pub display_mode: i32,
    #[serde(rename = "type")]
    pub property_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemRequirement {
    pub name: String,
    pub values: Vec<(String, i32)>,
    #[serde(rename = "displayMode")]
    pub display_mode: i32,
    #[serde(rename = "type")]
    pub requirement_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemExtendedInfo {
    pub dps: Option<f64>,
    pub pdps: Option<f64>,
    pub edps: Option<f64>,
    #[serde(rename = "dps_aug")]
    pub dps_augmented: Option<bool>,
    #[serde(rename = "pdps_aug")]
    pub pdps_augmented: Option<bool>,
    pub mods: Option<ModInfo>,
    pub hashes: Option<HashInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModInfo {
    pub explicit: Option<Vec<ModDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModDetail {
    pub name: String,
    pub tier: String,
    pub level: i32,
    pub magnitudes: Vec<Magnitude>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Magnitude {
    pub hash: String,
    pub min: String,
    pub max: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HashInfo {
    pub explicit: Option<Vec<(String, Vec<i32>)>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListingInfo {
    pub method: String,
    pub indexed: String,
    pub whisper: Option<String>,
    pub account: AccountInfo,
    pub price: Option<PriceInfo>,
    pub stash: Option<StashInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountInfo {
    pub name: String,
    pub online: Option<OnlineInfo>,
    pub language: Option<String>,
    pub realm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnlineInfo {
    pub league: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PriceInfo {
    #[serde(rename = "type")]
    pub price_type: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StashInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_trade_result() {
        let json = r#"{
            "result": [{
                "id": "c1b3f708d968f0fa2c14a4d1cecb4f691fdd6f0bae623d3c9b10cc27d2480545",
                "listing": {
                    "method": "psapi",
                    "indexed": "2025-02-07T02:26:42Z",
                    "whisper": "@OverEpix_a Hi, I would like to buy your Kraken Thunder Advanced Zealot Bow listed for 1 regal in Standard (stash tab \"for sell\"; position: left 1, top 1)",
                    "account": {
                        "name": "OverEpix#3364",
                        "online": {
                            "league": "Standard"
                        },
                        "language": "en_US",
                        "realm": "poe2"
                    },
                    "price": {
                        "type": "~price",
                        "amount": 1.0,
                        "currency": "regal"
                    },
                    "stash": {
                        "name": "for sell",
                        "x": 0,
                        "y": 0
                    }
                },
                "item": {
                    "realm": "poe2",
                    "verified": true,
                    "w": 2,
                    "h": 4,
                    "icon": "https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvV2VhcG9ucy9Ud29IYW5kV2VhcG9ucy9Cb3dzL0Jhc2V0eXBlcy9Cb3cwOCIsInciOjIsImgiOjQsInNjYWxlIjoxLCJyZWFsbSI6InBvZTIifV0/90f84002ea/Bow08.png",
                    "league": "Standard",
                    "id": "c1b3f708d968f0fa2c14a4d1cecb4f691fdd6f0bae623d3c9b10cc27d2480545",
                    "sockets": null,
                    "name": "Kraken Thunder",
                    "typeLine": "Advanced Zealot Bow",
                    "baseType": "Advanced Zealot Bow",
                    "rarity": "Rare",
                    "ilvl": 65,
                    "identified": true,
                    "corrupted": null,
                    "note": "~price 1 regal",
                    "properties": [
                        {
                            "name": "[Bow]",
                            "values": [],
                            "displayMode": 0,
                            "type": null
                        },
                        {
                            "name": "[Physical] Damage",
                            "values": [
                                [
                                    "72-113",
                                    1
                                ]
                            ],
                            "displayMode": 0,
                            "type": 9
                        },
                        {
                            "name": "Fire Damage",
                            "values": [
                                [
                                    "26-47",
                                    4
                                ]
                            ],
                            "displayMode": 0,
                            "type": 10
                        },
                        {
                            "name": "[Critical|Critical Hit] Chance",
                            "values": [
                                [
                                    "5.00%",
                                    0
                                ]
                            ],
                            "displayMode": 0,
                            "type": 12
                        },
                        {
                            "name": "Attacks per Second",
                            "values": [
                                [
                                    "1.20",
                                    0
                                ]
                            ],
                            "displayMode": 0,
                            "type": 13
                        }
                    ],
                    "requirements": [
                        {
                            "name": "Level",
                            "values": [
                                [
                                    "62",
                                    0
                                ]
                            ],
                            "displayMode": 0,
                            "type": 62
                        },
                        {
                            "name": "[Dexterity|Dex]",
                            "values": [
                                [
                                    "106",
                                    1
                                ]
                            ],
                            "displayMode": 1,
                            "type": 64
                        }
                    ],
                    "implicitMods": null,
                    "explicitMods": [
                        "28% increased [Physical] Damage",
                        "Adds 10 to 19 [Physical|Physical] Damage",
                        "Adds 26 to 47 [Fire|Fire] Damage",
                        "+54 to [Accuracy|Accuracy] Rating",
                        "25% reduced [Attributes|Attribute] Requirements",
                        "Bow [Attack|Attacks] fire an additional Arrow",
                        "[ManaLeech|Leeches] 6.28% of [Physical|Physical] Damage as Mana"
                    ],
                    "runeMods": null,
                    "frameType": 2,
                    "socketedItems": null,
                    "extended": {
                        "dps": 176.4,
                        "pdps": 132.6,
                        "edps": 43.8,
                        "dps_aug": true,
                        "pdps_aug": true,
                        "mods": {
                            "explicit": [
                                {
                                    "name": "Reaver's",
                                    "tier": "P3",
                                    "level": 23,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_1509134228",
                                            "min": "25",
                                            "max": "34"
                                        },
                                        {
                                            "hash": "explicit.stat_691932474",
                                            "min": "47",
                                            "max": "72"
                                        }
                                    ]
                                },
                                {
                                    "name": "Incinerating",
                                    "tier": "P7",
                                    "level": 51,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_709508406",
                                            "min": "26",
                                            "max": "39"
                                        },
                                        {
                                            "hash": "explicit.stat_709508406",
                                            "min": "40",
                                            "max": "59"
                                        }
                                    ]
                                },
                                {
                                    "name": "of the Talented",
                                    "tier": "S3",
                                    "level": 40,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_3639275092",
                                            "min": "-25",
                                            "max": "-25"
                                        }
                                    ]
                                },
                                {
                                    "name": "Honed",
                                    "tier": "P4",
                                    "level": 29,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_1940865751",
                                            "min": "8",
                                            "max": "12"
                                        },
                                        {
                                            "hash": "explicit.stat_1940865751",
                                            "min": "14",
                                            "max": "21"
                                        }
                                    ]
                                },
                                {
                                    "name": "of Splintering",
                                    "tier": "S1",
                                    "level": 55,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_3885405204",
                                            "min": "1",
                                            "max": "1"
                                        }
                                    ]
                                },
                                {
                                    "name": "of the Arid",
                                    "tier": "S3",
                                    "level": 54,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_669069897",
                                            "min": "6",
                                            "max": "6.9"
                                        }
                                    ]
                                }
                            ]
                        },
                        "hashes": {
                            "explicit": [
                                [
                                    "explicit.stat_1509134228",
                                    [
                                        0
                                    ]
                                ],
                                [
                                    "explicit.stat_1940865751",
                                    [
                                        3
                                    ]
                                ],
                                [
                                    "explicit.stat_709508406",
                                    [
                                        1
                                    ]
                                ],
                                [
                                    "explicit.stat_691932474",
                                    [
                                        0
                                    ]
                                ],
                                [
                                    "explicit.stat_3639275092",
                                    [
                                        2
                                    ]
                                ],
                                [
                                    "explicit.stat_3885405204",
                                    [
                                        4
                                    ]
                                ],
                                [
                                    "explicit.stat_669069897",
                                    [
                                        5
                                    ]
                                ]
                            ]
                        }
                    }
                }
            }],
            "total": 10000,
            "complexity": 8,
            "id": "oew9XGGcl"
        }"#;

        let result: TradeResult = serde_json::from_str(json).expect("Should parse successfully");

        // Verify top level fields
        assert_eq!(result.result.len(), 1);
        let item = &result.result[0];

        // Verify listing info
        assert_eq!(
            item.id,
            "c1b3f708d968f0fa2c14a4d1cecb4f691fdd6f0bae623d3c9b10cc27d2480545"
        );
        assert_eq!(item.listing.method, "psapi");
        assert_eq!(item.listing.indexed, "2025-02-07T02:26:42Z");

        // Verify stash info
        let stash = item.listing.stash.as_ref().unwrap();
        assert_eq!(stash.name, "for sell");
        assert_eq!(stash.x, 0);
        assert_eq!(stash.y, 0);

        // Verify account info
        assert_eq!(item.listing.account.name, "OverEpix#3364");
        let online = item.listing.account.online.as_ref().unwrap();
        assert_eq!(online.league, "Standard");
        assert_eq!(item.listing.account.language.as_ref().unwrap(), "en_US");
        assert_eq!(item.listing.account.realm.as_ref().unwrap(), "poe2");

        // Verify price info
        let price = item.listing.price.as_ref().unwrap();
        assert_eq!(price.price_type, "~price");
        assert_eq!(price.amount, 1.0);
        assert_eq!(price.currency, "regal");

        // Verify whisper
        assert!(item.listing.whisper.as_ref().unwrap().contains("@无丶聊"));

        // Verify item info
        let item_info = &item.item;
        assert_eq!(item_info.realm, "poe2");
        assert!(item_info.verified);
        assert_eq!(item_info.w, 2);
        assert_eq!(item_info.h, 4);
        assert!(item_info.icon.contains("Bow08.png"));
        assert_eq!(item_info.league, "Standard");
        assert_eq!(item_info.name, "Kraken Thunder");
        assert_eq!(item_info.type_line, "Advanced Zealot Bow");
        assert_eq!(item_info.base_type, "Advanced Zealot Bow");
        assert_eq!(item_info.rarity, Some("Rare".to_string()));
        assert_eq!(item_info.ilvl, Some(65));
        assert_eq!(item_info.identified, Some(true));
        assert_eq!(item_info.frame_type, 2);
        assert_eq!(item_info.note.as_ref().unwrap(), "~price 1 regal");

        // Verify sockets
        let sockets = item_info.sockets.as_ref().unwrap();
        assert_eq!(sockets.len(), 2);
        assert_eq!(sockets[0].group, 0);
        assert_eq!(sockets[0].socket_type, "rune");
        assert_eq!(sockets[1].group, 1);
        assert_eq!(sockets[1].socket_type, "rune");

        // Verify properties
        let properties = item_info.properties.as_ref().unwrap();
        assert_eq!(properties.len(), 5);
        assert_eq!(properties[0].name, "[Bow]");
        assert_eq!(properties[0].values.len(), 0);
        assert_eq!(properties[0].display_mode, 0);

        assert_eq!(properties[1].name, "[Physical] Damage");
        assert_eq!(properties[1].values[0].0, "72-113");
        assert_eq!(properties[1].values[0].1, 1);
        assert_eq!(properties[1].display_mode, 0);
        assert_eq!(properties[1].property_type.unwrap(), 9);

        assert_eq!(properties[2].name, "Fire Damage");
        assert_eq!(properties[2].values[0].0, "26-47");
        assert_eq!(properties[2].values[0].1, 4);
        assert_eq!(properties[2].display_mode, 0);
        assert_eq!(properties[2].property_type.unwrap(), 10);

        assert_eq!(properties[3].name, "[Critical|Critical Hit] Chance");
        assert_eq!(properties[3].values[0].0, "5.00%");
        assert_eq!(properties[3].values[0].1, 0);
        assert_eq!(properties[3].display_mode, 0);
        assert_eq!(properties[3].property_type.unwrap(), 12);

        assert_eq!(properties[4].name, "Attacks per Second");
        assert_eq!(properties[4].values[0].0, "1.20");
        assert_eq!(properties[4].values[0].1, 0);
        assert_eq!(properties[4].display_mode, 0);
        assert_eq!(properties[4].property_type.unwrap(), 13);

        // Verify requirements
        let requirements = item_info.requirements.as_ref().unwrap();
        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].name, "Level");
        assert_eq!(requirements[0].values[0].0, "62");
        assert_eq!(requirements[0].values[0].1, 0);
        assert_eq!(requirements[0].display_mode, 0);
        assert_eq!(requirements[0].requirement_type.unwrap(), 62);

        assert_eq!(requirements[1].name, "[Dexterity|Dex]");
        assert_eq!(requirements[1].values[0].0, "106");
        assert_eq!(requirements[1].values[0].1, 1);
        assert_eq!(requirements[1].display_mode, 1);
        assert_eq!(requirements[1].requirement_type.unwrap(), 64);

        // Verify mods
        if let ExtendedInfo::Full(extended) = &item_info.extended {
            let mods = extended.mods.as_ref().unwrap();
            let explicit_mods = mods.explicit.as_ref().unwrap();
            assert_eq!(explicit_mods.len(), 6);

            // Verify first mod (Reaver's - grants both phys damage and accuracy)
            assert_eq!(explicit_mods[0].name, "Reaver's");
            assert_eq!(explicit_mods[0].tier, "P3");
            assert_eq!(explicit_mods[0].level, 23);
            assert_eq!(explicit_mods[0].magnitudes.len(), 2);

            // Verify hash mapping
            let hashes = extended.hashes.as_ref().unwrap();
            let explicit_hashes = hashes.explicit.as_ref().unwrap();
            assert_eq!(explicit_hashes.len(), 7);

            // Verify first hash mapping (phys damage from Reaver's)
            assert_eq!(explicit_hashes[0].0, "explicit.stat_1509134228");
            assert_eq!(explicit_hashes[0].1, vec![0]);

            // Verify second hash mapping (flat phys from Honed)
            assert_eq!(explicit_hashes[1].0, "explicit.stat_1940865751");
            assert_eq!(explicit_hashes[1].1, vec![3]);
        } else {
            panic!("Expected ExtendedInfo::Full variant");
        }
    }
}
