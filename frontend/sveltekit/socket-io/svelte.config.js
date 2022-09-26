import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';

import { Server } from 'socket.io';

export const webSocketServer = {
	name: 'webSocketServer',
	configureServer(server) {
		const io = new Server(server.httpServer);

		io.on('connect', (socket) => {
			socket.emit('eventFromServer', 'Hello, World');
		});
	}
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		vite: {
			plugins: [webSocketServer]
		}
	}
};

export default config;
