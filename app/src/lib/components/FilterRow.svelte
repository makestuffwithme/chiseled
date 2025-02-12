<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	export let enabled: boolean;
	export let label: string;
	export let id: string;
</script>

<div class="p-0.5 pl-2">
	<label class="flex items-center gap-2 text-sm">
		<LockableCheckbox bind:checked={enabled} {id} />
		<slot />
		<span class="text-text">{label}</span>
	</label>
</div>
