import { RegistrationCreds } from '$lib/auth/types';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { authClient } from '$lib/auth-client';
import { APIError } from 'better-auth';

export const actions: Actions = {
	default: async ({ request }) => {
		const data = Object.fromEntries(await request.formData());
		const { data: registrationCreds, error } = RegistrationCreds.safeParse(data);
		if (error) {
			return fail(400, { message: error.message });
		}

		const { name, email, password } = registrationCreds;

		try {
			await authClient.signUp.email({ name, email, password });
		} catch (error) {
			if (error instanceof APIError) {
				return fail(400, { message: error.message || 'Registration failed' });
			}
			return fail(500, { message: 'Unexpected error' });
		}

		redirect(303, '/tasks');
	}
};
