import { RegistrationCreds } from '$lib/auth/types';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { APIError } from 'better-auth';
import { auth } from '$lib/server/auth';

export const load: PageServerLoad = (event) => {
	if (event.locals.user) {
		return redirect(302, '/auth/protected');
	}

	return {};
};

export const actions: Actions = {
	default: async ({ request }) => {
		const data = Object.fromEntries(await request.formData());
		const { data: registrationCreds, error } = RegistrationCreds.safeParse(data);
		if (error) {
			return fail(400, { message: error.message });
		}

		const { name, email, password } = registrationCreds;

		try {
			await auth.api.signUpEmail({
				body: {
					email,
					password,
					name,
					callbackURL: '/auth/verification-success'
				}
			});
		} catch (error) {
			if (error instanceof APIError) {
				return fail(400, { message: error.message || 'Registration failed' });
			}
			// return fail(500, { message: `Unexpected error: ${error.message}` });
			return fail(500, { message: 'Unexpected error' });
		}

		redirect(303, '/tasks');
	}
};
