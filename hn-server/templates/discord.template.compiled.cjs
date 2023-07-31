'use strict';

/** @returns {void} */

function run(fn) {
	return fn();
}

function blank_object() {
	return Object.create(null);
}

/**
 * @param {Function[]} fns
 * @returns {void}
 */
function run_all(fns) {
	fns.forEach(run);
}

let current_component;

/** @returns {void} */
function set_current_component(component) {
	current_component = component;
}

const ATTR_REGEX = /[&"]/g;
const CONTENT_REGEX = /[&<]/g;

/**
 * Note: this method is performance sensitive and has been optimized
 * https://github.com/sveltejs/svelte/pull/5701
 * @param {unknown} value
 * @returns {string}
 */
function escape(value, is_attr = false) {
	const str = String(value);
	const pattern = is_attr ? ATTR_REGEX : CONTENT_REGEX;
	pattern.lastIndex = 0;
	let escaped = '';
	let last = 0;
	while (pattern.test(str)) {
		const i = pattern.lastIndex - 1;
		const ch = str[i];
		escaped += str.substring(last, i) + (ch === '&' ? '&amp;' : ch === '"' ? '&quot;' : '&lt;');
		last = i + 1;
	}
	return escaped + str.substring(last);
}

function validate_component(component, name) {
	if (!component || !component.$$render) {
		if (name === 'svelte:component') name += ' this={...}';
		throw new Error(
			`<${name}> is not a valid SSR component. You may need to review your build config to ensure that dependencies are compiled, rather than imported as pre-compiled modules. Otherwise you may need to fix a <${name}>.`
		);
	}
	return component;
}

let on_destroy;

/** @returns {{ render: (props?: {}, { $$slots, context }?: { $$slots?: {}; context?: Map<any, any>; }) => { html: any; css: { code: string; map: any; }; head: string; }; $$render: (result: any, props: any, bindings: any, slots: any, context: any) => any; }} */
function create_ssr_component(fn) {
	function $$render(result, props, bindings, slots, context) {
		const parent_component = current_component;
		const $$ = {
			on_destroy,
			context: new Map(context || (parent_component ? parent_component.$$.context : [])),
			// these will be immediately discarded
			on_mount: [],
			before_update: [],
			after_update: [],
			callbacks: blank_object()
		};
		set_current_component({ $$ });
		const html = fn(result, props, bindings, slots);
		set_current_component(parent_component);
		return html;
	}
	return {
		render: (props = {}, { $$slots = {}, context = new Map() } = {}) => {
			on_destroy = [];
			const result = { title: '', head: '', css: new Set() };
			const html = $$render(result, props, {}, $$slots, context);
			run_all(on_destroy);
			return {
				html,
				css: {
					code: Array.from(result.css)
						.map((css) => css.code)
						.join('\n'),
					map: null // TODO
				},
				head: result.title + result.head
			};
		},
		$$render
	};
}

/** @returns {string} */
function add_attribute(name, value, boolean) {
	if (value == null || (boolean && !value)) return '';
	const assignment = boolean && value === true ? '' : `="${escape(value, true)}"`;
	return ` ${name}${assignment}`;
}

/* hn-server/templates/Input.svelte generated by Svelte v4.1.1 */

const Input = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { name = "input" } = $$props;
	let { label = "Label" } = $$props;
	let { help = "Help text description" } = $$props;
	let { value = "value" } = $$props;
	let { type = "text" } = $$props;
	let { placeholer = "..." } = $$props;
	if ($$props.name === void 0 && $$bindings.name && name !== void 0) $$bindings.name(name);
	if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
	if ($$props.help === void 0 && $$bindings.help && help !== void 0) $$bindings.help(help);
	if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
	if ($$props.type === void 0 && $$bindings.type && type !== void 0) $$bindings.type(type);
	if ($$props.placeholer === void 0 && $$bindings.placeholer && placeholer !== void 0) $$bindings.placeholer(placeholer);

	return `<div class="flex flex-col justify-stretch"><label${add_attribute("for", name, 0)} class="block mb-2 text-sm font-medium text-sys-on-background">${escape(label)}</label> <input${add_attribute("type", type, 0)}${add_attribute("id", name, 0)}${add_attribute("name", name, 0)}${add_attribute("value", value, 0)}${add_attribute("aria-describedby", help && `${name}-helper-text-explanation`, 0)} class="bg-sys-surface border border-sys-on-secondary-container text-on-secondary-container text-mono-base rounded-sm focus:ring-sys-primary focus:border-sys-primary block w-full p-2"${add_attribute("placeholder", placeholer, 0)}> ${help
	? `<p${add_attribute("id", `${name}-helper-text-explanation`, 0)} class="mt-2 text-ui-sm text-sys-on-secondary-container text-opacity-70">${escape(help)}</p>`
	: ``}</div>`;
});

/* hn-server/templates/discord.template.svelte generated by Svelte v4.1.1 */

const Discord_template = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	return `<div class="flex flex-col justify-stretch">${validate_component(Input, "Input").$$render(
		$$result,
		{
			label: "API Key",
			value: "jaljawkdkljaw",
			name: "api_key"
		},
		{},
		{}
	)}</div>`;
});

module.exports = Discord_template;