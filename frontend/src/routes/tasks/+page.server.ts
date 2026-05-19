import { fetchAllTasks } from '$lib/tasks/repository';

export async function load() {
	return {
		tasks: await fetchAllTasks()
	};
}
