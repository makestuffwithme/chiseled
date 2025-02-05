<script lang="ts">
	export let results: {
		listing?: {
			price?: {
				amount: number;
				currency: string;
			};
			indexed: string;
			account?: {
				name: string;
				online?: {
					league: string;
					status?: string;
				};
			};
			whisper?: string;
		};
		item: {
			name: string;
			baseType: string;
			typeLine: string;
			ilvl: number;
			rarity: string;
			frameType: number;
			icon: string;
			corrupted?: boolean;
			properties?: {
				name: string;
				values: [string, number][];
				displayMode: number;
				type?: number;
			}[];
			requirements?: {
				name: string;
				values: [string, number][];
				displayMode: number;
				type?: number;
			}[];
			implicitMods?: string[];
			explicitMods?: string[];
			runeMods?: string[];
			sockets?: {
				group: number;
				socketType: string;
				item?: string;
			}[];
			extended?: {
				dps?: number;
				pdps?: number;
				edps?: number;
				dpsAugmented?: boolean;
				pdpsAugmented?: boolean;
				mods?: {
					explicit?: {
						name: string;
						tier: string;
						level: number;
						magnitudes: {
							hash: string;
							min: string;
							max: string;
						}[];
					}[];
					implicit?: {
						name: string;
						tier: string;
						level: number;
						magnitudes: {
							hash: string;
							min: string;
							max: string;
						}[];
					}[];
				};
			};
		};
	}[] = [];

	let container: HTMLElement;

	function getRelativeTime(date: string | undefined): string {
		if (!date) return '';
		const now = new Date();
		const past = new Date(date);
		const diffMs = now.getTime() - past.getTime();
		const diffSecs = Math.round(diffMs / 1000);
		const diffMins = Math.round(diffSecs / 60);
		const diffHours = Math.round(diffMins / 60);
		const diffDays = Math.round(diffHours / 24);

		if (diffSecs < 60) return `${diffSecs}s ago`;
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		return `${diffDays}d ago`;
	}

	function getRarityClass(frameType: number): string {
		switch (frameType) {
			case 0:
				return 'text-white'; // Normal
			case 1:
				return 'text-blue-400'; // Magic
			case 2:
				return 'text-yellow-400'; // Rare
			case 3:
				return 'text-orange-400'; // Unique
			case 5:
				return 'text-primary'; // Special (e.g. Runes)
			default:
				return 'text-white';
		}
	}

	function formatModTier(tier: string): string {
		if (tier.startsWith('P')) return `T${parseInt(tier.slice(1))}`;
		if (tier.startsWith('S')) return `T${parseInt(tier.slice(1))}`;
		return tier;
	}

	function formatBracketedText(text: string): string {
		return text.replace(/\[(.*?)\]/g, (match, contents) => {
			const options = contents.split('|');
			return options[options.length - 1];
		});
	}

	function positionPopover(event: MouseEvent) {
		const target = event.currentTarget as HTMLElement;
		const popover = target.querySelector('.item-details-popover') as HTMLElement;
		if (!popover || !container) return;

		const rect = target.getBoundingClientRect();
		const popoverHeight = popover.offsetHeight;
		const windowHeight = window.innerHeight;
		const spaceBelow = windowHeight - rect.bottom;

		// If there's not enough space below and more space above, show above
		if (spaceBelow < popoverHeight && rect.top > popoverHeight) {
			popover.style.top = 'auto';
			popover.style.bottom = '100%';
			popover.style.marginTop = '0';
			popover.style.marginBottom = '4px';
		} else {
			popover.style.top = '100%';
			popover.style.bottom = 'auto';
			popover.style.marginTop = '4px';
			popover.style.marginBottom = '0';
		}
	}
</script>

