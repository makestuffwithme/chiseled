<script lang="ts">
	import LockableCheckbox from './LockableCheckbox.svelte';
	export let title: string;
	export let enabled: boolean = true;

	let groupDiv: HTMLElement;
	const filterId = `filter-group-${title.toLowerCase().replace(/\s+/g, '-')}`;

	function handleGroupToggle(checked: boolean) {
		enabled = checked;
		
		// Find all LockableCheckbox components within this group
		if (groupDiv) {
			const childLockableCheckboxes = groupDiv.querySelectorAll(':scope > div > label > div > input[type="checkbox"]');
			childLockableCheckboxes.forEach(checkbox => {
				// Skip if this checkbox is locked
				const lockButton = checkbox.parentElement?.querySelector('button');
				const isLocked = lockButton?.querySelector('svg')?.innerHTML.includes('v4');
				if (!isLocked) {
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
		<LockableCheckbox
			checked={enabled}
			onChange={handleGroupToggle}
			id={filterId}
		/>
		<h4 class="text-text capitalize font-semibold">{title}</h4>
	</div>
	<slot />
</div>
