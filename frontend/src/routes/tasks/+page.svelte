<script lang="ts">
	import TaskAddForm from '$lib/components/TaskAddForm.svelte';
	import TaskItem from '$lib/components/TaskItem.svelte';
	import { archiveCompleted } from '$lib/remote/tasks.remote';
	import type { PageProps } from './$types';

	let { data, form }: PageProps = $props();
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar" class="flex">
		<TaskAddForm error={form?.error} />

		<!-- TODO: Confirmation dialog -->
		<form {...archiveCompleted}>
			<button type="submit" class="btn preset-filled-error-500">Archive Completed</button>
		</form>
	</div>

	<div class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</div>
</main>
