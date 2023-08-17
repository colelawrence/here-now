<script lang="ts">
	import { iam } from '$lib';
	export let TEXT = iam.UIInputType.TEXT.content({
		examples: ['1132773161985908787', '1024467966416388126'],
		format_description: `Discord's "Application ID" or also called "Client ID".`
	});

	const { examples, format_description } = TEXT;

	let value = '';
</script>

<label>
	<input type="text" bind:value />
	<br />
	{format_description}
</label>
{#if examples?.length}
	<p>
		For example
		{#each examples as example}
			<button on:click={() => (value = example)}>{example}</button>
		{/each}
	</p>
{/if}

<style>
	label {
		display: flex;
		flex-direction: column;
	}
</style>