<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async ({ fetch, params }) => {
		const res = await fetch(
			`https://api.themoviedb.org/3/search/movie?api_key=${
				import.meta.env.VITE_API_KEY
			}&language=en-US&query=${params.id}&page=1&include_adult=false`
		);
		if (res.ok) {
			const data = await res.json();
			return {
				props: { searchedMovie: data.results }
			};
		}
	};
</script>

<script lang="ts">
	export let searchedMovie: any;
	import MovieCard from '../../components/MovieCard.svelte';
</script>

<div class="searched-movies">
	{#each searchedMovie as movie}
		<MovieCard {movie} />
	{/each}
</div>

<style>
	.searched-movies {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		grid-column-gap: 1rem;
		grid-row-gap: 2rem;
		height: 20vh;
	}
</style>
