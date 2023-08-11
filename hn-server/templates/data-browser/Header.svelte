<script lang="ts">
  import { sanitizeHTML } from "../sanitizeHTML";
  import { PageHeader } from "./templates";
  export let header: PageHeader = { title: "Data Collections", links: [] };
</script>

<svelte:head>
  <title>{header.title}</title>
  <link rel="shortcut icon" href="/duckyhn.png" type="image/png" />
</svelte:head>

<p class="dev-links">
  <a href="/"><img src="/duckyhn.png" type="image/png" /> Config</a>
  <a href="/dev/docs/hn_server/index.html" target="_blank">📦 Cargo Docs</a>
  <a href="/dev/traces/search?service=hn-server" target="_blank"
    ><img src="/dev/traces/static/jaeger-logo-ab11f618.svg" /> Traces</a
  >
  <a href="http://0.0.0.0:9000" target="_blank"><img src="http://0.0.0.0:9000/public/favicon.png" /> Public</a>
</p>

<h1>
  <div class="links">
    {#each header.links as [collection_label, href]}
      <a {href}>{collection_label}</a>
    {/each}
  </div>
  <a href="/data" class="title-link">{header.title}</a>
</h1>

{#if header.warning}
  <div class="warning-flash">
    {@html sanitizeHTML(header.warning)}
  </div>
{/if}

<style>
  :root {
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell,
      "Open Sans", "Helvetica Neue", sans-serif;
    max-width: 480px;
    margin: 0 auto 2rem;
  }
  :global(a) {
    color: dodgerblue;
    text-decoration: none;
  }
  :global(a[href*="/creds#"], a[href*="#cred_"]) {
    color: oklch(0.1 0.5 30);
    background-color: oklch(0.9 0.1 30);
  }
  :global(a[href*="/devices#"], a[href*="#web_"]) {
    color: oklch(0.1 0.5 60);
    background-color: oklch(0.9 0.1 60);
  }
  :global(a:hover) {
    text-decoration: underline;
  }

  .links a {
    font-size: 1rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    background-color: #f5f5f5;
    border: 1px solid transparent;
  }

  .links {
    display: flex;
    flex-direction: row;
    gap: 1rem;
  }

  .warning-flash {
    background-color: #fffae6;
    border: 1px solid #ffeb9c;
    border-radius: 4px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  a.title-link {
    color: inherit;
    text-decoration: none;
  }

  h1 {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .dev-links {
    display: flex;
    flex-direction: row;
    gap: 0.4em;
  }

  .dev-links a {
    display: flex;
    gap: 0.4em;
    align-items: center;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    color: black;
  }
  .dev-links a:hover {
    text-decoration: none;
  }

  .dev-links a img {
    height: 1em;
  }

  .dev-links a:hover {
    background: rgba(128, 128, 128, 0.2);
  }
</style>
