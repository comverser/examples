import type { Request } from '@sveltejs/kit';
import PrismaClient from '$lib/prisma';

const prisma = new PrismaClient();

export const api = async (request: Request, data?: Record<string, unknown>) => {
	let status = 500;
	let body = {};

	switch (request.method.toUpperCase()) {
		case 'GET':
			status = 200;
			body = await prisma.todo.findMany();
			break;
		case 'POST':
			status = 201;
			body = await prisma.todo.create({
				data: {
					created_at: data.created_at as Date,
					done: data.done as boolean,
					text: data.text as string
				}
			});
			break;
		case 'DELETE':
			status = 200;
			body = await prisma.todo.delete({
				where: {
					uid: request.params.uid
				}
			});
			break;
		case 'PATCH':
			status = 200;
			body = await prisma.todo.update({
				where: {
					uid: request.params.uid
				},
				data: {
					done: data.done,
					text: data.text
				}
			});
			break;

		default:
			break;
	}

	if (request.method.toUpperCase() !== 'GET' && request.headers.accept !== 'application/json') {
		return {
			status: 303,
			headers: {
				location: '/'
			}
		};
	}

	return {
		status,
		body
	};
};
