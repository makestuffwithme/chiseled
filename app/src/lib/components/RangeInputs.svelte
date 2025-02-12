<script lang="ts">
	export let min: number | null;
	export let max: number | null;
</script>

<div class="flex gap-1">
	<input
		type="number"
		class="w-16 p-0.5 bg-surface-dark border-border border rounded text-text placeholder-text-muted disabled:opacity-50"
		placeholder="Min"
		bind:value={min}
	/>
	<input
		type="number"
		class="w-16 p-0.5 bg-surface-dark border-border border rounded text-text placeholder-text-muted disabled:opacity-50"
		placeholder="Max"
		bind:value={max}
	/>
</div>
