/** `#[codegen(outputname = "discord", tags = "svelte")]` */
export type DiscordSettings = {
  app_id?: string | undefined | null | null | undefined;
  app_secret?: string | undefined | null | null | undefined;
  oauth2_client_secret?: string | undefined | null | null | undefined;
};
/** `#[codegen(outputname = "discord", tags = "svelte")]` */
export function DiscordSettings(inner: DiscordSettings): DiscordSettings {
  return inner;
}