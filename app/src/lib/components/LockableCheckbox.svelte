<script lang="ts">
	export let checked: boolean;
	export let locked: boolean = false;
	export let id: string;

	// Only load initial state on component mount
	const savedState = localStorage.getItem(`filter-lock-${id}`);
	if (savedState !== null) {
		console.log(`[${id}] Initial load - setting from storage:`, savedState);
		locked = true;
		checked = savedState === 'true';
	}

	// Handle changes to locked state
	$: {
		if (locked) {
			localStorage.setItem(`filter-lock-${id}`, checked.toString());
		} else {
			localStorage.removeItem(`filter-lock-${id}`);
		}
	}

	function toggleLock() {
		locked = !locked;
	}
</script>

<div class="flex items-center gap-1">
	<button
		class="{locked ? 'text-gray-300' : 'text-gray-500'} hover:text-gray-300 transition-colors"
		on:click={toggleLock}
		title={locked ? 'Unlock' : 'Lock'}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-5 w-5 focus:outline-none"
			viewBox="0 0 16 16"
			fill="none"
			stroke="currentColor"
			stroke-width="1.25"
			pointer-events="none"
		>
			{#if locked}
				<rect x="0" y="0" width="16" height="16" fill="transparent" stroke-opacity="0" />
				<rect x="3" y="7" width="10" height="7" rx="1" pointer-events="none" />
				<path d="M5 7V5a3 3 0 0 1 6 0v2" pointer-events="none" />
			{:else}
				<rect x="0" y="0" width="16" height="16" fill="transparent" stroke-opacity="0" />
				<rect x="3" y="7" width="10" height="7" rx="1" pointer-events="none" />
				<path d="M5 7V5a3 3 0 0 1 6 0" pointer-events="none" />
			{/if}
		</svg>
	</button>
	<input type="checkbox" class="w-4 h-4 rounded border-gray-300" bind:checked disabled={locked} />
</div>

<!-- you cant style a disabled checkbox (ffs), but you can apply a filter -->
<style>
	input[type='checkbox'][disabled] {
		filter: invert(100%) hue-rotate(9deg) brightness(4);
	}
</style>
