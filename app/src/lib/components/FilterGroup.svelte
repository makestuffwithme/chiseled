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

	let groupDiv: HTMLElement;
	const filterId = `filter-group-${title.toLowerCase().replace(/\s+/g, '-')}`;

	function handleGroupToggle(checked: boolean) {
		enabled = checked;
		
		// Update all filters in the group
		if (checked !== enabled) {
			filters.forEach(filter => {
				if (filter.rangeFilter) filter.rangeFilter.enabled = checked;
				if (filter.statFilter) filter.statFilter.enabled = checked;
				if (filter.textFilter) filter.textFilter.enabled = checked;
				if (filter.priceFilter) filter.priceFilter.enabled = checked;
				if (filter.toggleFilter) filter.toggleFilter.enabled = checked;
			});
		}
	}
</script>

<div 
	class="mb-2 px-2 pt-1 pb-2 bg-surface rounded shadow border border-border"
	bind:this={groupDiv}
>
	<div class="flex items-center gap-2 mb-1">
		<LockableCheckbox
			checked={enabled}
			onChange={handleGroupToggle}
			id={filterId}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	
	{#each filters as filter}
		{#if filter.rangeFilter}
			<RangeFilterInput filter={filter.rangeFilter} label={filter.label} />
		{:else if filter.statFilter}
			<StatFilterInput filter={filter.statFilter} />
		{:else if filter.textFilter}
			<TextFilterInput 
				filter={filter.textFilter} 
				label={filter.label}
				options={filter.options}
				readonly={filter.readonly}
			/>
		{:else if filter.priceFilter}
			<PriceFilterInput filter={filter.priceFilter} />
		{:else if filter.toggleFilter}
			<ToggleFilter 
				checked={filter.toggleFilter.enabled}
				label={filter.toggleFilter.label}
				onToggle={(checked) => filter.toggleFilter!.enabled = checked}
			/>
		{/if}
	{/each}
</div>
