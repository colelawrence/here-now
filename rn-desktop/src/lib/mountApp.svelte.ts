import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createApp, type AppState } from "./createApp.svelte";
import type { JotaiStore } from "./jotai-types";
import { ui } from "./ui";

export function mountAppInSvelte(store: JotaiStore): AppState {
  return createApp({
    sub(pool, a, fn) {
      const cleanup = store.sub(a, () => fn(store.get(a)));
      pool.addfn(cleanup);
      return cleanup;
    },
    listenToUIUpdates(handler) {
      let done = false;
      let unsub = () => {
        done = true;
      };
      const handlers = Object.entries(handler);
      listen("ui_update", (e) => {
        console.log("ui update event", e);
        for (let i = 0; i < handlers.length; i++) {
          const variantId = handlers[i][0];
          if (variantId in e) {
            handlers[i][1]((e.payload as any)[variantId] as any);
          }
        }
      }).then((unlisten) => {
        if (done) {
          unlisten();
        } else {
          unsub = unlisten;
        }
      });
      return () => unsub();
    },
    store,
    rnState: ui.createRightNowInvoker(invoke),
    notify: {
      reportError(message, info) {
        console.error(message, info);
        alert(message);
      },
    },
  });
}
