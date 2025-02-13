<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';
	import type { StatFilter } from '../types/filters';

	export let filter: StatFilter;
	export let groupEnabled: boolean | undefined = undefined;

	const filterId = `stat-filter-${filter.id}`;
</script>

<FilterRow
	bind:enabled={filter.enabled}
	label={filter.text}
	id={filterId}
	bind:groupEnabled
>
	<RangeInputs
		bind:min={filter.value.min}
		bind:max={filter.value.max}
	/>
</FilterRow>
