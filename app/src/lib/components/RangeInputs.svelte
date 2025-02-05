<script lang="ts">
	export let min: number | null;
	export let max: number | null;
	export let disabled: boolean = false;
	export let onMinChange: (value: string) => void;
	export let onMaxChange: (value: string) => void;
</script>

<div class="flex gap-1">
	<input
		type="number"
		class="w-16 p-0.5 bg-surface-dark border-border border rounded text-text placeholder-text-muted disabled:opacity-50"
		placeholder="Min"
		value={min}
		on:input={(e) => onMinChange(e.currentTarget.value)}
		{disabled}
	/>
	<input
		type="number"
		class="w-16 p-0.5 bg-surface-dark border-border border rounded text-text placeholder-text-muted disabled:opacity-50"
		placeholder="Max"
		value={max}
		on:input={(e) => onMaxChange(e.currentTarget.value)}
		{disabled}
	/>
</div>
