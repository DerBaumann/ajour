<script lang="ts">
	import { XIcon } from '@lucide/svelte';
	import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
	import type { Snippet } from 'svelte';

	const dialogAnimation = `transition transition-discrete opacity-0 translate-y-[100px]
      starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px]
      data-[state=open]:opacity-100 data-[state=open]:translate-y-0`;

	type Props = {
		open?: boolean;
		title: string;
		trigger: string | Snippet<[]>;
		content: Snippet<[]>;
		footer: Snippet<[]>;
	};

	let { open = $bindable(false), title, trigger, content, footer }: Props = $props();
</script>

<Dialog {open} onOpenChange={({ open: o }) => (open = o)}>
	<Dialog.Trigger class="btn preset-filled-secondary-500">
		{#if typeof trigger === 'string'}
			{trigger}
		{:else}
			{@render trigger()}
		{/if}
	</Dialog.Trigger>
	<Portal>
		<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
		<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
			<Dialog.Content
				class="w-full max-w-xl space-y-4 card bg-surface-100-900 p-4 shadow-xl {dialogAnimation}"
			>
				<header class="flex items-center justify-between">
					<Dialog.Title class="text-lg font-bold">{title}</Dialog.Title>
					<Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
						<XIcon class="size-4" />
					</Dialog.CloseTrigger>
				</header>
				<Dialog.Description>
					{@render content()}
				</Dialog.Description>
				<footer class="flex justify-end gap-2">
					<Dialog.CloseTrigger class="btn preset-tonal">Cancel</Dialog.CloseTrigger>
					{@render footer()}
				</footer>
			</Dialog.Content>
		</Dialog.Positioner>
	</Portal>
</Dialog>
