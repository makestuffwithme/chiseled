<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';

	export let filter: {
		min: number | null;
		max: number | null;
		enabled: boolean;
	};
	export let label: string;

	function updateFilter(key: 'min' | 'max', newValue: string) {
		filter[key] = newValue === '' ? null : Number(newValue);
		filter = filter;
	}
</script>

<FilterRow enabled={filter.enabled} {label} onToggle={(checked) => (filter.enabled = checked)}>
	<RangeInputs
		min={filter.min}
		max={filter.max}
		disabled={!filter.enabled}
		onMinChange={(v) => updateFilter('min', v)}
		onMaxChange={(v) => updateFilter('max', v)}
	/>
</FilterRow>
