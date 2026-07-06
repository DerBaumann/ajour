<script lang="ts">
	import TaskAddForm from '$lib/components/TaskAddForm.svelte';
	import TaskItem from '$lib/components/TaskItem.svelte';

	let { data } = $props();
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<TaskAddForm />
	</div>

	<div class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</div>
</main>
