import { redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = (event) => {
	if (event.locals.user) {
		return redirect(302, '/auth/protected');
	}

	return {};
};

export const actions: Actions = {
	default: async ({ request }) => {}
};
