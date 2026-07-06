<script lang="ts">
	import TaskAddForm from '$lib/components/TaskAddForm.svelte';
	import TaskItem from '$lib/components/TaskItem.svelte';
	import type { PageProps } from './$types';

	let { data, form }: PageProps = $props();
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<TaskAddForm error={form?.error} />
	</div>

	<div class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</div>
</main>
