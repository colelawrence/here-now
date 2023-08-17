<script lang="ts">
	import { iam } from '$lib';
	import UiInputTypeOfChoice from './UIInputTypeOfChoice.svelte';
	import UiInputTypeOfText from './UIInputTypeOfText.svelte';
	export let type: iam.UIInputType = iam.UIInputType.TEXT({
		examples: ['1132773161985908787', '1024467966416388126'],
		format_description: `Discord's "Application ID" or also called "Client ID".`
	});
</script>

{#if 'TEXT' in type}
	<UiInputTypeOfText TEXT={type.TEXT} />
{:else if 'CHOICE' in type}
	<UiInputTypeOfChoice CHOICE={type.CHOICE} />
{:else}
	<pre>unknown type
{JSON.stringify(type, null, 2)}</pre>
{/if}
