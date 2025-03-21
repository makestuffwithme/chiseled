<script lang="ts">
	import FilterRow from './FilterRow.svelte';
	import type { ToggleFilter } from '../types/filters';
	
	export let filter: ToggleFilter;
	export let label: string;
	export let groupEnabled: boolean | undefined = undefined;

	const filterId = `toggle-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;
</script>

<FilterRow 
	bind:enabled={filter.enabled} 
	{label} 
	id={filterId}
	bind:groupEnabled
/>
