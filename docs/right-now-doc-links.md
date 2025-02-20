# Right Now Library Docs

Includes docs for Tauri and similar. It's important to know that while we're using Tauri and it is a Rust tool, we are only going to use the TypeScript APIs to make things simpler.

- [Tauri Config files](https://v2.tauri.app/reference/config/)
  Understand how to configure the app and how it will get built, the custom build steps, all those kinds of things, and some special details that are only accessible through the config, such as transparent windows on macOS, for example.

## Notifications

- [@tauri-apps/plugin-notification](https://v2.tauri.app/reference/javascript/notification/)
Send toast notifications (brief auto-expiring OS window element) to your user. Can also be used with the Notification Web API.

## Persistence

- [@tauri-apps/plugin-store](https://v2.tauri.app/reference/javascript/store/)
  - has `LazyStore`–a lazy loaded key-value store persisted by the backend layer.
  - has `Store`–a key-value store persisted by the backend layer.
  - These are useful for us for storing all that other state that doesn't actually go into the markdown file or into the markdown file's front matter, such as usually some kind of the small details of timing information per task. Each task has a very high resolution information about the timings, but we don't actually need to store that in the markdown file, but we might want to store that on the disk. Then anything that's related to recent projects that are opened and projects that may not have a dedicated markdown file, they could have a virtual markdown file that's just stored in the store.

## macOS/Windows Tray & Window management

- [System Tray (guide)](https://v2.tauri.app/learn/system-tray/)
  Tauri allows you to create and customize a system tray for your application. This can enhance the user experience by providing quick access to common actions.

- [Window Customization](https://v2.tauri.app/learn/window-customization/)
  We are going to create the main window and change its background color from the Rust side.
- Configuration
- Usage

  - Creating a Custom Titlebar
  - Manual Implementation of data-tauri-drag-region
  - (macOS) Transparent Titlebar with Custom Window Background Color

- [tray](https://v2.tauri.app/reference/javascript/api/namespacetray/) - Classes: TrayIcon - Interfaces: TrayIconOptions - Type Aliases: MouseButton, MouseButtonState, TrayIconClickEvent, TrayIconEvent, TrayIconEventBase<T>, TrayIconEventType
  Tray management is pretty important because we want to be able to show what task you're currently working on in the macOS tray so it can actually show the text of what you're working on to help you see what you're doing.

- [window](https://v2.tauri.app/reference/javascript/api/namespacewindow/)
  Provides APIs to create windows, communicate with other windows and manipulate the current window.

## Markdown file

- [FileSystem](https://v2.tauri.app/reference/javascript/fs/)

  - Access the file system.
  - Functions: copyFile(), create(), exists(), lstat(), mkdir(), open(), readDir(), readFile(), readTextFile(), readTextFileLines(), remove(), rename(), size(), stat(), truncate(), watch(), watchImmediate(), writeFile(), writeTextFile()

- [Dialog](https://v2.tauri.app/reference/javascript/dialog/)

  - Choose places to open/save a current right now TODO markdown files
  - e.g. `function open<T>(options): Promise<OpenDialogReturn<T>>` or `function save(options): Promise<string | null>`

- [@tauri-apps/plugin-global-shortcut](https://v2.tauri.app/reference/javascript/global-shortcut/)

  - Register global shortcuts.

- [@tauri-apps/plugin-opener](https://v2.tauri.app/reference/javascript/opener/)

  - Open files and URLs using their default application.
  - Potentially relevant if we to make it easy to open a markdown file in the user's default editor.

- [@tauri-apps/plugin-positioner](https://v2.tauri.app/reference/javascript/positioner/)
  - You can read where the window is and change the position of it which is really useful because we want to save the position of the tracker separate from the position of the planner states.
    - Also see [@tauri-apps/plugin-window-state](https://v2.tauri.app/reference/javascript/window-state/) for storing the state of the window to a file automatically
