<script lang="ts">
	import type { Task } from '$lib/tasks/types';
	import { Asterisk, CircleAlert, CircleQuestionMark, Clock4, Flame } from '@lucide/svelte';

	interface Props {
		task: Task;
	}

	const { task }: Props = $props();
</script>

<div class="flex w-max items-center gap-4 card bg-surface-100 p-4 shadow-sm dark:bg-surface-900">
	<input class="checkbox" type="checkbox" checked={task.completed} />
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
		<p class="text-surface-500">fällig bis {task.deadline?.toString()}</p>
	</div>
</div>
