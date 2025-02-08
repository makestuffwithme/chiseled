<script lang="ts">
	export let enabled: boolean = true;
	export let label: string;
	export let onToggle: (checked: boolean) => void;
</script>

<div class="p-0.5">
	<label class="flex items-center gap-2 text-sm">
		<input
			type="checkbox"
			class="w-4 h-4 rounded border-gray-300"
			bind:checked={enabled}
			on:change={(e) => onToggle(e.currentTarget.checked)}
		/>
		<slot />
		<span class="text-text">{label}</span>
	</label>
</div>
