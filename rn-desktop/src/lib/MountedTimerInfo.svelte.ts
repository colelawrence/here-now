import type { TimerInfo } from "./createApp.svelte";

export type MountedTimerInfo = {
  /** e.g. "0:20" or "+2:00" */
  readonly timeDisplay: string;
  /** e.g. "Time worked" */
  readonly label: string;
};

export function timeUnixNow(): number {
  return (Date.now() * 0.001) | 0;
}

export function mountTimerDisplay(options: { readonly info: TimerInfo; readonly countUp: boolean }): MountedTimerInfo {
  let nowUnix = $state(timeUnixNow());
  $effect(() => {
    nowUnix = timeUnixNow();
    const interval = setInterval(() => {
      nowUnix = timeUnixNow();
    }, 150);
    return () => clearInterval(interval);
  });
  return {
    get timeDisplay() {
      if (options.countUp) {
        return formatTimeLeft(nowUnix - options.info.startedAtUnix);
      } else {
        const timeLeft = options.info.endsAtUnix - nowUnix;
        if (timeLeft < 0) return `+${formatTimeLeft(-timeLeft)}`;
        return formatTimeLeft(timeLeft);
      }
    },
    get label() {
      if (options.countUp) {
        return options.info.labelCountingUp;
      } else {
        return options.info.labelCountingDown;
      }
    },
  };
}

function formatTimeLeft(secs: number) {
  const minutes = Math.floor(secs / 60);
  const seconds = Math.floor(secs % 60);
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}
