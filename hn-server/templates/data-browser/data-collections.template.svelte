<script lang="ts">
  import Header from "./Header.svelte";
  import { DataCollections } from "./data-collections.props.ts";
  export let header: DataCollections["header"] = { title: "Data Collections" };
  export let collection_label_href: DataCollections["collection_label_href"] = [["Things", "/data/things"]];
</script>

<Header header={header} />

<div class="collections">
  {#each collection_label_href as [collection_label, href]}
    <a {href}>{collection_label}</a>
  {/each}
</div>

<style>
  .collections {
    display: flex;
    flex-direction: column;
  }
</style>
