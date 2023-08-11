<script lang="ts">
  import Header from "./Header.svelte";
  import { DataCollections } from "./data-collections.props.ts";
  export let header: DataCollections["header"] = { title: "Data Collections", links: [] };
</script>

<Header {header} />

<p>Welcome to the data browser, this shows us information in BonsaiDB and if it's linked to an entity.</p>
