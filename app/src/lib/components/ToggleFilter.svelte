<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let checked: boolean;
	export let label: string;
	export let onToggle: (checked: boolean) => void;

	const filterId = `toggle-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;
</script>

<FilterRow enabled={checked} {label} id={filterId} {onToggle} />
