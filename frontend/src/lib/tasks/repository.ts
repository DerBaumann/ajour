import type { CreateTaskRequest, Task } from './types';
import { API_URL } from '$env/static/private';
import { err, ok, type Result } from 'neverthrow';

export async function fetchAllTasks(): Promise<Result<Task[], string>> {
	const res = await fetch(`${API_URL}/tasks`);
	if (!res.ok) {
		const e = await res.text();
		console.error(e);
		return err(e);
	}
	const tasks = await res.json();
	return ok(tasks);
}

export async function createTask(task: CreateTaskRequest): Promise<Result<Task, string>> {
	const res = await fetch(`${API_URL}/tasks`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(task)
	});
	if (!res.ok) {
		const e = await res.text();
		console.error(e);
		return err(e);
	}

	const body = await res.text();
	return ok(JSON.parse(body));
}
