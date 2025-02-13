<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount, onDestroy } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { PhysicalSize, PhysicalPosition } from '@tauri-apps/api/window';
	import { message } from '@tauri-apps/plugin-dialog';
	import FilterGroup from './components/FilterGroup.svelte';
	import SearchResults from './components/SearchResults.svelte';
	import type { TradeFilters } from './types/filters';

	let searchResults: any = null;
	let isLoading = false;
	let error: string | null = null;
	let filters: TradeFilters | null = null;
	let hasBeenResized = false;
	let keydownHandler: (event: KeyboardEvent) => Promise<void>;
	let uuid = crypto.randomUUID();

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
				// we use a uuid here to guarantee a fresh form each time we get new filters
				uuid = crypto.randomUUID();
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
		<div class="p-2 pt-1">
			{#key uuid}
				<FilterGroup 
					title="Item Filters"
					filters={[
						filters.item_category && {
							label: "Item Category",
							textFilter: filters.item_category,
							options: [
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
							]
						},
						filters.item_name && {
							label: "Item Name",
							textFilter: filters.item_name,
							readonly: true
						},
						filters.item_base_type && {
							label: "Base Type",
							textFilter: filters.item_base_type,
							readonly: true
						},
						filters.rarity && {
							label: "Rarity",
							textFilter: filters.rarity,
							options: [
								{ value: "", label: "Any Rarity"},
								{ value: "unique", label: "Unique"},
								{ value: "rare", label: "Rare"},
								{ value: "magic", label: "Magic"},
								{ value: "normal", label: "Normal"},
								{ value: "nonunique", label: "Any Non-Unique"},
							]
						},
						filters.item_level && {
							label: "Item Level",
							rangeFilter: filters.item_level
						},
						filters.socket_count && {
							label: "Socket Count",
							rangeFilter: filters.socket_count
						}
					].filter((f): f is NonNullable<typeof f> => Boolean(f))}
				/>

				{#if filters.attack_speed || filters.physical_dps || filters.elemental_dps || filters.total_dps || filters.critical_chance}
					<FilterGroup 
						title="Damage Filters"
						filters={[
							filters.attack_speed && {
								label: "Attacks per Second",
								rangeFilter: filters.attack_speed
							},
							filters.physical_dps && {
								label: "Physical DPS",
								rangeFilter: filters.physical_dps
							},
							filters.elemental_dps && {
								label: "Elemental DPS",
								rangeFilter: filters.elemental_dps
							},
							filters.total_dps && {
								label: "Total DPS",
								rangeFilter: filters.total_dps
							},
							filters.critical_chance && {
								label: "Critical Hit Chance",
								rangeFilter: filters.critical_chance
							}
						].filter((f): f is NonNullable<typeof f> => Boolean(f))}
					/>
				{/if}

				{#if filters.explicit_mods.length > 0}
					<FilterGroup 
						title="Explicit Mods"
						filters={filters.explicit_mods.map(mod => ({
							label: mod.text,
							statFilter: mod
						}))}
					/>
				{/if}

				{#if filters.implicit_mods.length > 0}
					<FilterGroup 
						title="Implicit Mods"
						filters={filters.implicit_mods.map(mod => ({
							label: mod.text,
							statFilter: mod
						}))}
					/>
				{/if}

				{#if filters.rune_mods.length > 0}
					<FilterGroup 
						title="Rune Mods"
						filters={filters.rune_mods.map(mod => ({
							label: mod.text,
							statFilter: mod
						}))}
					/>
				{/if}

				<FilterGroup 
					title="Trade Filters"
					filters={[
						{
							label: "Online Only",
							toggleFilter: {
								enabled: filters.online_only,
								label: "Online Only"
							}
						},
						{
							label: "Price",
							priceFilter: filters.price
						}
					]}
				/>

				<button
					class="px-4 py-2 w-full bg-primary text-white rounded hover:bg-primary/90 disabled:bg-gray-400 disabled:cursor-not-allowed"
					on:click={searchTrade}
					disabled={isLoading}
				>
					{isLoading ? 'Searching...' : 'Search Trade'}
				</button>
			{/key}

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
