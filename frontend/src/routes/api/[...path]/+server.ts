import { env } from '$env/dynamic/private';
import { auth } from '$lib/server/auth';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const proxy: RequestHandler = async ({ request, url }) => {
	const token = (await auth.api.getToken({ headers: request.headers })).token;

	const path =
		url.pathname === '/' ? url.pathname : url.pathname.replace('/api', '').replace(/\/$/, '');
	const targetUrl = `${env.API_URL}${path}${url.search}`;
	const body = request.method !== 'GET' ? await request.text() : undefined;

	const response = await fetch(targetUrl, {
		method: request.method,
		headers: {
			...Object.fromEntries(request.headers.entries()),
			Authorization: `Bearer ${token}`
		},
		body: body
	});

	const data = await response.json();
	console.log(data);
	return json(data, { status: response.status });
};

// handle all HTTP methods
export const GET = proxy;
export const POST = proxy;
export const PUT = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
export const OPTIONS = proxy;
