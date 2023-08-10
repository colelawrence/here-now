<script lang="ts">
  import { devStringify } from "../helpers/devstringify";
  import { sanitizeHTML } from "../sanitizeHTML";
  import Header from "./Header.svelte";
  import { CollectionPage } from "./collection-page.props";
  export let header: CollectionPage["header"] = { title: "Title", links: [] };
  export let rows: CollectionPage["rows"] = [];

  const shorthand_lookup = {
    web: "devices",
    cred: "creds",
  };
</script>

<Header {header} />

<div class="rows">
  {#each rows as row}
    <div class="collection-row" id={row.id}>
      <a href="#{row.id}" class="title">{row.id}</a>
      <pre>{@html sanitizeHTML(
          devStringify(row.content).replace(
            /(token:\s*"\w{3})([^"]+?)(\w{3}")/g,
            (_, start, secret, end) => start + secret.replace(/./g, "*") + end
          )
        ).replace(
          // replace things like cred_awhuhawduihaw with a link to the corresponding collection with target to the id
          /"((\w{2,})_\w+)"/g,
          (_, id, shorthand) => `<a href="/data/${shorthand_lookup[shorthand] ?? `${shorthand}s`}#${id}">${id}</a>`
        )}</pre>
      {#if row.ecs_content}
        <pre>{@html sanitizeHTML(devStringify(row.ecs_content))}</pre>
        {:else}
        <pre class="warning">No ECS Content Loaded</pre>
      {/if}
    </div>
  {/each}
</div>

<style>
  .rows {
    display: flex;
    flex-direction: column;
  }

  .collection-row .title {
    font-weight: bold;
    margin-bottom: 0.5rem;
    text-decoration: none;
  }

  .collection-row {
    border-radius: 4px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .collection-row:target {
    background-color: #e6f6ff;
    border: 1px solid #9cc7ff;
  }

  .warning {
    padding: 1rem;
    background-color: #fffae6;
    border: 1px solid #ffeb9c;
  }
</style>
