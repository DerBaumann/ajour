import { CreateTask, type TaskError } from '$lib/tasks/types';
import { error, fail, redirect } from '@sveltejs/kit';

export async function load({ locals, fetch }) {
	if (!locals.user) {
		return redirect(302, '/auth/signin');
	}

	const res = await fetch('/api/tasks/current');
	if (!res.ok) {
		const e = await res.text();
		error(500, { message: e });
	}
	const tasks = await res.json();

	return {
		user: locals.user,
		tasks
	};
}

type FormError = { error: TaskError };

export const actions = {
	create: async ({ request, fetch }) => {
		const form = await request.formData();
		const data = Object.fromEntries(form.entries());
		console.log(data);

		const { data: task, error: e } = CreateTask.safeParse(data);
		if (e) {
			return fail<FormError>(400, { error: { type: 'zod_error', issues: e.issues } });
		}

		console.log(task);

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
			return fail<FormError>(res.status, {
				error: { type: 'http_error', status: res.status, message: e }
			});
		}

		return {
			success: true
		};
	}
};
