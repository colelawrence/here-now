<script lang="ts">
	import { example_offer, iam } from '$lib';
	import OutIdentify from '$lib/OutIdentify.svelte';
	import OutOffer from '$lib/OutOffer.svelte';
	export let current: iam.Out[] = [
		{
			IDENTIFY: {
				title: 'Discord setup'
			}
		},
		{
			OFFER: example_offer
		}
	];
</script>

{#each current as item}
	{#if 'IDENTIFY' in item}
		<OutIdentify IDENTIFY={item.IDENTIFY} />
	{:else if 'OFFER' in item}
		<OutOffer OFFER={item.OFFER} />
	{:else}
		Unknown item
	{/if}
	<br />
{/each}

<style>
	pre {
		padding: 1em;
		margin: 0;
	}
	pre:nth-child(odd) {
		background-color: #eee;
	}
	:global(html) {
		font-family: Arial, Helvetica, sans-serif;
	}
</style>
