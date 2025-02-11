<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	export let enabled: boolean = true;
	export let label: string;
	export let onToggle: (checked: boolean) => void;
	export let id: string;
</script>

<div class="p-0.5">
	<label class="flex items-center gap-2 text-sm">
		<LockableCheckbox checked={enabled} onChange={onToggle} {id} />
		<slot />
		<span class="text-text">{label}</span>
	</label>
</div>
