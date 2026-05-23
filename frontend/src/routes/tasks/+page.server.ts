import { fetchAllTasks } from '$lib/tasks/repository';
import { error } from '@sveltejs/kit';

export async function load() {
	return {
		tasks: (await fetchAllTasks()).match(
			(tasks) => tasks,
			(e) => error(500, { message: e })
		)
	};
}
