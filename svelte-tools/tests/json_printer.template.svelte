<script>
  import { devStringify } from "./devstringify.ts";

  export let value = {
    a: 1,
    b: {
      c: [],
    },
  };
</script>

<pre>{devStringify(value)}</pre>

<style>
  pre {
    background: whitesmoke;
    padding: 20px;
  }
</style>
