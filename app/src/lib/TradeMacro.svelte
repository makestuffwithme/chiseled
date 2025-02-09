<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount, onDestroy } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { PhysicalSize, PhysicalPosition } from '@tauri-apps/api/window';
	import { message } from '@tauri-apps/plugin-dialog';
	import RangeFilterInput from './components/RangeFilter.svelte';
	import StatFilterInput from './components/StatFilter.svelte';
	import TextFilterInput from './components/TextFilter.svelte';
	import PriceFilterInput from './components/PriceFilter.svelte';
	import ToggleFilter from './components/ToggleFilter.svelte';
	import FilterGroup from './components/FilterGroup.svelte';
	import SearchResults from './components/SearchResults.svelte';

	interface StatValue {
		min: number | null;
		max: number | null;
	}

	interface RangeFilter {
		min: number | null;
		max: number | null;
		enabled: boolean;
	}

	interface TextFilter {
		text: string;
		enabled: boolean;
	}

	interface StatFilter {
		id: string;
		text: string;
		enabled: boolean;
		value: StatValue;
	}

	interface PriceFilter {
		enabled: boolean;
		option: string;
		min: number | null;
		max: number | null;
	}

	interface TradeFilters {
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
		explicit_mods: StatFilter[];
		implicit_mods: StatFilter[];
		rune_mods: StatFilter[];
		price: PriceFilter;
		listed_time: string | null;
		online_only: boolean;
	}

	let searchResults: any = null;
	let isLoading = false;
	let error: string | null = null;
	let filters: TradeFilters | null = null;
	let hasBeenResized = false;
	let keydownHandler: (event: KeyboardEvent) => Promise<void>;

	// Reset filter group states when new filters are parsed
	$: if (filters) {
		// Reset all filter groups to enabled by default
		const filterGroups = document.querySelectorAll('.filter-group-checkbox') as NodeListOf<HTMLInputElement>;
		filterGroups.forEach(checkbox => {
			checkbox.checked = true;
		});
	}

	onMount(async () => {
		keydownHandler = async (event: KeyboardEvent) => {
			if (event.key === 'Escape') {
				event.preventDefault();
				try {
					await invoke('minimize_window');
				} catch (err) {
					console.error('Error minimizing window:', err);
				}
			}
		};

		window.addEventListener('keydown', keydownHandler);

		await listen('parsed_filters', async (event: any) => {
			try {
				const parsedFilters = JSON.parse(event.payload);

				if (!hasBeenResized) {
					try {
						const appWindow = await getCurrentWindow();
						await appWindow.setSize(new PhysicalSize(510, window.screen.height - 62));
						await appWindow.setPosition(new PhysicalPosition(1155, 0));
						hasBeenResized = true;
					} catch (windowErr) {
						console.error('Error resizing window:', windowErr);
						await message('Failed to resize window: ' + windowErr, {
							title: 'Window Error',
							kind: 'error'
						});
					}
				}

				searchResults = null;
				error = null;
				isLoading = false;
				filters = parsedFilters;
			} catch (err) {
				console.error('Error parsing filters:', err);
				error = err instanceof Error ? err.message : 'Error parsing filters';
			}
		});
	});

	onDestroy(() => {
		if (keydownHandler) {
			window.removeEventListener('keydown', keydownHandler);
		}
	});

	async function searchTrade() {
		if (!filters) return;

		try {
			isLoading = true;
			error = null;
			const response = (await invoke('search_trade', {
				filters: JSON.stringify(filters)
			})) as string;
			searchResults = JSON.parse(response);
		} catch (err) {
			console.error('Error searching trade:', err);
			error = String(err);
			searchResults = null;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="h-screen w-full overflow-y-auto bg-surface-dark">
	{#if !filters}
		<div class="h-full flex items-center justify-center p-8 text-center">
			<div>
				<p class="text-text-muted">
					To start, hover over an item in POE2 and press <kbd class="px-2 py-1 bg-surface rounded"
						>Ctrl+D</kbd
					>
				</p><br>
				<p class="text-text-muted">
					Press <kbd class="px-2 py-1 bg-surface rounded">Esc</kbd> to minimize the window and return to POE2
				</p>
			</div>
		</div>
	{:else}
		<div class="p-4 pt-1">
			{#if filters}
				<FilterGroup title="Item Filters">
					<!-- Item Category -->
					{#if filters.item_category}
						<TextFilterInput
							filter={filters.item_category}
							label="Item Category"
							options={[
								// One-Handed Weapons
								{ value: 'weapon.claw', label: 'Claw' },
								{ value: 'weapon.dagger', label: 'Dagger' },
								{ value: 'weapon.onesword', label: 'One-Handed Sword' },
								{ value: 'weapon.oneaxe', label: 'One-Handed Axe' },
								{ value: 'weapon.onemace', label: 'One-Handed Mace' },
								{ value: 'weapon.spear', label: 'Spear' },
								{ value: 'weapon.flail', label: 'Flail' },

								// Two-Handed Weapons
								{ value: 'weapon.twosword', label: 'Two-Handed Sword' },
								{ value: 'weapon.twoaxe', label: 'Two-Handed Axe' },
								{ value: 'weapon.twomace', label: 'Two-Handed Mace' },
								{ value: 'weapon.warstaff', label: 'Quarterstaff' },

								// Ranged Weapons
								{ value: 'weapon.bow', label: 'Bow' },
								{ value: 'weapon.crossbow', label: 'Crossbow' },

								// Caster Weapons
								{ value: 'weapon.wand', label: 'Wand' },
								{ value: 'weapon.sceptre', label: 'Sceptre' },
								{ value: 'weapon.staff', label: 'Staff' },

								// Armour
								{ value: 'armour.helmet', label: 'Helmet' },
								{ value: 'armour.chest', label: 'Body Armour' },
								{ value: 'armour.gloves', label: 'Gloves' },
								{ value: 'armour.boots', label: 'Boots' },
								{ value: 'armour.quiver', label: 'Quiver' },
								{ value: 'armour.shield', label: 'Shield' },
								{ value: 'armour.focus', label: 'Focus' },
								{ value: 'armour.buckler', label: 'Buckler' },

								// Accessories
								{ value: 'accessory.amulet', label: 'Amulet' },
								{ value: 'accessory.belt', label: 'Belt' },
								{ value: 'accessory.ring', label: 'Ring' },

								// Gems
								{ value: 'gem', label: 'Any Gem' },
								{ value: 'gem.activegem', label: 'Skill Gem' },
								{ value: 'gem.supportgem', label: 'Support Gem' },
								{ value: 'gem.metagem', label: 'Meta Gem' },

								// Flasks
								{ value: 'flask', label: 'Any Flask' },
								{ value: 'flask.life', label: 'Life Flask' },
								{ value: 'flask.mana', label: 'Mana Flask' },

								// Endgame Items
								{ value: 'map.waystone', label: 'Waystone' },
								{ value: 'map.fragment', label: 'Map Fragment' },
								{ value: 'map.logbook', label: 'Logbook' },
								{ value: 'map.breachstone', label: 'Breachstone' },
								{ value: 'map.barya', label: 'Barya' },
								{ value: 'map.bosskey', label: 'Pinnacle Key' },
								{ value: 'map.ultimatum', label: 'Ultimatum Key' },
								{ value: 'map.tablet', label: 'Tablet' },

								// Misc Items
								{ value: 'card', label: 'Divination Card' },
								{ value: 'sanctum.relic', label: 'Relic' },

								{ value: 'currency.omen', label: 'Omen' },
								{ value: 'currency.rune', label: 'Rune' },
								{ value: 'currency.soulcore', label: 'Soul Core' }
							]}
						/>
					{/if}

					<!-- Item Name -->
					{#if filters.item_name}
						<TextFilterInput
							filter={filters.item_name}
							label="Item Name"
							readonly={true}
						/>
					{/if}

					<!-- Base Type -->
					{#if filters.item_base_type}
						<TextFilterInput
							filter={filters.item_base_type}
							label="Base Type"
							readonly={true}
						/>
					{/if}

					<!-- Item Level Filter -->
					{#if filters.item_level}
						<RangeFilterInput filter={filters.item_level} label="Item Level" />
					{/if}

					<!-- Attack Speed Filter -->
					{#if filters.attack_speed}
						<RangeFilterInput filter={filters.attack_speed} label="Attacks per Second" />
					{/if}

					<!-- DPS Filters -->
					{#if filters.physical_dps || filters.elemental_dps || filters.total_dps}
						{#if filters.physical_dps}
							<RangeFilterInput filter={filters.physical_dps} label="Physical DPS" />
						{/if}
						{#if filters.elemental_dps}
							<RangeFilterInput filter={filters.elemental_dps} label="Elemental DPS" />
						{/if}
						{#if filters.total_dps}
							<RangeFilterInput filter={filters.total_dps} label="Total DPS" />
						{/if}
					{/if}

					<!-- Critical Chance Filter -->
					{#if filters.critical_chance}
						<RangeFilterInput filter={filters.critical_chance} label="Critical Hit Chance" />
					{/if}

					<!-- Socket Count Filter -->
					{#if filters.socket_count}
						<RangeFilterInput filter={filters.socket_count} label="Socket Count" />
					{/if}
				</FilterGroup>

				<!-- Explicit Mods -->

				{#if filters.explicit_mods.length > 0}
					<FilterGroup title="Explicit Mods">
						{#each filters.explicit_mods as mod}
							<StatFilterInput filter={mod} />
						{/each}
					</FilterGroup>
				{/if}

				<!-- Implicit Mods -->
				{#if filters.implicit_mods.length > 0}
					<FilterGroup title="Implicit Mods">
						{#each filters.implicit_mods as mod}
							<StatFilterInput filter={mod} />
						{/each}
					</FilterGroup>
				{/if}

				<!-- Rune Mods -->
				{#if filters.rune_mods.length > 0}
					<FilterGroup title="Rune Mods">
						{#each filters.rune_mods as mod}
							<StatFilterInput filter={mod} />
						{/each}
					</FilterGroup>
				{/if}

				<!-- Price Filter -->
				<FilterGroup title="Trade Filters">
					<ToggleFilter
						checked={filters!.online_only}
						label="Online Only"
					/>

					<PriceFilterInput
						filter={filters!.price}
					/>
				</FilterGroup>

				<button
					class="px-4 py-2 w-full bg-primary text-white rounded hover:bg-primary/90 disabled:bg-gray-400 disabled:cursor-not-allowed"
					on:click={searchTrade}
					disabled={isLoading}
				>
					{isLoading ? 'Searching...' : 'Search Trade'}
				</button>
			{/if}

			<div class="mt-2">
				{#if error}
					<div class="p-2 bg-red-50 text-red-600 rounded">
						{error}
					</div>
				{:else if isLoading}
					<div class="text-text-muted italic">
						{filters ? 'Searching trade...' : 'Parsing item...'}
					</div>
				{:else if searchResults?.result}
					<SearchResults results={searchResults.result} />
				{/if}
			</div>
		</div>
	{/if}
</div>
