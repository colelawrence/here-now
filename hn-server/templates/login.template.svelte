<script lang="ts">
  import { LoginURL } from "./login-page.ts"
  export let loginURLs = [
    LoginURL({ label: "Discord", url: "#login-discord" }),
    LoginURL({ label: "Slack", url: "#login-slack" }),
    LoginURL({ label: "Google Workspace", url: "#login-google-workspace" }),
  ];
</script>

<h1><span class="wave">👋</span>&nbsp;Here Now</h1>
{#each loginURLs as { url, label }}
  <a href={url} class="login-btn">Continue with {label}</a>
{/each}

<style>
  :root {
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell,
      "Open Sans", "Helvetica Neue", sans-serif;
    max-width: 480px;
    margin: 2rem auto;
  }
  .login-btn {
    text-decoration: none;
    display: block;
    padding: 16px 12px;
  }
  .login-btn:hover {
    background: rgba(128, 128, 138, 0.1);
  }
  h1 {
    position: relative;
  }
  .wave {
    position: absolute;
    right: 100%;
  }
</style>
