<script lang="ts">
  import { mountTimerDisplay } from "./MountedTimerInfo.svelte";
  import type { TimerInfo } from "./createApp.svelte";
  const { info } = $props<{ info: TimerInfo }>();
  const display = mountTimerDisplay(info);
</script>

<div class="flex text-ui-lg font-bold p-1 rounded border-sys-on-primary select-none" data-tauri-drag-region>
  {display.timeDisplay}
</div>
