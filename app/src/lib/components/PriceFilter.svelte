<script lang="ts">
	import RangeInputs from './RangeInputs.svelte';
	import FilterRow from './FilterRow.svelte';

	export let filter: {
		enabled: boolean;
		option: string;
		min: number | null;
		max: number | null;
	};

	const options = [
		{ value: '', label: 'Exalted Orb Equivalent' },
		{ value: 'exalted', label: 'Exalted Orbs' },
		{ value: 'divine', label: 'Divine Orbs' },
		{ value: 'exalted_divine', label: 'Exalted Orbs or Divine Orbs' }
	];

	const filterId = 'price-filter';

	function updateFilter(key: 'min' | 'max', newValue: string) {
		filter[key] = newValue === '' ? null : Number(newValue);
		filter = filter;
	}
</script>

<FilterRow
	enabled={filter.enabled}
	label="Price"
	id={filterId}
	onToggle={(value) => {
		filter.enabled = value;
		filter = filter;
	}}
>
	<RangeInputs
		min={filter.min}
		max={filter.max}
		disabled={!filter.enabled}
		onMinChange={(v) => updateFilter('min', v)}
		onMaxChange={(v) => updateFilter('max', v)}
	/>
	<select
		class="p-0.5 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
		bind:value={filter.option}
		disabled={!filter.enabled}
	>
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>
</FilterRow>
