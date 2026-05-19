<script lang="ts">
	import { resolve } from '$app/paths';

	let { data } = $props();
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<a href={resolve('/tasks/create')} class="btn preset-filled-secondary-500">Neue Aufgabe</a>
	</div>

	<!-- TODO: Fetch tasks from backend -->
	<ul>
		{#each data.tasks as task (task)}
			<li>
				<input class="checkbox" type="checkbox" />
				<span>{task.name}</span>
			</li>
		{/each}
	</ul>
</main>
