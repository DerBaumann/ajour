import { command, getRequestEvent } from '$app/server';
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
