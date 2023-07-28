<script lang="ts">
  export let name: string = "input";
  export let label: string = "Label";
  export let help: string | null = "Help text description";
  export let value: string = "value";
  export let type: string = "text";
  export let placeholer: string = "...";
</script>

<div class="flex flex-col justify-stretch">
  <label for={name} class="block mb-2 text-sm font-medium text-sys-on-background">{label}</label>
  <input
    {type}
    id={name}
    {name}
    {value}
    aria-describedby={help && `${name}-helper-text-explanation`}
    class="bg-sys-surface border border-sys-on-secondary-container text-on-secondary-container text-mono-base rounded-sm focus:ring-sys-primary focus:border-sys-primary block w-full p-2"
    placeholder={placeholer}
  />
  {#if help}
    <p id={`${name}-helper-text-explanation`} class="mt-2 text-ui-sm text-sys-on-secondary-container text-opacity-70">
      {help}
    </p>
  {/if}
</div>
