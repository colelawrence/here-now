/** `#[codegen(tags = "templates", template = "login")]` */
export type LoginProps = {
  loginURLs: Array<LoginURL>;
};
/** `#[codegen(tags = "templates", template = "login")]` */
export function LoginProps(inner: LoginProps): LoginProps {
  return inner;
}
/**
 * What kind of login URL?
 *
 * `#[codegen(tags = "templates")]`
 */
export type LoginURL = {
  label: string;
  url: string;
};
/**
 * What kind of login URL?
 *
 * `#[codegen(tags = "templates")]`
 */
export function LoginURL(inner: LoginURL): LoginURL {
  return inner;
}
/** `#[codegen(tags = "templates")]` */
export type CallbackError = {
  error: string;
  error_description?: string | undefined | null | null | undefined;
};
/** `#[codegen(tags = "templates")]` */
export function CallbackError(inner: CallbackError): CallbackError {
  return inner;
}
/** `#[codegen(tags = "templates", template = "discord-callback")]` */
export type DiscordCallbackProps = {
  query: DiscordCallbackQuery;
  text: string;
};
/** `#[codegen(tags = "templates", template = "discord-callback")]` */
export function DiscordCallbackProps(inner: DiscordCallbackProps): DiscordCallbackProps {
  return inner;
}
/** `#[codegen(tags = "templates")]` */
export type DiscordCallbackQuery = {
  code?: string | undefined | null | null | undefined;
} // flattened fields:
/**
 * `error=invalid_scope&error_description=the+requested+scope+is+invalid%2c+unknown%2c+or+malformed.`
 *
 * `#[serde(flatten)]`
 *
 * Flattened from `.error`.
 */
& Partial<CallbackError | undefined | null>;
/** `#[codegen(tags = "templates")]` */
export function DiscordCallbackQuery(inner: DiscordCallbackQuery): DiscordCallbackQuery {
  return inner;
}