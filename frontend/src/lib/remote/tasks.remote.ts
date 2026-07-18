import { command, form, getRequestEvent } from '$app/server';
import { Task, UpdateTask, type TaskError } from '$lib/tasks/types';
import { error, fail } from '@sveltejs/kit';
import z from 'zod';

type FormError = { error: TaskError };

export const updateTask = form(
	z.object({ id: z.int(), fields: UpdateTask }),
	async ({ id, fields }) => {
		const res = await fetch(`/api/tasks/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(fields)
		});

		if (!res.ok) {
			const e = await res.text();
			return fail<FormError>(res.status, {
				error: { type: 'http_error', status: res.status, message: e }
			});
		}

		const updated = Task.parse(await res.json());

		return {
			task: updated
		};
	}
);

export const toggleTask = command(z.int(), async (id) => {
	const { fetch } = getRequestEvent();

	const res = await fetch(`/api/tasks/${id}/toggle`, {
		method: 'PUT'
	});

	if (!res.ok) {
		error(res.status, res.status === 404 ? 'Not found!' : await res.text());
	}
});

export const archiveCompleted = form(async () => {
	const { fetch } = getRequestEvent();
	console.log('working');

	const res = await fetch(`/api/tasks/archive-completed`, {
		method: 'POST'
	});

	if (!res.ok) {
		error(res.status, res.status === 404 ? 'Not found!' : await res.text());
	}
});
