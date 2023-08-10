<script lang="ts">
    import { sanitizeHTML } from "../sanitizeHTML";
  import { PageHeader } from "./templates";
  export let header: PageHeader = { title: "Data Collections", links: [] };
</script>

<h1>
  <a href="/data" class="title-link">{header.title}</a>
  <div class="links">
    {#each header.links as [collection_label, href]}
      <a {href}>{collection_label}</a>
    {/each}
  </div>
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
    margin: 2rem auto;
  }
  :global(a) {
    color: dodgerblue;
    text-decoration: none;
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
  }
</style>
