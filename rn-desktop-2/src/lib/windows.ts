import { LogicalSize, Window } from "@tauri-apps/api/window";
import { TrayIcon } from "@tauri-apps/api/tray";
import { Menu, MenuItem } from "@tauri-apps/api/menu";
import { Image } from "@tauri-apps/api/image";
import { path } from "@tauri-apps/api";
import { withError } from "./withError";
import { atom, getDefaultStore } from "jotai";
import { exit } from "@tauri-apps/plugin-process";

const res = (resPath: `resources/${string}`) => path.resolveResource(resPath);

const TRAY_IMAGES = {
  ZeroWidth: res("resources/tray-0width.png")
    .then((a) => Image.fromPath(a))
    .catch(withError(`Failed to load ZeroWidth tray icon from resources/tray-0width.png`)),
  Base: res("resources/tray-base.png")
    .then((a) => Image.fromPath(a))
    .catch(withError(`Failed to load Base tray icon from resources/tray-base.png`)),
  Base2x: res("resources/tray-base@2x.png")
    .then((a) => Image.fromPath(a))
    .catch(withError(`Failed to load Base2x tray icon from resources/tray-base@2x.png`)),
} as const;

export class AppWindows {
  // main window
  private window?: Window;
  // See https://v2.tauri.app/learn/system-tray/ for tray docs
  private tray?: TrayIcon;
  private trayMenu?: Menu;
  private showPlanner?: MenuItem;
  private showTracker?: MenuItem;
  private focus?: MenuItem;
  #currentlyMiniAtom = atom<boolean | undefined>();
  public readonly currentlyMiniAtom = atom(
    (get) => get(this.#currentlyMiniAtom),
    async (_get, set, mini: boolean) => {
      this.#applyWindowAppearance(mini);
      await this.showPlanner?.setEnabled(mini);
      await this.showTracker?.setEnabled(!mini);
    },
  );

  async #applyWindowAppearance(mini: boolean) {
    if (!this.window) return;
    const store = getDefaultStore();
    if (mini === store.get(this.#currentlyMiniAtom)) return;
    store.set(this.#currentlyMiniAtom, mini);
    const w = this.window;
    const miniHeight = 40;
    await Promise.all([
      w.setAlwaysOnTop(mini),
      w.setSize(mini ? new LogicalSize(400, miniHeight) : new LogicalSize(600, 400)),
      w.setMaximizable(!mini),
      w.setSizeConstraints(
        mini ? { maxHeight: miniHeight, minHeight: miniHeight, minWidth: 300, maxWidth: 3000 } : undefined,
      ),
      w.setDecorations(mini ? false : true),
      w.setTitleBarStyle(mini ? "transparent" : "visible"),
      w.setSkipTaskbar(!mini),
    ]);
    // Ensure window is visible when interacting with it
    await w.setFocus();
  }

  async initialize() {
    this.window = Window.getCurrent();
    // Setup tray with current task
    this.tray = await TrayIcon.new({
      id: "main-tray",
      icon: await TRAY_IMAGES.Base,
      title: undefined,
      menu: (this.trayMenu = await Menu.new({
        id: "tray-menu",
        items: [
          (this.focus = await MenuItem.new({ text: "Focus", action: () => this.window?.setFocus() })),
          (this.showPlanner = await MenuItem.new({ text: "Show Planner", action: () => this.expandToPlanner() })),
          (this.showTracker = await MenuItem.new({ text: "Show Tracker", action: () => this.collapseToTracker() })),
          await MenuItem.new({ text: "Quit", action: () => exit(0) }),
        ],
      })),
    });
  }

  async setTitle(title: string | null) {
    if (title) {
      await this.tray?.setIcon(await TRAY_IMAGES.ZeroWidth);
      await this.tray?.setTitle(truncateString(title, 15, "…"));
      await this.window?.setTitle(title);
    } else {
      await this.tray?.setIcon(await TRAY_IMAGES.Base);
      await this.tray?.setTitle(null);
      await this.window?.setTitle("Right Now");
    }
  }

  async collapseToTracker() {
    const store = getDefaultStore();
    store.set(this.currentlyMiniAtom, true);
  }

  async expandToPlanner() {
    const store = getDefaultStore();
    store.set(this.currentlyMiniAtom, false);
  }
}

function truncateString(str: string, maxLength: number, ellipsis: string) {
  return str.length > maxLength + ellipsis.length ? str.slice(0, maxLength) + ellipsis : str;
}
