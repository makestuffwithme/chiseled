<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	import RangeFilterInput from './RangeFilter.svelte';
	import StatFilterInput from './StatFilter.svelte';
	import TextFilterInput from './TextFilter.svelte';
	import PriceFilterInput from './PriceFilter.svelte';
	import ToggleFilter from './ToggleFilter.svelte';
	import type { FilterConfig } from '../types/filters';

	function uuid() {
		return Math.random().toString(36).substring(2, 15);
	}

	export let title: string;
	export let enabled: boolean = true;
	export let filters: FilterConfig[] = [];

	const filterId = `filter-group-${title.toLowerCase().replace(/\s+/g, '-')}`;
	let filterContainer: HTMLDivElement;

	// Handle initial state and filter changes
	$: {
		if (filterContainer && filters) {
			setTimeout(() => {
				const groupCheckbox = filterContainer.parentElement?.querySelector(':scope > div > div > input[type="checkbox"]') as HTMLInputElement;
				const childCheckboxes = Array.from(filterContainer.querySelectorAll('input[type="checkbox"]')) as HTMLInputElement[];
				
				if (groupCheckbox) {
					// If group isn't locked and filters just changed, set it to checked by default
					if (!groupCheckbox.disabled && filters) {
						enabled = true;
						groupCheckbox.checked = true;
						groupCheckbox.dispatchEvent(new Event('change', { bubbles: true }));
					}

					childCheckboxes.forEach(checkbox => {
						if (!checkbox.disabled && checkbox.checked !== groupCheckbox.checked) {
							checkbox.checked = groupCheckbox.checked;
							checkbox.dispatchEvent(new Event('change', { bubbles: true }));
						}
					});
				}
			}, 0);
		}
	}

	// Handle runtime changes to enabled state
	$: if (filterContainer) {
		setTimeout(() => {
			const childCheckboxes = Array.from(filterContainer.querySelectorAll('input[type="checkbox"]')) as HTMLInputElement[];
			childCheckboxes.forEach(checkbox => {
				if (!checkbox.disabled && checkbox.checked !== enabled) {
					checkbox.checked = enabled;
					checkbox.dispatchEvent(new Event('change', { bubbles: true }));
				}
			});
		}, 0);
	}
</script>

<div class="mb-2 px-2 py-1 bg-surface rounded shadow border border-border" >
	<div class="flex items-center gap-2 mb-1">
		<LockableCheckbox
			bind:checked={enabled}
			id={filterId}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	
	<div bind:this={filterContainer}>
		{#each filters as filter (uuid())}
			{#if filter.rangeFilter}
				<RangeFilterInput bind:filter={filter.rangeFilter} label={filter.label} />
			{:else if filter.statFilter}
				<StatFilterInput bind:filter={filter.statFilter} />
			{:else if filter.textFilter}
				<TextFilterInput 
					bind:filter={filter.textFilter} 
					label={filter.label}
					options={filter.options}
					readonly={filter.readonly}
				/>
			{:else if filter.priceFilter}
				<PriceFilterInput bind:filter={filter.priceFilter} />
			{:else if filter.toggleFilter}
				<ToggleFilter 
					bind:checked={filter.toggleFilter.enabled}
					label={filter.toggleFilter.label}
				/>
			{/if}
		{/each}
	</div>
</div>
