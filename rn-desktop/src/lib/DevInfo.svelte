<script lang="ts">
  const { info } = $props<{
    info: unknown;
  }>();
</script>

<pre>{JSON.stringify(info, null, 2)}</pre>

<style>
  pre::selection {
    background-color: cyan;
    color: #111;
  }
  pre {
    user-select: text;
    font-size: 12px;
    line-height: 112%;
    white-space: pre-wrap;
    padding: 0 0.5em;
    color: whitesmoke;
    background-color: purple;
  }
</style>
