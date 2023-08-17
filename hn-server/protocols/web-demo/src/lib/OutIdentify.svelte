<script>
	import { iam } from '$lib';
	export let IDENTIFY = iam.Out.IDENTIFY.content({
		title: 'Identify title'
	});
</script>

<span class="title">{IDENTIFY.title}</span>

<style>
	.title {
		font-family: Arial, Helvetica, sans-serif;
		font-size: 2em;
		font-weight: bold;
	}
</style>
