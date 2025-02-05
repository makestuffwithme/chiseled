<script lang="ts">
	import FilterRow from './FilterRow.svelte';

	export let filter: {
		text: string;
		enabled: boolean;
	};
	export let label: string;
	export let options: { value: string; label: string }[] | undefined = undefined;
</script>

<FilterRow enabled={filter.enabled} {label} onToggle={(checked) => (filter.enabled = checked)}>
	{#if options}
		<select
			class="p-0.5 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
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
			class="p-0.5 bg-surface-dark border-border border rounded text-text placeholder-text-muted disabled:opacity-50"
			bind:value={filter.text}
			disabled={!filter.enabled}
		/>
	{/if}
</FilterRow>