<div class="mt-2 p-1 bg-surface-dark rounded-lg border border-border relative" bind:this={container}>
	{#if results.length > 0}
		<div class="space-y-1">
			{#each results as result}
				<div
					class="group relative p-1 bg-surface rounded shadow border border-border flex items-center gap-2 hover:bg-surface/80"
					on:mouseenter={positionPopover}
					role="button"
					tabindex="0"
				>
					<div class="font-semibold text-primary whitespace-nowrap">
						{result.listing?.price?.amount}
						{result.listing?.price?.currency}
						{#if result.listing?.price?.currency === 'chaos'}
							<span class="text-text-muted text-sm font-normal"
								>(≈{(result.listing?.price?.amount / 160).toFixed(1)}div)</span
							>
						{/if}
					</div>
					<div class="text-text-muted text-sm whitespace-nowrap">
						{getRelativeTime(result.listing?.indexed)}
					</div>
					<div class="text-text flex items-center gap-2 ml-auto">
						<span>{result.listing?.account?.name}</span>
						{#if result.listing?.account?.online}
							<span
								class="text-xs text-primary bg-primary/10 px-1.5 py-0.5 rounded-full flex items-center gap-1"
							>
								Online
								{#if result.listing?.account?.online?.status}
									<span class="text-text-muted">({result.listing.account.online.status})</span>
								{/if}
							</span>
						{/if}
					</div>

					<!-- Item Details Popover -->
					<div
						class="invisible group-hover:visible absolute left-1/2 -translate-x-1/2 item-details-popover w-[80%] p-3 bg-surface-dark rounded-lg shadow-lg border border-border z-10"
					>
						<div class="space-y-0.5 text-sm">
							<div class="flex items-start gap-3">
								<img
									src={result.item.icon}
									alt={result.item.name}
									class="w-12 h-12 object-contain bg-black/50 rounded"
								/>
								<div class="flex flex-col">
									<span class={getRarityClass(result.item.frameType)}>{result.item.name}</span>
									<span class="text-text-muted text-sm">{result.item.baseType}</span>
									<span class="text-text-muted text-sm">Item Level: {result.item.ilvl}</span>
									{#if result.item.corrupted}
										<span class="text-red-500 text-sm">Corrupted</span>
									{/if}
								</div>
							</div>

							{#if result.item.properties?.length}
								<div class="border-t border-border py-1">
									{#each result.item.properties as prop}
										<div class="text-text-muted">
											{formatBracketedText(prop.name)}
											{#if prop.values.length > 0}
												:
												{#each prop.values as [value]}
													<span class="text-text">{value}</span>
												{/each}
											{/if}
										</div>
									{/each}
								</div>
								{#if result.item.extended?.dps}
									<div class="border-t border-border py-1">
										<div class="text-text-muted">
											Total DPS: <span class="text-text">{result.item.extended.dps.toFixed(1)}</span
											>
											{#if result.item.extended.dpsAugmented}
												<span class="text-primary text-xs ml-1">augmented</span>
											{/if}
										</div>
										{#if result.item.extended.pdps}
											<div class="text-text-muted">
												Physical DPS: <span class="text-text"
													>{result.item.extended.pdps.toFixed(1)}</span
												>
												{#if result.item.extended.pdpsAugmented}
													<span class="text-primary text-xs ml-1">augmented</span>
												{/if}
											</div>
										{/if}
										{#if result.item.extended.edps}
											<div class="text-text-muted">
												Elemental DPS: <span class="text-text"
													>{result.item.extended.edps.toFixed(1)}</span
												>
											</div>
										{/if}
									</div>
								{/if}
							{/if}

							{#if result.item.requirements?.length}
								<div class="border-t border-border py-1">
									{#each result.item.requirements as req}
										<div class="text-text-muted">
											Required {formatBracketedText(req.name)}
											{#if req.values.length > 0}
												:
												{#each req.values as [value]}
													<span class="text-text">{value}</span>
												{/each}
											{/if}
										</div>
									{/each}
								</div>
							{/if}

							{#if result.item.implicitMods?.length}
								<div class="border-t border-border py-1">
									{#each result.item.implicitMods as mod}
										<div class="text-blue-400">{formatBracketedText(mod)}</div>
									{/each}
								</div>
							{/if}

							{#if result.item.runeMods?.length}
								<div class="border-t border-border py-1">
									{#each result.item.runeMods as mod}
										<div class="text-primary">{formatBracketedText(mod)}</div>
									{/each}
								</div>
							{/if}

							{#if result.item.explicitMods?.length}
								<div class="border-t border-border py-1">
									{#each result.item.explicitMods as mod, i}
										<div class="text-text flex items-baseline gap-2">
											{#if result.item.extended?.mods?.explicit?.[i]}
												<span class="text-xs text-text-muted">
													{formatModTier(result.item.extended.mods.explicit[i].tier)}
												</span>
											{/if}
											<span>{formatBracketedText(mod)}</span>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<p class="text-text-muted italic">No items found matching your search.</p>
	{/if}
</div>

<style>
	/* Ensure parent container has relative positioning for proper popover containment */
	:global(.group) {
		position: relative;
	}

	/* Ensure popover stays within viewport */
	:global(.group:hover > .item-details-popover) {
		visibility: visible;
	}
</style>
