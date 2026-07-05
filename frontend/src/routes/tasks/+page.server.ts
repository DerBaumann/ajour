import { CreateTask } from '$lib/tasks/types';
import { error, redirect } from '@sveltejs/kit';

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

export const actions = {
	// TODO: replace error with fail
	create: async ({ request, fetch }) => {
		const form = await request.formData();
		const data = Object.fromEntries(form.entries());
		console.log(data);

		const { data: task, error: e } = CreateTask.safeParse(data);
		if (e) {
			error(400, { message: e.message });
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
			return error(res.status, { message: e });
		}

		return {
			success: true
		};
	}
};
