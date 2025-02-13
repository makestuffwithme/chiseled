<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let checked: boolean;
	export let label: string;
	export let groupEnabled: boolean | undefined = undefined;

	const filterId = `toggle-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;
</script>

<FilterRow 
	bind:enabled={checked} 
	{label} 
	id={filterId}
	bind:groupEnabled
/>
