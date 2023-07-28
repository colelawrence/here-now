<script>
  import Button from "./increment/Button.svelte"
	let count = 0;

	function handleClick() {
		count += 1;
	}
</script>


<p>
  <button on:click={handleClick} data-hx-get="/het">
    Clicked {count}
    {count === 1 ? 'time' : 'times'}
  </button>
  <Button count={1}/>
</p>

<style>
  button {
    background: dodgerblue;
    border: 1px solid blue;
    font-family: cursive, system-ui;
  }
  button:hover {
    background: blue;
    color: dodgerblue;
  }
</style>
