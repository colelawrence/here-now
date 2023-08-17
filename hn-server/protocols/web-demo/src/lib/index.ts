// place files you want to import through the `$lib` alias in this folder.
import * as iam from '../../../protocol/is/iam.v0.gen.ts';
export { iam };

export const example_offer = iam.Out.OFFER.content({
	key: 'setup-discord',
	title: 'Setup Discord',
	known_params: [
		iam.UIItem.INPUT({
			key: 'discord-client-id',
			label: 'Application ID',
			type: iam.UIInputType.TEXT({
				examples: ['1132773161985908787', '1024467966416388126'],
				format_description: `Discord's "Application ID" or also called "Client ID".`
			})
		}),
		iam.UIItem.INPUT({
			key: 'discord-integration-kind',
			label: 'Integration Kind',
			type: iam.UIInputType.CHOICE({
				choices: [
					iam.UIInputChoice({
						key: 'bot',
						label: 'Bot',
						inputs: [
							iam.UIInput({
								key: 'discord-bot-token',
								label: 'Bot Token',
								type: {
									TEXT: {
										examples: ['NzEzMjc3MzE2MTk4NTkwODc4'],
										format_description: "Discord's Bot Token"
									}
								}
							})
						]
					}),
					iam.UIInputChoice({
						key: 'auth',
						label: 'Login Provider',
						inputs: [
							iam.UIInput({
								key: 'discord-callback-uri',
								label: 'Callback URI',
								type: {
									TEXT: {
										examples: ['https://example.com/login/discord/callback'],
										format_description:
											"Discord's Callback URI matching the callback in the Discord Developer Portal"
									}
								}
							})
						]
					})
				]
			})
		})
	]
});
