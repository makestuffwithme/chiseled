<script lang="ts">
	import FilterRow from './FilterRow.svelte';
	import type { TextFilter } from '../types/filters';

	export let filter: TextFilter;
	export let label: string;
	export let options: { value: string; label: string }[] | undefined = undefined;
	export let readonly = false;
	export let groupEnabled: boolean | undefined = undefined;
	export let onChange: ((value: string) => void) | undefined = undefined;

	const filterId = `text-filter-${label.toLowerCase().replace(/\s+/g, '-')}`;
</script>

<FilterRow bind:enabled={filter.enabled} {label} id={filterId} bind:groupEnabled>
	{#if options}
		<select
			class="p-0 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
			bind:value={filter.text}
			on:change={() => onChange?.(filter.text)}
			disabled={readonly}
		>
			{#each options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	{:else}
		<input
			type="text"
			class="p-0 px-1 bg-surface-dark border-border border rounded text-text disabled:opacity-50"
			bind:value={filter.text}
			on:input={() => onChange?.(filter.text)}
			disabled={readonly}
		/>
	{/if}
</FilterRow>
