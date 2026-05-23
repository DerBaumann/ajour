import { createTask } from '$lib/tasks/repository';
import { CreateTaskRequest } from '$lib/tasks/types.js';
import { error, fail, redirect } from '@sveltejs/kit';

export const actions = {
	default: async (event) => {
		const form = await event.request.formData();
		const data = Object.fromEntries(form.entries());

		const res = CreateTaskRequest.safeParse(data);
		if (!res.success) {
			return fail(400, { error: { name: res.error.name, message: res.error.message } });
		}

		(await createTask(res.data)).mapErr((e) => error(500, { message: e }));

		redirect(303, '/tasks');
	}
};
