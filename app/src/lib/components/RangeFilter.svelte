<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';
	import type { RangeFilter } from '../types/filters';

	export let filter: RangeFilter;
	export let label: string;

	const filterId = `range-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;
</script>

<FilterRow bind:enabled={filter.enabled} {label} id={filterId}>
	<RangeInputs
		bind:min={filter.min}
		bind:max={filter.max}
	/>
</FilterRow>
