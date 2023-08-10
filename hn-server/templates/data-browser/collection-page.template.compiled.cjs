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

function tightJsonStringify(obj, replacer) {
  return JSON.stringify(obj, replacer, 2).replace(/^([\{\[])\n (\s+)/, "$1$2").replace(/(\n[ ]+[\{\[])\n\s+/g, "$1 ").replace(/\n\s*([\]\}])/g, " $1");
}

function devStringify(input, display = true) {
  try {
    if (typeof input === "string") {
      if (input[0] === "{" || input[0] === "[") {
        try {
          return devStringify(JSON.parse(input), display);
        } catch {
        }
      }
      return input;
    } else if (typeof input === "function") {
      return input.toString();
    } else {
      const replacer = (_key, value) => {
        try {
          if (value && value.toJSON === void 0) {
            if (value instanceof Error) {
              return {
                error: value.toString(),
                stack: value.stack ?? null,
                // @ts-ignore
                cause: value.cause ? replacer("cause", value.cause) : void 0
              };
            }
          }
        } catch {
        }
        return value;
      };
      const json = tightJsonStringify(input, replacer);
      return display ? cleanNewlinesAndStacks(json.replace(/(\\?")([^"]+)\1:/g, "$2:")) : json;
    }
  } catch (err) {
    return input?.name || String(input);
  }
}
function cleanNewlinesAndStacks(stack) {
  return stack.replace(/(\(|\sat )\/[^\)\s]+node_modules\//gm, "$1node_modules/").replace(/([^"]+?)"((?:\\.|[^\"])*)"/g, (_fullMatch, beforeQuote, inside) => {
    return beforeQuote + (inside ? `"${inside.split(/\\n/g).join("\n" + " ".repeat(beforeQuote.length))}"` : '""');
  });
}

function sanitizeHTML(html) {
  return html.replace(/</g, "&lt;").replace(/&lt;(\/?(?:code|u|b|em|strong|ul|li)>)/g, "<$1");
}

/* hn-server/templates/data-browser/Header.svelte generated by Svelte v4.1.1 */

