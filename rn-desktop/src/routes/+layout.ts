import "overlayscrollbars/overlayscrollbars.css";
import "./styles.css";
// Reference for future https://github.com/colinlienard/gitlight/blob/main/src/routes/%2Blayout.ts

// Needed for svelte kit static adapter
export const prerender = true;
// Needed so we can assume window is defined for Tauri plugins
export const ssr = false;

// // Listen for scheme request on desktop app
// if (typeof window !== "undefined" && window.__TAURI__) {
//   listen("scheme-request", ({ payload }) => {
//     const searchParams = new URLSearchParams((payload as string).replace("gitlight://", ""));
//     // const githubAccessToken = searchParams.get('github_access_token');
//     // const gitlabAccessToken = searchParams.get('gitlab_access_token');
//     // const gitlabRefreshToken = searchParams.get('gitlab_refresh_token');
//     // const gitlabExpiresIn = searchParams.get('gitlab_expires_in');

//     // if (
//     // 	(githubAccessToken && storage.has('gitlab-user')) ||
//     // 	(gitlabAccessToken && storage.has('github-user'))
//     // ) {
//     // 	window.location.reload();
//     // } else {
//     goto("/dashboard");
//     // }
//   });

//   // Enable autostart
//   if (!(await isEnabled())) {
//     enable();
//   }
// }
