<script lang="ts">
	import { setContext } from 'svelte';
	import { writable } from 'svelte/store';

	export let title: string;
	export let enabled: boolean = true;

	let groupCheckbox: HTMLInputElement;
	const groupState = writable(enabled);

	setContext('filterGroup', groupState);

	$: {
		if (groupCheckbox) {
			groupState.set(enabled);
		}
	}
</script>

<div class="mb-2 px-2 pt-1 pb-2 bg-surface rounded shadow border border-border">
	<div class="flex items-center gap-2 mb-1">
		<input
			type="checkbox"
			class="w-4 h-4 rounded border-gray-300"
			bind:this={groupCheckbox}
			bind:checked={enabled}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	<slot />
</div>
