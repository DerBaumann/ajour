import { CreateTask } from '$lib/tasks/types.js';
import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = ({ locals }) => {
	if (!locals.user) {
		return redirect(302, '/auth/signin');
	}

	return { user: locals.user };
};

export const actions = {
	default: async ({ request, fetch }) => {
		const form = await request.formData();
		const data = Object.fromEntries(form.entries());

		const { data: task, error: e } = CreateTask.safeParse(data);
		if (e) {
			error(400, { message: e.message });
		}

		const res = await fetch('/api/tasks', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(task)
		});

		if (!res.ok) {
			const e = await res.text();
			console.error(e);
			return error(res.status, { message: e });
		}

		redirect(303, '/tasks');
	}
};
