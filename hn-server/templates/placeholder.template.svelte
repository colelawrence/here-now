<script lang="ts">
  import Header from "./Header.svelte";
  import {devStringify} from "./helpers/devstringify";
</script>

<Header />

<div class="result">{devStringify($$restProps)}</div>

<style>
  .result {
    padding: 1rem;
    background: rgba(125, 255, 125, 0.2);
    white-space: pre-wrap;
  }
</style>
