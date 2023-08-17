<script lang="ts">
	import { iam } from '$lib';
	import UiInputType from './UIInputType.svelte';
	export let item: iam.UIItem = iam.UIItem.INPUT({
		key: 'discord-client-id',
		label: 'Application ID',
		type: iam.UIInputType.TEXT({
			examples: ['1132773161985908787', '1024467966416388126'],
			format_description: `Discord's "Application ID" or also called "Client ID".`
		})
	});
</script>

{#if 'INPUT' in item}
	<UiInputType type={item.INPUT.type} />
{:else if 'CONTENT' in item}
	CONTENT
	<pre>{JSON.stringify(item)}</pre>
{:else if 'WARNING' in item}
	WARNING
	<pre>{JSON.stringify(item)}</pre>
{:else}
	Unknown
	<pre>{JSON.stringify(item)}</pre>
{/if}
