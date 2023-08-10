export function sanitizeHTML(html: string): string {
  return html.replace(/</g, "&lt;").replace(/&lt;(\/?(?:code|u|b|em|strong|ul|li)>)/g, "<$1")
}