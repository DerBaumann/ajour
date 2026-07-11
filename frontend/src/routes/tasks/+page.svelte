<script lang="ts">
	import TaskAddForm from '$lib/components/TaskAddForm.svelte';
	import TaskItem from '$lib/components/TaskItem.svelte';
	import { archiveCompleted } from '$lib/remote/tasks.remote';
	import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
	import type { PageProps } from './$types';
	import { XIcon } from '@lucide/svelte';

	let { data, form }: PageProps = $props();

	const animation =
		'transition transition-discrete opacity-0 translate-y-[100px] starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px] data-[state=open]:opacity-100 data-[state=open]:translate-y-0';
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar" class="flex">
		<TaskAddForm error={form?.error} />

		<Dialog>
			<Dialog.Trigger class="btn preset-filled-error-500">Archive completed</Dialog.Trigger>
			<Portal>
				<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
				<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
					<Dialog.Content
						class="w-full max-w-xl space-y-4 card bg-surface-100-900 p-4 shadow-xl {animation}"
					>
						<header class="flex items-center justify-between">
							<Dialog.Title class="text-lg font-bold">Archive all completed tasks?</Dialog.Title>
							<Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
								<XIcon class="size-4" />
							</Dialog.CloseTrigger>
						</header>
						<footer class="flex justify-end gap-2">
							<form {...archiveCompleted}>
								<Dialog.CloseTrigger type="submit" class="btn preset-filled-error-500"
									>Archive Completed</Dialog.CloseTrigger
								>
							</form>
							<Dialog.CloseTrigger class="btn preset-tonal">Cancel</Dialog.CloseTrigger>
						</footer>
					</Dialog.Content>
				</Dialog.Positioner>
			</Portal>
		</Dialog>
		<!-- TODO: Confirmation dialog -->
	</div>

	<div class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</div>
</main>