const css$1 = {
	code: ":root{font-family:system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Oxygen, Ubuntu, Cantarell,\n      \"Open Sans\", \"Helvetica Neue\", sans-serif;max-width:480px;margin:2rem auto}a{color:dodgerblue;text-decoration:none}a:hover{text-decoration:underline}.links.svelte-1jmhjpm a.svelte-1jmhjpm{font-size:1rem;padding:0.25rem 0.5rem;border-radius:4px;background-color:#f5f5f5;border:1px solid transparent}.links.svelte-1jmhjpm.svelte-1jmhjpm{display:flex;flex-direction:row;gap:1rem}.warning-flash.svelte-1jmhjpm.svelte-1jmhjpm{background-color:#fffae6;border:1px solid #ffeb9c;border-radius:4px;padding:1rem;margin-bottom:1rem}a.title-link.svelte-1jmhjpm.svelte-1jmhjpm{color:inherit;text-decoration:none}h1.svelte-1jmhjpm.svelte-1jmhjpm{position:relative}",
	map: "{\"version\":3,\"file\":\"Header.svelte\",\"sources\":[\"Header.svelte\"],\"sourcesContent\":[\"<script lang=\\\"ts\\\">import { sanitizeHTML } from \\\"../sanitizeHTML\\\";\\nexport let header = { title: \\\"Data Collections\\\", links: [] };\\n</script>\\n\\n<h1>\\n  <a href=\\\"/data\\\" class=\\\"title-link\\\">{header.title}</a>\\n  <div class=\\\"links\\\">\\n    {#each header.links as [collection_label, href]}\\n      <a {href}>{collection_label}</a>\\n    {/each}\\n  </div>\\n</h1>\\n\\n{#if header.warning}\\n  <div class=\\\"warning-flash\\\">\\n    {@html sanitizeHTML(header.warning)}\\n  </div>\\n{/if}\\n\\n<style>\\n  :root {\\n    font-family: system-ui, -apple-system, BlinkMacSystemFont, \\\"Segoe UI\\\", Roboto, Oxygen, Ubuntu, Cantarell,\\n      \\\"Open Sans\\\", \\\"Helvetica Neue\\\", sans-serif;\\n    max-width: 480px;\\n    margin: 2rem auto;\\n  }\\n  :global(a) {\\n    color: dodgerblue;\\n    text-decoration: none;\\n  }\\n  :global(a:hover) {\\n    text-decoration: underline;\\n  }\\n\\n  .links a {\\n    font-size: 1rem;\\n    padding: 0.25rem 0.5rem;\\n    border-radius: 4px;\\n    background-color: #f5f5f5;\\n    border: 1px solid transparent;\\n  }\\n\\n  .links {\\n    display: flex;\\n    flex-direction: row;\\n    gap: 1rem;\\n  }\\n\\n\\n  .warning-flash {\\n    background-color: #fffae6;\\n    border: 1px solid #ffeb9c;\\n    border-radius: 4px;\\n    padding: 1rem;\\n    margin-bottom: 1rem;\\n  }\\n\\n  a.title-link {\\n    color: inherit;\\n    text-decoration: none;\\n  }\\n\\n  h1 {\\n    position: relative;\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAoBE,KAAM,CACJ,WAAW,CAAE,SAAS,CAAC,CAAC,aAAa,CAAC,CAAC,kBAAkB,CAAC,CAAC,UAAU,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,MAAM,CAAC,CAAC,SAAS,CAAC;AAC7G,MAAM,WAAW,CAAC,CAAC,gBAAgB,CAAC,CAAC,UAAU,CAC3C,SAAS,CAAE,KAAK,CAChB,MAAM,CAAE,IAAI,CAAC,IACf,CACQ,CAAG,CACT,KAAK,CAAE,UAAU,CACjB,eAAe,CAAE,IACnB,CACQ,OAAS,CACf,eAAe,CAAE,SACnB,CAEA,qBAAM,CAAC,gBAAE,CACP,SAAS,CAAE,IAAI,CACf,OAAO,CAAE,OAAO,CAAC,MAAM,CACvB,aAAa,CAAE,GAAG,CAClB,gBAAgB,CAAE,OAAO,CACzB,MAAM,CAAE,GAAG,CAAC,KAAK,CAAC,WACpB,CAEA,oCAAO,CACL,OAAO,CAAE,IAAI,CACb,cAAc,CAAE,GAAG,CACnB,GAAG,CAAE,IACP,CAGA,4CAAe,CACb,gBAAgB,CAAE,OAAO,CACzB,MAAM,CAAE,GAAG,CAAC,KAAK,CAAC,OAAO,CACzB,aAAa,CAAE,GAAG,CAClB,OAAO,CAAE,IAAI,CACb,aAAa,CAAE,IACjB,CAEA,CAAC,yCAAY,CACX,KAAK,CAAE,OAAO,CACd,eAAe,CAAE,IACnB,CAEA,gCAAG,CACD,QAAQ,CAAE,QACZ\"}"
};

const Header = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { header = { title: "Data Collections", links: [] } } = $$props;
	if ($$props.header === void 0 && $$bindings.header && header !== void 0) $$bindings.header(header);
	$$result.css.add(css$1);

	return `<h1 class="svelte-1jmhjpm"><a href="/data" class="title-link svelte-1jmhjpm">${escape(header.title)}</a> <div class="links svelte-1jmhjpm">${each(header.links, ([collection_label, href]) => {
		return `<a${add_attribute("href", href, 0)} class="svelte-1jmhjpm">${escape(collection_label)}</a>`;
	})}</div></h1> ${header.warning
	? `<div class="warning-flash svelte-1jmhjpm">${sanitizeHTML(header.warning)}</div>`
	: ``}`;
});

/* hn-server/templates/data-browser/collection-page.template.svelte generated by Svelte v4.1.1 */

