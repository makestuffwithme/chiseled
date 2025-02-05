<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let checked: boolean;
	export let label: string;
	export let onToggle: (checked: boolean) => void;
</script>

<FilterRow enabled={checked} {label} {onToggle} />
