use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeResult {
    pub result: Vec<ItemListing>,
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
    pub rarity: String,
    pub ilvl: i32,
    pub identified: bool,
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
    pub extended: Option<ItemExtendedInfo>,
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
                "id": "1e1e45ea81d32638af21ccb285932c447a36942a653eb3973add1d7b2eeb57bf",
                "listing": {
                    "method": "psapi",
                    "indexed": "2025-01-29T17:06:53Z",
                    "stash": {
                        "name": "弓",
                        "x": 0,
                        "y": 4
                    },
                    "whisper": "@无丶聊 你好，我想購買 巨獸 影弦 專家短弓 標價 1 transmute 在 Standard (倉庫頁 \"弓\"; 位置: 左 1, 上 5)",
                    "account": {
                        "name": "拾丶壹#4422",
                        "online": {
                            "league": "Standard"
                        },
                        "lastCharacterName": "无丶聊",
                        "language": "zh_TW",
                        "realm": "poe2"
                    },
                    "price": {
                        "type": "~price",
                        "amount": 1,
                        "currency": "transmute"
                    }
                },
                "item": {
                    "realm": "poe2",
                    "verified": true,
                    "w": 2,
                    "h": 4,
                    "icon": "https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvV2VhcG9ucy9Ud29IYW5kV2VhcG9ucy9Cb3dzL0Jhc2V0eXBlcy9Cb3cwMiIsInciOjIsImgiOjQsInNjYWxlIjoxLCJyZWFsbSI6InBvZTIifV0/1ea0e1977e/Bow02.png",
                    "league": "Standard",
                    "id": "1e1e45ea81d32638af21ccb285932c447a36942a653eb3973add1d7b2eeb57bf",
                    "sockets": [
                        {
                            "group": 0,
                            "type": "rune"
                        },
                        {
                            "group": 1,
                            "type": "rune"
                        }
                    ],
                    "name": "Behemoth Nock",
                    "typeLine": "Expert Shortbow",
                    "baseType": "Expert Shortbow",
                    "rarity": "Rare",
                    "ilvl": 67,
                    "identified": true,
                    "note": "~price 1 transmute",
                    "properties": [
                        {
                            "name": "[Bow]",
                            "values": [],
                            "displayMode": 0
                        },
                        {
                            "name": "[Physical] Damage",
                            "values": [["57-101", 1]],
                            "displayMode": 0,
                            "type": 9
                        },
                        {
                            "name": "Fire Damage",
                            "values": [
                                [
                                    "51-62",
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
                                    "1.49",
                                    1
                                ]
                            ],
                            "displayMode": 0,
                            "type": 13
                        }
                    ],
                    "requirements": [
                        {
                            "name": "Level",
                            "values": [["67", 0]],
                            "displayMode": 0,
                            "type": 62
                        },
                        {
                            "name": "[Dexterity|Dex]",
                            "values": [
                                [
                                    "147",
                                    1
                                ]
                            ],
                            "displayMode": 1,
                            "type": 64
                        }
                    ],
                    "explicitMods": [
                        "Adds 16 to 25 [Physical|Physical] Damage",
                        "Adds 51 to 62 [Fire|Fire] Damage",
                        "19% increased [Attack] Speed",
                        "15% reduced [Attributes|Attribute] Requirements",
                        "Grants 4 Life per Enemy [HitDamage|Hit]"
                    ],
                    "frameType": 2,
                    "socketedItems": [],
                    "extended": {
                        "dps": 224.99,
                        "pdps": 140.81,
                        "edps": 84.19,
                        "dps_aug": true,
                        "pdps_aug": true,
                        "mods": {
                            "explicit": [
                                {
                                    "name": "of the Worthy",
                                    "tier": "S1",
                                    "level": 24,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_3639275092",
                                            "min": "-15",
                                            "max": "-15"
                                        }
                                    ]
                                },
                                {
                                    "name": "Blasting",
                                    "tier": "P8",
                                    "level": 62,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_709508406",
                                            "min": "35",
                                            "max": "52"
                                        },
                                        {
                                            "hash": "explicit.stat_709508406",
                                            "min": "53",
                                            "max": "79"
                                        }
                                    ]
                                },
                                {
                                    "name": "of Acclaim",
                                    "tier": "S5",
                                    "level": 37,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_210067635",
                                            "min": "17",
                                            "max": "19"
                                        }
                                    ]
                                },
                                {
                                    "name": "of Regrowth",
                                    "tier": "S3",
                                    "level": 30,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_821021828",
                                            "min": "4",
                                            "max": "4"
                                        }
                                    ]
                                },
                                {
                                    "name": "Annealed",
                                    "tier": "P6",
                                    "level": 46,
                                    "magnitudes": [
                                        {
                                            "hash": "explicit.stat_1940865751",
                                            "min": "13",
                                            "max": "20"
                                        },
                                        {
                                            "hash": "explicit.stat_1940865751",
                                            "min": "23",
                                            "max": "35"
                                        }
                                    ]
                                }
                            ]
                        },
                        "hashes": {
                            "explicit": [
                                [
                                    "explicit.stat_1940865751",
                                    [
                                        4
                                    ]
                                ],
                                [
                                    "explicit.stat_709508406",
                                    [
                                        1
                                    ]
                                ],
                                [
                                    "explicit.stat_210067635",
                                    [
                                        2
                                    ]
                                ],
                                [
                                    "explicit.stat_3639275092",
                                    [
                                        0
                                    ]
                                ],
                                [
                                    "explicit.stat_821021828",
                                    [
                                        3
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
            "1e1e45ea81d32638af21ccb285932c447a36942a653eb3973add1d7b2eeb57bf"
        );
        assert_eq!(item.listing.method, "psapi");
        assert_eq!(item.listing.indexed, "2025-01-29T17:06:53Z");

        // Verify stash info
        let stash = item.listing.stash.as_ref().unwrap();
        assert_eq!(stash.name, "弓");
        assert_eq!(stash.x, 0);
        assert_eq!(stash.y, 4);

        // Verify account info
        assert_eq!(item.listing.account.name, "拾丶壹#4422");
        let online = item.listing.account.online.as_ref().unwrap();
        assert_eq!(online.league, "Standard");
        assert_eq!(item.listing.account.language.as_ref().unwrap(), "zh_TW");
        assert_eq!(item.listing.account.realm.as_ref().unwrap(), "poe2");

        // Verify price info
        let price = item.listing.price.as_ref().unwrap();
        assert_eq!(price.price_type, "~price");
        assert_eq!(price.amount, 1.0);
        assert_eq!(price.currency, "transmute");

        // Verify whisper
        assert!(item.listing.whisper.as_ref().unwrap().contains("@无丶聊"));

        // Verify item info
        let item_info = &item.item;
        assert_eq!(item_info.realm, "poe2");
        assert!(item_info.verified);
        assert_eq!(item_info.w, 2);
        assert_eq!(item_info.h, 4);
        assert!(item_info.icon.contains("Bow02.png"));
        assert_eq!(item_info.league, "Standard");
        assert_eq!(item_info.name, "Behemoth Nock");
        assert_eq!(item_info.type_line, "Expert Shortbow");
        assert_eq!(item_info.base_type, "Expert Shortbow");
        assert_eq!(item_info.rarity, "Rare");
        assert_eq!(item_info.ilvl, 67);
        assert!(item_info.identified);
        assert_eq!(item_info.frame_type, 2);
        assert_eq!(item_info.note.as_ref().unwrap(), "~price 1 transmute");

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
        assert_eq!(properties[1].values[0].0, "57-101");
        assert_eq!(properties[1].values[0].1, 1);
        assert_eq!(properties[1].display_mode, 0);
        assert_eq!(properties[1].property_type.unwrap(), 9);

        assert_eq!(properties[2].name, "Fire Damage");
        assert_eq!(properties[2].values[0].0, "51-62");
        assert_eq!(properties[2].values[0].1, 4);
        assert_eq!(properties[2].display_mode, 0);
        assert_eq!(properties[2].property_type.unwrap(), 10);

        assert_eq!(properties[3].name, "[Critical|Critical Hit] Chance");
        assert_eq!(properties[3].values[0].0, "5.00%");
        assert_eq!(properties[3].values[0].1, 0);
        assert_eq!(properties[3].display_mode, 0);
        assert_eq!(properties[3].property_type.unwrap(), 12);

        assert_eq!(properties[4].name, "Attacks per Second");
        assert_eq!(properties[4].values[0].0, "1.49");
        assert_eq!(properties[4].values[0].1, 1);
        assert_eq!(properties[4].display_mode, 0);
        assert_eq!(properties[4].property_type.unwrap(), 13);

        // Verify requirements
        let requirements = item_info.requirements.as_ref().unwrap();
        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].name, "Level");
        assert_eq!(requirements[0].values[0].0, "67");
        assert_eq!(requirements[0].values[0].1, 0);
        assert_eq!(requirements[0].display_mode, 0);
        assert_eq!(requirements[0].requirement_type.unwrap(), 62);

        assert_eq!(requirements[1].name, "[Dexterity|Dex]");
        assert_eq!(requirements[1].values[0].0, "147");
        assert_eq!(requirements[1].values[0].1, 1);
        assert_eq!(requirements[1].display_mode, 1);
        assert_eq!(requirements[1].requirement_type.unwrap(), 64);

        // Verify mods
        let explicit_mods = item_info.explicit_mods.as_ref().unwrap();
        assert_eq!(explicit_mods.len(), 5);
        assert_eq!(explicit_mods[0], "Adds 16 to 25 [Physical|Physical] Damage");
        assert_eq!(explicit_mods[1], "Adds 51 to 62 [Fire|Fire] Damage");
        assert_eq!(explicit_mods[2], "19% increased [Attack] Speed");
        assert_eq!(
            explicit_mods[3],
            "15% reduced [Attributes|Attribute] Requirements"
        );
        assert_eq!(explicit_mods[4], "Grants 4 Life per Enemy [HitDamage|Hit]");

        // Verify extended info
        let extended = item_info.extended.as_ref().unwrap();
        assert_eq!(extended.dps.unwrap(), 224.99);
        assert_eq!(extended.pdps.unwrap(), 140.81);
        assert_eq!(extended.edps.unwrap(), 84.19);
        assert!(extended.dps_augmented.unwrap());
        assert!(extended.pdps_augmented.unwrap());

        // Verify mod details
        let mods = extended.mods.as_ref().unwrap();
        assert_eq!(mods.explicit.as_ref().unwrap().len(), 5);

        let explicit_mod = &mods.explicit.as_ref().unwrap()[0];
        assert_eq!(explicit_mod.name, "of the Worthy");
        assert_eq!(explicit_mod.tier, "S1");
        assert_eq!(explicit_mod.level, 24);
        assert_eq!(explicit_mod.magnitudes[0].hash, "explicit.stat_3639275092");
        assert_eq!(explicit_mod.magnitudes[0].min, "-15");
        assert_eq!(explicit_mod.magnitudes[0].max, "-15");

        // Verify all explicit mods
        assert_eq!(mods.explicit.as_ref().unwrap()[1].name, "Blasting");
        assert_eq!(mods.explicit.as_ref().unwrap()[1].tier, "P8");
        assert_eq!(mods.explicit.as_ref().unwrap()[2].name, "of Acclaim");
        assert_eq!(mods.explicit.as_ref().unwrap()[2].tier, "S5");
        assert_eq!(mods.explicit.as_ref().unwrap()[3].name, "of Regrowth");
        assert_eq!(mods.explicit.as_ref().unwrap()[3].tier, "S3");
        assert_eq!(mods.explicit.as_ref().unwrap()[4].name, "Annealed");
        assert_eq!(mods.explicit.as_ref().unwrap()[4].tier, "P6");

        // Verify hashes
        let hashes = extended.hashes.as_ref().unwrap();
        assert_eq!(hashes.explicit.as_ref().unwrap().len(), 5);
        assert_eq!(
            hashes.explicit.as_ref().unwrap()[0].0,
            "explicit.stat_1940865751"
        );
        assert_eq!(hashes.explicit.as_ref().unwrap()[0].1[0], 4);
    }
}
