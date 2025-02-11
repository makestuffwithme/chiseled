<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let filter: {
		text: string;
		enabled: boolean;
	};
	export let label: string;
	export let options: { value: string; label: string }[] | undefined = undefined;
	export let readonly = false;

	const filterId = `text-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;

	function updateFilter(newValue: string) {
		filter.text = newValue;
		filter = filter;
	}
</script>

<FilterRow enabled={filter.enabled} {label} id={filterId} onToggle={(value) => (filter.enabled = value)}>
	{#if options}
		<select
			class="p-0 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
			bind:value={filter.text}
			disabled={!filter.enabled}
		>
			{#each options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	{:else}
		<input
			type="text"
			class="p-0 px-1 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
			value={filter.text}
			on:input={(e) => updateFilter(e.currentTarget.value)}
			disabled={!filter.enabled || readonly}
		/>
	{/if}
</FilterRow>
