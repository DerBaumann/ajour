import { fetchCurrentTasks } from '$lib/tasks/repository';
import { error } from '@sveltejs/kit';

export async function load() {
	return {
		tasks: (await fetchCurrentTasks()).match(
			(tasks) => tasks,
			(e) => error(500, { message: e })
		)
	};
}
