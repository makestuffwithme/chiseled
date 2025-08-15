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
</script>

<div class="mb-2 px-2 py-1 bg-surface rounded shadow border border-border" >
	<div class="flex items-center gap-2 mb-1">
		<LockableCheckbox
			bind:checked={enabled}
			id={filterId}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	
	<div>
		{#each filters as filter}
			{#if filter.rangeFilter}
				<RangeFilterInput bind:filter={filter.rangeFilter} label={filter.label} groupEnabled={enabled} />
			{:else if filter.statFilter}
				<StatFilterInput bind:filter={filter.statFilter} groupEnabled={enabled} />
			{:else if filter.textFilter}
				<TextFilterInput 
					bind:filter={filter.textFilter} 
					label={filter.label}
					options={filter.options}
					readonly={filter.readonly}
					groupEnabled={enabled}
					onChange={filter.onChange}
				/>
			{:else if filter.priceFilter}
				<PriceFilterInput bind:filter={filter.priceFilter} groupEnabled={enabled} />
			{:else if filter.toggleFilter}
				<ToggleFilter 
					bind:filter={filter.toggleFilter}
					label={filter.label}
					groupEnabled={enabled}
				/>
			{/if}
		{/each}
	</div>
</div>
