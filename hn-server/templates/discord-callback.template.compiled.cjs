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

/* hn-server/templates/Header.svelte generated by Svelte v4.1.1 */

const css$1 = {
	code: ":root{font-family:system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Oxygen, Ubuntu, Cantarell,\n      \"Open Sans\", \"Helvetica Neue\", sans-serif;max-width:480px;margin:2rem auto}img.svelte-6fbmw5{height:1em}a.svelte-6fbmw5{color:inherit;text-decoration:none;padding-left:10px}h1.svelte-6fbmw5{position:relative}.wave.svelte-6fbmw5{position:absolute;right:100%}",
	map: "{\"version\":3,\"file\":\"Header.svelte\",\"sources\":[\"Header.svelte\"],\"sourcesContent\":[\"<h1><span class=\\\"wave\\\"><img src=\\\"/public/favicon.png\\\" alt=\\\"Ducky\\\"></span><a href=\\\"/\\\">Here Now</a></h1>\\n\\n<svelte:head>\\n  <title>Here Now</title>\\n</svelte:head>\\n\\n<style>\\n  :root {\\n    font-family: system-ui, -apple-system, BlinkMacSystemFont, \\\"Segoe UI\\\", Roboto, Oxygen, Ubuntu, Cantarell,\\n      \\\"Open Sans\\\", \\\"Helvetica Neue\\\", sans-serif;\\n    max-width: 480px;\\n    margin: 2rem auto;\\n  }\\n\\n  img {\\n    height: 1em;\\n  }\\n\\n  a {\\n    color: inherit;\\n    text-decoration: none;\\n    padding-left: 10px;\\n  }\\n\\n  h1 {\\n    position: relative;\\n  }\\n  .wave {\\n    position: absolute;\\n    right: 100%;\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAOE,KAAM,CACJ,WAAW,CAAE,SAAS,CAAC,CAAC,aAAa,CAAC,CAAC,kBAAkB,CAAC,CAAC,UAAU,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,SAAS,CAAC;AAC7G,MAAM,WAAW,CAAC,CAAC,gBAAgB,CAAC,CAAC,UAAU,CAC3C,SAAS,CAAE,KAAK,CAChB,MAAM,CAAE,IAAI,CAAC,IACf,CAEA,iBAAI,CACF,MAAM,CAAE,GACV,CAEA,eAAE,CACA,KAAK,CAAE,OAAO,CACd,eAAe,CAAE,IAAI,CACrB,YAAY,CAAE,IAChB,CAEA,gBAAG,CACD,QAAQ,CAAE,QACZ,CACA,mBAAM,CACJ,QAAQ,CAAE,QAAQ,CAClB,KAAK,CAAE,IACT\"}"
};

const Header = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	$$result.css.add(css$1);
	return `<h1 class="svelte-6fbmw5"><span class="wave svelte-6fbmw5"><img src="/public/favicon.png" alt="Ducky" class="svelte-6fbmw5"></span><a href="/" class="svelte-6fbmw5">Here Now</a></h1> ${($$result.head += `${($$result.title = `<title>Here Now</title>`, "")}`, "")}`;
});

/* hn-server/templates/discord-callback.template.svelte generated by Svelte v4.1.1 */

const css = {
	code: ".error.svelte-1ge268u{padding:1rem;background:rgba(255, 125, 125, 0.2)}.result.svelte-1ge268u{padding:1rem;background:rgba(125, 255, 125, 0.2)}",
	map: "{\"version\":3,\"file\":\"discord-callback.template.svelte\",\"sources\":[\"discord-callback.template.svelte\"],\"sourcesContent\":[\"<script lang=\\\"ts\\\">import Header from \\\"./Header.svelte\\\";\\nexport let query = null;\\nexport let text = \\\"Text\\\";\\n</script>\\n\\n<Header />\\n\\n{#if text}\\n<div class=\\\"result\\\">{text}</div>\\n{/if}\\n{#if query.error}\\n  <div class=\\\"error\\\">\\n    <b>{query.error}</b>\\n    {#if query.error_description}\\n      : {query.error_description}\\n    {/if}\\n  </div>\\n{/if}\\n\\n<style>\\n  .error {\\n    padding: 1rem;\\n    background: rgba(255, 125, 125, 0.2);\\n  }\\n  .result {\\n    padding: 1rem;\\n    background: rgba(125, 255, 125, 0.2);\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAoBE,qBAAO,CACL,OAAO,CAAE,IAAI,CACb,UAAU,CAAE,KAAK,GAAG,CAAC,CAAC,GAAG,CAAC,CAAC,GAAG,CAAC,CAAC,GAAG,CACrC,CACA,sBAAQ,CACN,OAAO,CAAE,IAAI,CACb,UAAU,CAAE,KAAK,GAAG,CAAC,CAAC,GAAG,CAAC,CAAC,GAAG,CAAC,CAAC,GAAG,CACrC\"}"
};

const Discord_callback_template = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { query = null } = $$props;
	let { text = "Text" } = $$props;
	if ($$props.query === void 0 && $$bindings.query && query !== void 0) $$bindings.query(query);
	if ($$props.text === void 0 && $$bindings.text && text !== void 0) $$bindings.text(text);
	$$result.css.add(css);

	return `${validate_component(Header, "Header").$$render($$result, {}, {}, {})} ${text
	? `<div class="result svelte-1ge268u">${escape(text)}</div>`
	: ``} ${query.error
	? `<div class="error svelte-1ge268u"><b>${escape(query.error)}</b> ${query.error_description
		? `: ${escape(query.error_description)}`
		: ``}</div>`
	: ``}`;
});

module.exports = Discord_callback_template;
