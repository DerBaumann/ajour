<script lang="ts">
	import { resolve } from '$app/paths';

	const tasks = ['Task 1', 'Task 2', 'Task 3', 'Task 4'];
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<a href={resolve('/tasks/create')} class="btn preset-filled-secondary-500">Neue Aufgabe</a>
	</div>

	<!-- TODO: Fetch tasks from backend -->
	<ul>
		{#each tasks as task (task)}
			<li>
				<input class="checkbox" type="checkbox" />
				<span>{task}</span>
			</li>
		{/each}
	</ul>
</main>
