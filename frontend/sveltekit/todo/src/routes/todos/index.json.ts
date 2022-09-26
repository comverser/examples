import type { RequestHandler, Request } from '@sveltejs/kit';
import { api } from './_api';

export const get: RequestHandler = (request) => {
	return api(request);
};

export const post: RequestHandler<Request, FormData> = (request) => {
	return api(request, {
		created_at: new Date(),
		text: request.body.get('text'),
		done: false
	});
};
