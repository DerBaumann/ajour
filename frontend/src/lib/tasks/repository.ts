import { API_URL } from '$env/static/private';
import type { Task } from './types';

export async function fetchAllTasks(): Promise<Task[]> {
	const res = await fetch(`${API_URL}/tasks`);
	const tasks = await res.json();
	return tasks;
}
