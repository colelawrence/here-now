<script lang="ts">
  // import {DiscordSettings} from "./discord.ts"
  import Input from "./Input.svelte";
  export let api_key = "00000001";
  export let client_id = "00000001";
</script>

<div class="flex flex-col justify-stretch">
  <Input label="API Key" bind:value={api_key} name="api_key" />
  <Input label="Client ID" bind:value={client_id} name="client_id" />
</div>
