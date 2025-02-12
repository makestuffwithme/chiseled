<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	import RangeFilterInput from './RangeFilter.svelte';
	import StatFilterInput from './StatFilter.svelte';
	import TextFilterInput from './TextFilter.svelte';
	import PriceFilterInput from './PriceFilter.svelte';
	import ToggleFilter from './ToggleFilter.svelte';
	import type { FilterConfig } from '../types/filters';

	export let title: string;
	export let enabled: boolean = true;
	export let filters: FilterConfig[] = [];

	const filterId = `filter-group-${title.toLowerCase().replace(/\s+/g, '-')}`;
	let filterContainer: HTMLDivElement;

	$: if (filterContainer) {
		const checkboxes = Array.from(filterContainer.querySelectorAll('input[type="checkbox"]')) as HTMLInputElement[];
		checkboxes.forEach(checkbox => {
			// Update each checkbox that isn't disabled (locked)
			if (!checkbox.disabled && checkbox !== document.activeElement) {
				checkbox.checked = enabled;
				checkbox.dispatchEvent(new Event('change', { bubbles: true }));
			}
		});
	}

	// Set non-locked group checkboxes to checked when filters change
	$: if (filters && filterContainer) {
		const groupCheckbox = filterContainer.parentElement?.querySelector(':scope > div > div > input[type="checkbox"]') as HTMLInputElement;
		if (groupCheckbox && !groupCheckbox.disabled) {
			enabled = true;
			groupCheckbox.checked = true;
			groupCheckbox.dispatchEvent(new Event('change', { bubbles: true }));
		}
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
		{#each filters as filter}
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
