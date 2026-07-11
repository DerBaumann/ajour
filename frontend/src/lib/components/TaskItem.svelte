<script lang="ts">
	import { enhance } from '$app/forms';
	import { completeTask } from '$lib/remote/tasks.remote';
	import type { Task } from '$lib/tasks/types';
	import { Asterisk, CircleAlert, CircleQuestionMark, Clock4, Flame, XIcon } from '@lucide/svelte';
	import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';

	interface Props {
		task: Task;
	}

	const { task }: Props = $props();

	const animation =
		'transition transition-discrete opacity-0 translate-y-[100px] starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px] data-[state=open]:opacity-100 data-[state=open]:translate-y-0';

	let isDeleteDialogOpen = $state(false);
</script>

<div class="flex w-max items-center gap-4 card bg-surface-100 p-4 shadow-sm dark:bg-surface-900">
	<Dialog open={isDeleteDialogOpen} onOpenChange={({ open }) => (isDeleteDialogOpen = open)}>
		<Dialog.Trigger class="btn-icon preset-filled-error-500">
			<XIcon class="size-4" />
		</Dialog.Trigger>
		<Portal>
			<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
			<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
				<Dialog.Content
					class="w-full max-w-xl space-y-4 card bg-surface-100-900 p-4 shadow-xl {animation}"
				>
					<header class="flex items-center justify-between">
						<Dialog.Title class="text-lg font-bold">Delete Task "{task.name}"?</Dialog.Title>
						<Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
							<XIcon class="size-4" />
						</Dialog.CloseTrigger>
					</header>
					<footer class="flex justify-end gap-2">
						<form
							method="POST"
							action="?/delete"
							use:enhance={() =>
								async ({ result, update }) => {
									await update();
									if (result.type === 'success' || result.type === 'redirect') {
										isDeleteDialogOpen = false;
									}
								}}
						>
							<input type="hidden" name="id" value={task.id} />
							<Dialog.CloseTrigger class="btn preset-tonal">Cancel</Dialog.CloseTrigger>
							<button type="submit" class="btn preset-filled">Confirm</button>
						</form>
					</footer>
				</Dialog.Content>
			</Dialog.Positioner>
		</Portal>
	</Dialog>
	<input
		onclick={async () => await completeTask(task.id)}
		type="checkbox"
		checked={task.completed}
		class="checkbox"
	/>
	<div>
		{#if task.priority === 'very_high'}
			<Flame color="var(--color-red-500)" />
		{:else if task.priority === 'high'}
			<CircleAlert color="var(--color-orange-500)" />
		{:else if task.priority === 'medium'}
			<Asterisk color="var(--color-green-500)" />
		{:else if task.priority === 'low'}
			<Clock4 color="var(--color-sky-500)" />
		{:else}
			<CircleQuestionMark />
		{/if}
	</div>
	<div>
		<p><strong class="font-bold">{task.name}</strong></p>
		<p>{task.description}</p>
		{#if task.deadline}
			<p class="text-surface-500">fällig bis {task.deadline.toString()}</p>
		{/if}
	</div>
</div>
