import { error, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { auth } from '$lib/server/auth';
import { APIError } from 'better-auth';

export const load: PageServerLoad = async (event) => {
	if (!event.locals.user) {
		return redirect(302, '/auth/signin');
	}

	let token = '';
	try {
		token = (await auth.api.getToken({ headers: event.request.headers })).token;
	} catch (err) {
		if (err instanceof APIError) {
			error(401, { message: 'Unauthenticated' });
		}
	}

	return { user: event.locals.user, token };
};

export const actions: Actions = {
	signOut: async (event) => {
		await auth.api.signOut({ headers: event.request.headers });
		return redirect(302, '/auth/signin');
	}
};
