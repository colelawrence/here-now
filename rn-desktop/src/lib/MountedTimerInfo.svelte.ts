export type MountedTimerInfo = {
  /** e.g. "0:20" or "+2:00" */
  timeDisplay: string;
};

export function timeUnixNow(): number {
  return (Date.now() * 0.001) | 0;
}
export function mountTimerDisplay(value: { endsAtUnix: number }): MountedTimerInfo {
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
      const timeLeft = value.endsAtUnix - nowUnix;
      if (timeLeft < 0) return `+${formatTimeLeft(-timeLeft)}`;
      return formatTimeLeft(timeLeft);
    },
  };
}

function formatTimeLeft(secs: number) {
  const minutes = Math.floor(secs / 60);
  const seconds = Math.floor(secs % 60);
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}
