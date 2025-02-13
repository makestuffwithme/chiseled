export interface StatValue {
    min: number | null;
    max: number | null;
}

export interface RangeFilter {
    min: number | null;
    max: number | null;
    enabled: boolean;
}

export interface TextFilter {
    text: string;
    enabled: boolean;
}

export interface StatFilter {
    id: string;
    text: string;
    enabled: boolean;
    value: StatValue;
}

export interface PriceFilter {
    enabled: boolean;
    option: string;
    min: number | null;
    max: number | null;
}

export interface ToggleFilter {
    enabled: boolean;
    label: string;
}

export interface FilterConfig {
    label: string;
    statFilter?: StatFilter;
    rangeFilter?: RangeFilter;
    textFilter?: TextFilter;
    priceFilter?: PriceFilter;
    toggleFilter?: ToggleFilter;
    options?: { value: string; label: string }[];
    readonly?: boolean;
}

export interface TradeFilters {
    item_category: TextFilter | null;
    item_name: TextFilter | null;
    item_base_type: TextFilter | null;
    rarity: TextFilter | null;
    item_level: RangeFilter | null;
    physical_dps: RangeFilter | null;
    elemental_dps: RangeFilter | null;
    total_dps: RangeFilter | null;
    attack_speed: RangeFilter | null;
    critical_chance: RangeFilter | null;
    socket_count: RangeFilter | null;

    // Armour Properties
    armour: RangeFilter | null;
    energy_shield: RangeFilter | null;
    evasion: RangeFilter | null;

    explicit_mods: StatFilter[];
    implicit_mods: StatFilter[];
    rune_mods: StatFilter[];
    price: PriceFilter;
    listed_time: string | null;
    online_only: boolean;
} 