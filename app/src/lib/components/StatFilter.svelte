<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';
	import type { StatFilter } from '../types/filters';

	export let filter: StatFilter;

	const filterId = `stat-filter-${filter.id}`;
</script>

<FilterRow
	bind:enabled={filter.enabled}
	label={filter.text}
	id={filterId}
>
	<RangeInputs
		bind:min={filter.value.min}
		bind:max={filter.value.max}
	/>
</FilterRow>
