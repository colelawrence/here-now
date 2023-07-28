<script>
	export let count = 0;

	function handleClick() {
		count += 1;
	}
</script>

<button on:click={handleClick}>
	Clicked {count}
	{count === 1 ? 'time' : 'times'}
</button>

<style>
  button {
    background: lawngreen;
    border: 1px solid green;
    font-family: cursive, system-ui;
  }
  button:hover {
    background: green;
    color: lawngreen;
  }
</style>
