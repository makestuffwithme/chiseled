<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';
	import type { PriceFilter } from '../types/filters';

	export let filter: PriceFilter;

	const options = [
		{ value: '', label: 'Exalted Orb Equivalent' },
		{ value: 'exalted', label: 'Exalted Orbs' },
		{ value: 'divine', label: 'Divine Orbs' },
		{ value: 'exalted_divine', label: 'Exalted Orbs or Divine Orbs' }
	];

	const filterId = 'price-filter';
</script>

<FilterRow
	bind:enabled={filter.enabled}
	label="Price"
	id={filterId}
>
	<RangeInputs
		bind:min={filter.min}
		bind:max={filter.max}
	/>
	<select
		class="p-0.5 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
		bind:value={filter.option}
	>
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>
</FilterRow>
