<script lang="ts">
	import type { Task } from '$lib/tasks/types';

	interface Props {
		task: Task;
	}

	const { task }: Props = $props();
</script>

<div class="flex w-max card preset-outlined">
	<input class="checkbox" type="checkbox" checked={task.completed} />
	<div>{task.priority}</div>
	<div>
		<p>{task.name}</p>
		<p>{task.description}</p>
		<p>fällig bis {task.deadline?.toString()}</p>
	</div>
</div>
