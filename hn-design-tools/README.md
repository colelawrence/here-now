Hey everyone, I've got another project I'm building in conjunction with the `derive-codegen` project I mentioned before.

Essentially, I'm working on a piece of tooling which helps me maintain design tokens between multiple different languages and environments.

The initial environments include a custom loaded Tailwind CSS plugin, Figma plugin which updates Text Styles and Color Variables, and eventually Slint-ui widgets.

I'm not super far along, but I'm building off of a strong foundation I created for a previous project called Artprompt.

**Motivation:**

 * Learn more about color science and typography while delivering a valuable tool for synchronizing design tokens across ecosystems.
 * Contribute another tool for correct/precise design system maintenance to the Rust ecosystem
  * (I know most tools are in TypeScript, but I have major TypeScript/JavaScript package management fatigue)

🧑‍🤝‍🧑 **Target audience:**

 * Small-to-medium sized (5-30 contributor) teams with a product with strong needs for themability
 * Determined Rust nerds who want to use Rust for as much of their Design Ops tooling as possible

**Project Overview:**

 * The MVP is a simple JSON-in JSON-out library which can be made into a CLI or WASM module to load in browser, nodejs, etc.
  * JSON-inputs define two kinds of rule-sets including the "tokens" set, then the "selection" set.
    * Some systems like TailwindCSS can use all tokens together, while some softwares like Figma's Text Styles require many tokens are grouped together.
 * Most of the project is in Rust with end-user configuration passed in with JSON (via deno / node / Rust)
 * The tooling validates the configuration of design token groups to generate appropriate configurations for a Tailwind Plugin and a Figma Plugin.
 

**About colors:**

 * I'm currently generating a base palette with the Material 3 Color System
 * I intend to use a color-space like Oklab for brightness/value (and for interpolation if necessary)
 * I wonder if we could extend this somewhat generic system to accept more kinds of design system coloring palette approaches.

**About typography:**

 * A big part of my motivation has been to make it easier to generate typography size scales which respect optical sizing [see "Font size is useless; let’s fix it" by Tonsky](https://tonsky.me/blog/font-size/) and which have pixel aligned caps or xs (see [Capsize CSS](https://seek-oss.github.io/capsize/) for more info)
 * This in combination with allowing tracking (letter-spacing) configuration with [Dynamic Metrics](https://rsms.me/inter/dynmetrics/), means we may be able to make it really easy to empower everyone to create legible, yet customizable, typography.

**About lengths/borders/box shadows/motion designs:**

I do not know, yet. But, I'm happy to collaborate with someone who wants to design something for these kinds of tokens.
