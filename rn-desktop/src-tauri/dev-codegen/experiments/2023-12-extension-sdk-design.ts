// @ts-nocheck
// deno-lint-ignore-file
interface System {
  audio;
}

type MaybePromise<T> = T | Promise<T>;
type TimeFormat = { iso: string } | { unixSecsSinceEpoch: number } | Date;
interface RightNowPlugin {
  onLoadAssets: () => MaybePromise<{
    audio: { filetype: "mp3"; key: string; bytes: Uint8Array }[];
  }>;
  syncEvents: () => MaybePromise<{
    events: {
      key: string;
      title: string;
      description: string;
      startsAt: TimeFormat;
      endsAt: TimeFormat;
    }[];
  }>;
}

const calendar: RightNowPlugin = {
  onLoadAssets: async () => {
    return {
      audio: [
        {
          key: "1",
          filetype: "mp3",
          bytes: await Deno.readFile("calendar/1.mp3"),
        },
      ],
    };
  },
};
