import type { RequestHandler } from '@sveltejs/kit';
import { api } from './_api';

export const del: RequestHandler = (request) => {
	return api(request);
};

export const patch: RequestHandler<Request, FormData> = (request) => {
	return api(request, {
		text: request.body.has('text') ? request.body.get('text') : undefined,
		done: request.body.has('done') ? !!request.body.get('done') : undefined
	});
};
