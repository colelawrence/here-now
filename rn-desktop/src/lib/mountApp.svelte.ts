import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import type { InvokeArgs } from "@tauri-apps/api/tauri";
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
          // e.g. "UpdateTodos" or "UpdateWorkState" or "AddTodo" etc
          const variantId = handlers[i][0];
          const { payload } = e as any;
          if (variantId in payload) {
            handlers[i][1](payload[variantId]);
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
    rn: ui.createRightNowInvoker((eventName: string, payload: InvokeArgs | undefined) => {
      console.log("Invoked", eventName, payload);
      return invoke(eventName, payload);
    }),
    notify: {
      reportError(message, info) {
        alert(message + "\n\n" + JSON.stringify(info, null, 2));
        console.error(message, info);
        invoke("report_error", { error: { message, info } }).catch((e) => {
          console.error("Failed to report error", e);
        });
      },
    },
  });
}
