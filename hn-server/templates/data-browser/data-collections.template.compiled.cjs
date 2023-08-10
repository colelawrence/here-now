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

// general each functions:

function ensure_array_like(array_like_or_iterator) {
	return array_like_or_iterator?.length !== undefined
		? array_like_or_iterator
		: Array.from(array_like_or_iterator);
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

/** @returns {string} */
function each(items, fn) {
	items = ensure_array_like(items);
	let str = '';
	for (let i = 0; i < items.length; i += 1) {
		str += fn(items[i], i);
	}
	return str;
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

function sanitizeHTML(html) {
  return html.replace(/</g, "&lt;").replace(/&lt;(\/?(?:code|u|b|em|strong|ul|li)>)/g, "<$1");
}

/* hn-server/templates/data-browser/Header.svelte generated by Svelte v4.1.1 */

const css$1 = {
	code: ":root{font-family:system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Oxygen, Ubuntu, Cantarell,\n      \"Open Sans\", \"Helvetica Neue\", sans-serif;max-width:480px;margin:2rem auto}a{color:dodgerblue;text-decoration:none}a[href*=\"/creds#\"], a[href*=\"#cred_\"]{color:oklch(0.1 0.5 30);background-color:oklch(0.9 0.1 30)}a[href*=\"/devices#\"], a[href*=\"#web_\"]{color:oklch(0.1 0.5 60);background-color:oklch(0.9 0.1 60)}a:hover{text-decoration:underline}.links.svelte-1scqkfu a.svelte-1scqkfu{font-size:1rem;padding:0.25rem 0.5rem;border-radius:4px;background-color:#f5f5f5;border:1px solid transparent}.links.svelte-1scqkfu.svelte-1scqkfu{display:flex;flex-direction:row;gap:1rem}.warning-flash.svelte-1scqkfu.svelte-1scqkfu{background-color:#fffae6;border:1px solid #ffeb9c;border-radius:4px;padding:1rem;margin-bottom:1rem}a.title-link.svelte-1scqkfu.svelte-1scqkfu{color:inherit;text-decoration:none}h1.svelte-1scqkfu.svelte-1scqkfu{position:relative;display:flex;flex-direction:column;gap:0.5rem}",
	map: "{\"version\":3,\"file\":\"Header.svelte\",\"sources\":[\"Header.svelte\"],\"sourcesContent\":[\"<script lang=\\\"ts\\\">import { sanitizeHTML } from \\\"../sanitizeHTML\\\";\\nexport let header = { title: \\\"Data Collections\\\", links: [] };\\n</script>\\n\\n<h1>\\n  <div class=\\\"links\\\">\\n    {#each header.links as [collection_label, href]}\\n      <a {href}>{collection_label}</a>\\n    {/each}\\n  </div>\\n  <a href=\\\"/data\\\" class=\\\"title-link\\\">{header.title}</a>\\n</h1>\\n\\n{#if header.warning}\\n  <div class=\\\"warning-flash\\\">\\n    {@html sanitizeHTML(header.warning)}\\n  </div>\\n{/if}\\n\\n<style>\\n  :root {\\n    font-family: system-ui, -apple-system, BlinkMacSystemFont, \\\"Segoe UI\\\", Roboto, Oxygen, Ubuntu, Cantarell,\\n      \\\"Open Sans\\\", \\\"Helvetica Neue\\\", sans-serif;\\n    max-width: 480px;\\n    margin: 2rem auto;\\n  }\\n  :global(a) {\\n    color: dodgerblue;\\n    text-decoration: none;\\n  }\\n  :global(a[href*=\\\"/creds#\\\"], a[href*=\\\"#cred_\\\"]) {\\n    color: oklch(0.1 0.5 30);\\n    background-color: oklch(0.9 0.1 30);\\n  }\\n  :global(a[href*=\\\"/devices#\\\"], a[href*=\\\"#web_\\\"]) {\\n    color: oklch(0.1 0.5 60);\\n    background-color: oklch(0.9 0.1 60);\\n  }\\n  :global(a:hover) {\\n    text-decoration: underline;\\n  }\\n\\n  .links a {\\n    font-size: 1rem;\\n    padding: 0.25rem 0.5rem;\\n    border-radius: 4px;\\n    background-color: #f5f5f5;\\n    border: 1px solid transparent;\\n  }\\n\\n  .links {\\n    display: flex;\\n    flex-direction: row;\\n    gap: 1rem;\\n  }\\n\\n\\n  .warning-flash {\\n    background-color: #fffae6;\\n    border: 1px solid #ffeb9c;\\n    border-radius: 4px;\\n    padding: 1rem;\\n    margin-bottom: 1rem;\\n  }\\n\\n  a.title-link {\\n    color: inherit;\\n    text-decoration: none;\\n  }\\n\\n  h1 {\\n    position: relative;\\n    display: flex;\\n    flex-direction: column;\\n    gap: 0.5rem;\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAoBE,KAAM,CACJ,WAAW,CAAE,SAAS,CAAC,CAAC,aAAa,CAAC,CAAC,kBAAkB,CAAC,CAAC,UAAU,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,SAAS,CAAC;AAC7G,MAAM,WAAW,CAAC,CAAC,gBAAgB,CAAC,CAAC,UAAU,CAC3C,SAAS,CAAE,KAAK,CAChB,MAAM,CAAE,IAAI,CAAC,IACf,CACQ,CAAG,CACT,KAAK,CAAE,UAAU,CACjB,eAAe,CAAE,IACnB,CACQ,qCAAuC,CAC7C,KAAK,CAAE,MAAM,GAAG,CAAC,GAAG,CAAC,EAAE,CAAC,CACxB,gBAAgB,CAAE,MAAM,GAAG,CAAC,GAAG,CAAC,EAAE,CACpC,CACQ,sCAAwC,CAC9C,KAAK,CAAE,MAAM,GAAG,CAAC,GAAG,CAAC,EAAE,CAAC,CACxB,gBAAgB,CAAE,MAAM,GAAG,CAAC,GAAG,CAAC,EAAE,CACpC,CACQ,OAAS,CACf,eAAe,CAAE,SACnB,CAEA,qBAAM,CAAC,gBAAE,CACP,SAAS,CAAE,IAAI,CACf,OAAO,CAAE,OAAO,CAAC,MAAM,CACvB,aAAa,CAAE,GAAG,CAClB,gBAAgB,CAAE,OAAO,CACzB,MAAM,CAAE,GAAG,CAAC,KAAK,CAAC,WACpB,CAEA,oCAAO,CACL,OAAO,CAAE,IAAI,CACb,cAAc,CAAE,GAAG,CACnB,GAAG,CAAE,IACP,CAGA,4CAAe,CACb,gBAAgB,CAAE,OAAO,CACzB,MAAM,CAAE,GAAG,CAAC,KAAK,CAAC,OAAO,CACzB,aAAa,CAAE,GAAG,CAClB,OAAO,CAAE,IAAI,CACb,aAAa,CAAE,IACjB,CAEA,CAAC,yCAAY,CACX,KAAK,CAAE,OAAO,CACd,eAAe,CAAE,IACnB,CAEA,gCAAG,CACD,QAAQ,CAAE,QAAQ,CAClB,OAAO,CAAE,IAAI,CACb,cAAc,CAAE,MAAM,CACtB,GAAG,CAAE,MACP\"}"
};

const Header = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { header = { title: "Data Collections", links: [] } } = $$props;
	if ($$props.header === void 0 && $$bindings.header && header !== void 0) $$bindings.header(header);
	$$result.css.add(css$1);

	return `<h1 class="svelte-1scqkfu"><div class="links svelte-1scqkfu">${each(header.links, ([collection_label, href]) => {
		return `<a${add_attribute("href", href, 0)} class="svelte-1scqkfu">${escape(collection_label)}</a>`;
	})}</div> <a href="/data" class="title-link svelte-1scqkfu">${escape(header.title)}</a></h1> ${header.warning
	? `<div class="warning-flash svelte-1scqkfu">${sanitizeHTML(header.warning)}</div>`
	: ``}`;
});

