<script lang="ts">
  import Header from "./Header.svelte";
import { LoginProps } from "./login.props.ts";
  import { LoginURL } from "./templates.ts";
  export let note: LoginProps["note"] = "Note"
  export let loginURLs: LoginProps["loginURLs"] = [
    LoginURL({ label: "Discord", url: "#login-discord" }),
    LoginURL({ label: "Slack", url: "#login-slack" }),
    LoginURL({ label: "Google Workspace", url: "#login-google-workspace" }),
  ];
</script>

<Header/>

{#if note }
  <p class="note">{note}</p>
{/if}

{#each loginURLs as { url, label }}
  <a href={url} class="login-btn">{label}</a>
{/each}

<style>
  .login-btn {
    text-decoration: none;
    display: block;
    padding: 16px 12px;
  }
  .login-btn:hover {
    background: rgba(128, 128, 138, 0.1);
  }
  .note {
    font-size: 0.8rem;
    color: #666;
    margin-bottom: 16px;
  }
</style>