const css = {
	code: ".rows.svelte-kwel3p.svelte-kwel3p{display:flex;flex-direction:column}.collection-row.svelte-kwel3p .title.svelte-kwel3p{font-weight:bold;margin-bottom:0.5rem;text-decoration:none}.collection-row.svelte-kwel3p.svelte-kwel3p{border-radius:4px;padding:1rem;margin-bottom:1rem}.collection-row.svelte-kwel3p.svelte-kwel3p:target{background-color:#fffae6;border:1px solid #ffeb9c}",
	map: "{\"version\":3,\"file\":\"collection-page.template.svelte\",\"sources\":[\"collection-page.template.svelte\"],\"sourcesContent\":[\"<script lang=\\\"ts\\\">import { devStringify } from \\\"../helpers/devstringify\\\";\\nimport { sanitizeHTML } from \\\"../sanitizeHTML\\\";\\nimport Header from \\\"./Header.svelte\\\";\\nexport let header = { title: \\\"Title\\\" };\\nexport let rows = [];\\nconst shorthand_lookup = {\\n    web: \\\"devices\\\",\\n    cred: \\\"creds\\\",\\n};\\n</script>\\n\\n<Header {header} />\\n\\n<div class=\\\"rows\\\">\\n  {#each rows as row}\\n    <div class=\\\"collection-row\\\" id={row.id}>\\n      <a href=\\\"#{row.id}\\\" class=\\\"title\\\">{row.id}</a>\\n      <pre>{@html sanitizeHTML(\\n          devStringify(row.content).replace(\\n            /(token:\\\\s*\\\"\\\\w{3})([^\\\"]+?)(\\\\w{3}\\\")/g,\\n            (_, start, secret, end) => start + secret.replace(/./g, \\\"*\\\") + end\\n          )\\n        ).replace(\\n          // replace things like cred_awhuhawduihaw with a link to the corresponding collection with target to the id\\n          /\\\"((\\\\w{2,})_\\\\w+)\\\"/g,\\n          (_, id, shorthand) => `<a href=\\\"/data/${shorthand_lookup[shorthand] ?? `${shorthand}s`}#${id}\\\">${id}</a>`\\n        )}</pre>\\n    </div>\\n  {/each}\\n</div>\\n\\n<style>\\n  .rows {\\n    display: flex;\\n    flex-direction: column;\\n  }\\n\\n  .collection-row .title {\\n    font-weight: bold;\\n    margin-bottom: 0.5rem;\\n    text-decoration: none;\\n  }\\n\\n  .collection-row {\\n    border-radius: 4px;\\n    padding: 1rem;\\n    margin-bottom: 1rem;\\n  }\\n\\n  .collection-row:target {\\n    background-color: #fffae6;\\n    border: 1px solid #ffeb9c;\\n  }\\n</style>\\n\"],\"names\":[],\"mappings\":\"AAgCE,iCAAM,CACJ,OAAO,CAAE,IAAI,CACb,cAAc,CAAE,MAClB,CAEA,6BAAe,CAAC,oBAAO,CACrB,WAAW,CAAE,IAAI,CACjB,aAAa,CAAE,MAAM,CACrB,eAAe,CAAE,IACnB,CAEA,2CAAgB,CACd,aAAa,CAAE,GAAG,CAClB,OAAO,CAAE,IAAI,CACb,aAAa,CAAE,IACjB,CAEA,2CAAe,OAAQ,CACrB,gBAAgB,CAAE,OAAO,CACzB,MAAM,CAAE,GAAG,CAAC,KAAK,CAAC,OACpB\"}"
};

const Collection_page_template = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let { header = { title: "Title" } } = $$props;
	let { rows = [] } = $$props;
	const shorthand_lookup = { web: "devices", cred: "creds" };
	if ($$props.header === void 0 && $$bindings.header && header !== void 0) $$bindings.header(header);
	if ($$props.rows === void 0 && $$bindings.rows && rows !== void 0) $$bindings.rows(rows);
	$$result.css.add(css);

	return `${validate_component(Header, "Header").$$render($$result, { header }, {}, {})} <div class="rows svelte-kwel3p">${each(rows, row => {
		return `<div class="collection-row svelte-kwel3p"${add_attribute("id", row.id, 0)}><a href="${"#" + escape(row.id, true)}" class="title svelte-kwel3p">${escape(row.id)}</a> <pre>${sanitizeHTML(devStringify(row.content).replace(/(token:\s*"\w{3})([^"]+?)(\w{3}")/g, (_, start, secret, end) => start + secret.replace(/./g, "*") + end)).replace(
			// replace things like cred_awhuhawduihaw with a link to the corresponding collection with target to the id
			/"((\w{2,})_\w+)"/g,
			(_, id, shorthand) => `<a href="/data/${shorthand_lookup[shorthand] ?? `${shorthand}s`}#${id}">${id}</a>`
		)}</pre> </div>`;
	})} </div>`;
});

module.exports = Collection_page_template;
