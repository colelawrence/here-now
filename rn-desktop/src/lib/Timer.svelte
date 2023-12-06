<script lang="ts">
  import type { Placement } from "tippy.js";
  import { mountTimerDisplay } from "./MountedTimerInfo.svelte";
  import type { TimerInfo } from "./createApp.svelte";
  import { tooltip } from "./tooltip";
  const { info, popoverPlacement } = $props<{ info: TimerInfo; popoverPlacement: Placement }>();
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
  use:tooltip={{ content: display.label, placement: popoverPlacement }}
  aria-label={display.label}
  on:click={() => (countUp = !countUp)}
>
  {display.timeDisplay}
</button>
