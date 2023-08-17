<script lang="ts">
	import { iam } from '$lib';
	import UiInputType from './UIInputType.svelte';
	let INPUT = iam.UIItem.INPUT.content({
		key: 'discord-client-id',
		label: 'Application ID',
		type: iam.UIInputType.TEXT({
			examples: ['1132773161985908787', '1024467966416388126'],
			format_description: `Discord's "Application ID" or also called "Client ID".`
		})
	});
</script>

<span class="title">{INPUT.label}</span>
<code>{INPUT.key}</code>
<br />
<UiInputType type={INPUT.type} />
