<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';

	export let filter: {
		id: string;
		text: string;
		enabled: boolean;
		value: {
			min: number | null;
			max: number | null;
		};
	};

	function updateFilter(key: 'min' | 'max', newValue: string) {
		filter.value[key] = newValue === '' ? null : Number(newValue);
		filter = filter;
	}
</script>

<FilterRow
	enabled={filter.enabled}
	label={filter.text}
	onToggle={(checked) => (filter.enabled = checked)}
>
	<RangeInputs
		min={filter.value.min}
		max={filter.value.max}
		disabled={!filter.enabled}
		onMinChange={(v) => updateFilter('min', v)}
		onMaxChange={(v) => updateFilter('max', v)}
	/>
</FilterRow>
