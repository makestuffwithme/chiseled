<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let checked: boolean;
	export let label: string;
</script>

<FilterRow enabled={checked} {label} onToggle={(value) => checked = value} />
