import { LogicalSize, Window } from "@tauri-apps/api/window";
import { TrayIcon } from "@tauri-apps/api/tray";
import { Menu, MenuItem } from "@tauri-apps/api/menu";
import { Image } from "@tauri-apps/api/image";
import { path } from "@tauri-apps/api";
import { withError } from "./withError";

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

  async #applyWindowAppearance(mini: boolean) {
    if (!this.window) return;
    const w = this.window;
    const miniHeight = 40;
    await Promise.all([
      // w.center(),
      w.setAlwaysOnTop(mini),
      w.setSize(mini ? new LogicalSize(400, miniHeight) : new LogicalSize(600, 400)),
      w.setMaximizable(!mini),
      w.setSizeConstraints(mini ? { maxHeight: miniHeight, minHeight: miniHeight, minWidth: 300, maxWidth: 3000 } : undefined),
      w.setDecorations(mini ? false : true),
      w.setTitleBarStyle(mini ? "transparent" : "visible"),
      w.setSkipTaskbar(!mini),
    ]);
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
          (this.showPlanner = await MenuItem.new({ text: "Show Planner", action: () => this.expandToPlanner() })),
          (this.showTracker = await MenuItem.new({ text: "Show Tracker", action: () => this.collapseToTracker() })),
        ],
      })),
    });
  }

  async setTitle(title: string | null) {
    if (title) {
      await this.tray?.setIcon(await TRAY_IMAGES.ZeroWidth);
      await this.tray?.setTitle(title);
      await this.window?.setTitle(title);
    } else {
      await this.tray?.setIcon(await TRAY_IMAGES.Base);
      await this.tray?.setTitle(null);
      await this.window?.setTitle("Right Now");
    }
  }

  async collapseToTracker() {
    if (!this.window) return;
    await this.#applyWindowAppearance(true);
    await this.showPlanner?.setEnabled(true);
    await this.showTracker?.setEnabled(false);
    await this.window.setFocus(); // Ensure window is visible when starting work
  }

  async expandToPlanner() {
    if (!this.window) return;
    await this.#applyWindowAppearance(false);
    await this.showPlanner?.setEnabled(false);
    await this.showTracker?.setEnabled(true);
  }

  // Helper to check current mode
  async isTrackerMode(): Promise<boolean> {
    if (!this.window) return false;
    const size = await this.window.innerSize();
    return size.width === 300; // Using width as proxy for mode
  }
}
