<script lang="ts">
    import { fade, scale } from "svelte/transition";
    import { FeedbackStore } from "../stores";
    import FeedbackItem from "./FeedbackItem.svelte";

    // import { onMount, onDestroy } from "svelte";

    // let feedback = [];
    // const unsubscribe = FeedbackStore.subscribe((data) => (feedback = data));

    // onMount(() => {
    //     console.log("mounted");
    // });
    // onDestroy(() => {
    //     console.log("destroyed");
    //     unsubscribe();
    // });
</script>

{#each $FeedbackStore as fb (fb.id)}
    <div in:scale out:fade={{ duration: 500 }}>
        <FeedbackItem item={fb} />
    </div>
{/each}
