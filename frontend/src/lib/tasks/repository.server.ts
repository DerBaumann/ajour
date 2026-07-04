import type { CreateTask, Task } from './types';
import { err, ok, type Result } from 'neverthrow';

export async function fetchAllTasks(fetchFn: typeof fetch): Promise<Result<Task[], string>> {
	const res = await fetchFn('api/tasks');
	if (!res.ok) {
		const e = await res.text();
		return err(e);
	}
	const tasks = await res.json();
	return ok(tasks);
}

export async function fetchCurrentTasks(fetchFn: typeof fetch): Promise<Result<Task[], string>> {
	const res = await fetchFn('/api/tasks/current');
	if (!res.ok) {
		const e = await res.text();
		return err(e);
	}
	const tasks = await res.json();
	return ok(tasks);
}

export async function createTask(
	fetchFn: typeof fetch,
	task: CreateTask
): Promise<Result<Task, string>> {
	const res = await fetchFn('/api/tasks', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(task)
	});
	if (!res.ok) {
		const e = await res.text();
		return err(e);
	}

	const body = await res.text();
	return ok(JSON.parse(body));
}
