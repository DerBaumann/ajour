import { command, form, getRequestEvent } from '$app/server';
import { error } from '@sveltejs/kit';
import z from 'zod';

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
