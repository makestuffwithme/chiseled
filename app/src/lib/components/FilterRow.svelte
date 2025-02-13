<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	export let enabled: boolean;
	export let label: string;
	export let id: string;
	export let groupEnabled: boolean | undefined = undefined;

	function handleChange(value: boolean) {
		enabled = value;
	}
</script>

<div class="p-0.5 pl-2 flex items-center gap-2 text-sm">
	<LockableCheckbox 
		checked={enabled} 
		{id} 
		{groupEnabled}
		onChange={handleChange}
	/>
	<slot />
	<span class="text-text">{label}</span>
</div>
