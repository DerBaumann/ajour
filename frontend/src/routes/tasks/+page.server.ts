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