/* hn-server/templates/data-browser/data-collections.template.svelte generated by Svelte v4.1.1 */

const css = {
	code: ".collections.svelte-1lx5yk8{display:flex;flex-direction:column}",
	map: "{\"version\":3,\"file\":\"data-collections.template.svelte\",\"sources\":[\"data-collections.template.svelte\"],\"sourcesContent\":[\"<script lang=\\\"ts\\\">import Header from \\\"./Header.svelte\\\";\\nexport let header = { title: \\\"Data Collections\\\" };\\nexport let collection_label_href = [[\\\"Things\\\", \\\"/data/things\\\"]];\\n</script>\\n\\n<Header header={header} />\\n\\n<div class=\\\"collections\\\">\\n  {#each collection_label_href as [collection_label, href]}\\n    <a {href}>{collection_label}</a>\\n  {/each}\\n</div>\\n\\n<style>\\n  .collections {\\n    display: flex;\\n    flex-direction: column;\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAcE,2BAAa,CACX,OAAO,CAAE,IAAI,CACb,cAAc,CAAE,MAClB\"}"
};

const Data_collections_template = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { header = { title: "Data Collections" } } = $$props;
	let { collection_label_href = [["Things", "/data/things"]] } = $$props;
	if ($$props.header === void 0 && $$bindings.header && header !== void 0) $$bindings.header(header);
	if ($$props.collection_label_href === void 0 && $$bindings.collection_label_href && collection_label_href !== void 0) $$bindings.collection_label_href(collection_label_href);
	$$result.css.add(css);

	return `${validate_component(Header, "Header").$$render($$result, { header }, {}, {})} <div class="collections svelte-1lx5yk8">${each(collection_label_href, ([collection_label, href]) => {
		return `<a${add_attribute("href", href, 0)}>${escape(collection_label)}</a>`;
	})} </div>`;
});

module.exports = Data_collections_template;
