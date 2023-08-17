<script lang="ts">
	import { iam } from '$lib';
	import UiInputType from './UIInputType.svelte';
	export let CHOICE = iam.UIInputType.CHOICE.content({
		choices: [
			iam.UIInputChoice({
				key: '1',
				label: 'item-1',
				inputs: [
					iam.UIInput({
						key: '1-1',
						label: 'item-1-1',
						type: iam.UIInputType.TEXT({
							examples: ['1-1-1', '1-1-2'],
							format_description: 'item-1-1'
						})
					}),
					iam.UIInput({
						key: '1-2',
						label: 'item-1-2',
						type: iam.UIInputType.TEXT({
							examples: ['1-2-1', '1-2-2'],
							format_description: 'item-1-2'
						})
					})
				]
			}),
			iam.UIInputChoice({
				key: '2',
				label: 'item-2',
				inputs: [
					iam.UIInput({
						key: '2-1',
						label: 'item-2-1',
						type: iam.UIInputType.TEXT({
							examples: ['2-1-1', '2-1-2'],
							format_description: 'item-2-1'
						})
					})
				]
			})
		]
	});

	let currentRadio: string = '';
</script>

<ul>
	{#each CHOICE.choices as c}
		<li>
			<label for={c.key}>
				<input type="radio" id={c.key} name={c.key} value={c.key} bind:group={currentRadio} />
				{c.label}</label
			>
			{#if c.inputs}
				<ul class={currentRadio !== c.key ? 'inactive' : null}>
					{#each c.inputs as i}
						<li>
							<UiInputType type={i.type} />
						</li>
					{/each}
				</ul>
			{/if}
		</li>
	{/each}
</ul>

<style>
	.inactive {
		color: rgb(128, 128, 128);
		display: none;
	}
  ul {
    padding: 1rem;
  }
</style>
