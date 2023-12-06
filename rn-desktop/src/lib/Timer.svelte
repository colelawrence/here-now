<script lang="ts">
  import { mountTimerDisplay } from "./MountedTimerInfo.svelte";
  import type { TimerInfo } from "./createApp.svelte";
  import { tooltip } from "./tooltip";
  const { info } = $props<{ info: TimerInfo }>();
  let countUp = $state(false);
  const display = mountTimerDisplay({
    get info() {
      return info;
    },
    get countUp() {
      return countUp;
    },
  });
</script>

<button
  class="flex text-ui-lg font-bold p-1 rounded border-sys-on-primary select-none"
  data-tauri-drag-region
  use:tooltip={{ content: display.label }}
  aria-label={display.label}
  on:click={() => (countUp = !countUp)}
>
  {display.timeDisplay}
</button>
