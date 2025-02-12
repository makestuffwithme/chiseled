<script lang="ts">
	export let checked: boolean;
	export let locked: boolean = false;
	export let id: string;

	// Load initial locked state and saved checked state from localStorage
	$: {
		const savedState = localStorage.getItem(`filter-lock-${id}`);
		if (savedState) {
			const state = JSON.parse(savedState);
			locked = state.locked;
			if (locked) {
				checked = state.checkedState;
			}
		}
	}

	function toggleLock() {
		locked = !locked;
		if (locked) {
			// Save state when locking
			localStorage.setItem(
				`filter-lock-${id}`,
				JSON.stringify({
					locked: true,
					checkedState: checked
				})
			);
		} else {
			// Clear state when unlocking
			localStorage.removeItem(`filter-lock-${id}`);
		}
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
		</svg></button
	>
	<input
		type="checkbox"
		class="w-4 h-4 rounded border-gray-300"
		bind:checked
		disabled={locked}
	/>
</div>
