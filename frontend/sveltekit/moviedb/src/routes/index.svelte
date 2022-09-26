<script context="module" lang="ts">
	export const load = async ({ fetch }) => {
		const res = await fetch(
			`https://api.themoviedb.org/3/movie/popular?api_key=${
				import.meta.env.VITE_API_KEY
			}&language=en-US&page=1`
		);
		const data = await res.json();
		if (res.ok) {
			return {
				props: { popular: data.results }
			};
		}
	};
</script>

<script lang="ts">
	import PopularMovies from '../components/PopularMovies.svelte';
	import SearchMovies from '../components/SearchMovies.svelte';
	export let popular: Array<any>;
	import '../global.css';
	import { fly } from 'svelte/transition';
</script>

<section in:fly={{ y: 50, duration: 500, delay: 500 }} out:fly={{ y: 50, duration: 500 }}>
	<SearchMovies />
	<PopularMovies {popular} />
</section>
