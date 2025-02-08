<script lang="ts">
	export let title: string;
	export let enabled: boolean = true;

	let groupDiv: HTMLElement;

	function handleGroupToggle(e: Event) {
		const checked = (e.target as HTMLInputElement).checked;
		
		// Only find checkboxes that are direct children of this group
		if (groupDiv) {
			const childCheckboxes = groupDiv.querySelectorAll(':scope > div > label > input[type="checkbox"]');
			childCheckboxes.forEach(checkbox => {
				if (checkbox !== e.target) {
					(checkbox as HTMLInputElement).checked = checked;
					checkbox.dispatchEvent(new Event('change'));
				}
			});
		}
	}
</script>

<div 
	class="mb-2 px-2 pt-1 pb-2 bg-surface rounded shadow border border-border"
	bind:this={groupDiv}
>
	<div class="flex items-center gap-2 mb-1">
		<input
			type="checkbox"
			class="w-4 h-4 rounded border-gray-300 filter-group-checkbox"
			bind:checked={enabled}
			on:change={handleGroupToggle}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	<slot />
</div>
