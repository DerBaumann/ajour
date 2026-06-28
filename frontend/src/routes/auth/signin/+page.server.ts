import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { LoginCreds } from '$lib/auth/types';
import { auth } from '$lib/server/auth';
import { APIError } from 'better-auth';

export const load: PageServerLoad = (event) => {
	if (event.locals.user) {
		return redirect(302, '/auth/protected');
	}

	return {};
};

export const actions: Actions = {
	default: async ({ request }) => {
		const data = Object.fromEntries(await request.formData());
		const { data: loginCreds, error } = LoginCreds.safeParse(data);
		if (error) {
			return fail(400, { message: error.message });
		}

		const { email, password } = loginCreds;

		try {
			await auth.api.signInEmail({
				body: {
					email,
					password,
					callbackURL: '/auth/verification-success'
				}
			});
		} catch (error) {
			if (error instanceof APIError) {
				return fail(400, { message: error.message || 'Registration failed' });
			}
			return fail(500, { message: 'Unexpected error' });
		}

		redirect(303, '/tasks');
	}
};
