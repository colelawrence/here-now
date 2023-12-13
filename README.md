## <img src="./rn-desktop/src-tauri/icons/app-icon.png" height="24px" /> Right Now

This is an offline-first desktop app for tracking todos with timers.

![Screen capture of Right Now in macOS tray](rn-desktop/screenshots/2023-12-13-Screen-capture-of-Right-Now.png)

## <img src="./hn-server/templates/public/favicon.png" height="24px" /> Here Now

[Tentative plans for "Here Now"](./README-here-now.md).

## Getting started

**Install watchexec** for watch commands like `cargo xtask dev` and `cargo xtask web-build --watch`.

```sh
cargo install watchexec-cli
```

**Get all the submodules** (we have several forks with smal tweaks so we can more easily upstream fixes as we build)

```sh
git submodule update --recursive --init
```

**Install node** ([I recommend using nvm](https://github.com/nvm-sh/nvm)) and then install the npm dependencies (these are for tailwind intellisense, prettier, and for a special launch-editor function from Jaeger).

```sh
npm install
```

**Install deno** ([installation](https://deno.land/manual@v1.35.3/getting_started/installation)). Deno is used for code generation and for as many non-cargo scripting things as we can manage, since it's much easier to maintain security of deno scripts than node scripts.

## Using Jaeger

Jaeger is a way for us to look at what's happening inside our application based on it's tracing spans.
This tooling greatly improves our ability to understand what happened and what is taking a long time to perform.

![](./doc-images/2023-08-02_Here-Now_Dev_Tools_with_Jaeger_UI.png)
[See a recorded Loom: "2023-08-02 Here-Now Dev Tools with Jaeger UI"](https://www.loom.com/share/1cc6765cfe6046408d672da0520eed87)

### Using Jaeger locally

Install `jaeger-all-in-one` by adding [the binary from releases](https://github.com/jaegertracing/jaeger/releases/) to your `PATH`.

```sh
cargo xtask jaeger # in one terminal
# right now
cargo xtask rn dev # in the other
# here now
cargo xtask dev-server # in the other
cargo xtask dev-desktop # in the other
```

### Using Jaeger in docker.

```sh
cargo xtask jaeger --docker # in one terminal
# right now
cargo xtask rn dev # in the other
# here now
cargo xtask dev-server # in the other
cargo xtask dev-desktop # in the other
```

### Codebase maintenance

Tools for maintaining tidyness in the codebase.

- `xtask`: all codebase commands go through `cargo xtask ...` and those go to this executable.
  This allows us to write the majority of our code in Rust and cross-platform, including codebase management code. See [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask).
- `workspace-hack`: A crate maintained by [hakari](https://docs.rs/cargo-hakari/latest/cargo_hakari/), updated by the `cargo xtask hakari` command.
