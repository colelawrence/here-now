<script lang="ts">
  import Header from "./Header.svelte";
  import { DiscordCallbackProps } from "./discord-callback.props";
  export let query: DiscordCallbackProps["query"] = null;
  export let text: DiscordCallbackProps["text"] = "Text";
</script>

<Header />

{#if text}
<div class="result">{text}</div>
{/if}
{#if query.error}
  <div class="error">
    <b>{query.error}</b>
    {#if query.error_description}
      : {query.error_description}
    {/if}
  </div>
{/if}

<style>
  .error {
    padding: 1rem;
    background: rgba(255, 125, 125, 0.2);
  }
  .result {
    padding: 1rem;
    background: rgba(125, 255, 125, 0.2);
  }
</style>
