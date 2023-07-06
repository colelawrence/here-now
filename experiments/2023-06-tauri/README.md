# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## Tauri Desktop publishing

Benefits:

 + Leverage existing TypeScript / V8 performance knowledge
 + Leverage existing desktop publishing resources

Costs:

 - New stack of tools compared to just Rust / Slint for UI
 - Two UI frameworks maybe, with both Slint + Tauri

Exploring in #2

Resources
 * [CI/CD](https://www.jacobbolda.com/setting-up-ci-and-cd-for-tauri)
 * "Transparency/Custom title bars" https://tauri.app/v1/guides/features/window-customization/ https://github.com/tauri-apps/tauri/issues/4881


Alternative desktop publishing
 * Sniffnet with iced https://github.com/GyulyVGC/sniffnet/blob/main/.github/workflows/package.yml
 * Cargo-ui https://github.com/slint-ui/cargo-ui/blob/master/.github/workflows/build_binary.yaml
