<script lang="ts">
	import { resolve } from '$app/paths';
	import TaskItem from '$lib/components/TaskItem.svelte';

	let { data } = $props();
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<a href={resolve('/tasks/create')} class="btn preset-filled-secondary-500">Neue Aufgabe</a>
	</div>

	<ul class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</ul>
</main>
