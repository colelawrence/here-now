<script>
	import { example_offer, iam } from '$lib';
	import UiItem from './UIItem.svelte';
	export let OFFER = example_offer;
</script>

<span class="title">{OFFER.title}</span>
<code>{OFFER.key}</code>
<br />
<b>Known params</b>
{#each OFFER.known_params as param}
	<UiItem item={param} />
{/each}

<style>
	.title {
		font-family: Arial, Helvetica, sans-serif;
		font-size: 1.5rem;
		font-weight: 600;
	}
</style>
