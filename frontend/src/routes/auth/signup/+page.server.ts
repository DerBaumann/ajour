import { RegistrationCreds } from '$lib/auth/types';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { authClient } from '$lib/auth-client';

export const actions: Actions = {
	default: async ({ request }) => {
		const data = Object.fromEntries(await request.formData());
		const { data: registrationCreds, error } = RegistrationCreds.safeParse(data);
		if (error) {
			return fail(400, { error: { name: error.name, message: error.message } });
		}

		const { name, email, password } = registrationCreds;

		await authClient.signUp.email({ name, email, password });

		redirect(303, '/tasks');
	}
};
