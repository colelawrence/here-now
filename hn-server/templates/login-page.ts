/** `#[codegen(tags = "login-page")]` */
export type LoginProps = {
  loginURLs: Array<LoginURL>;
};
/** `#[codegen(tags = "login-page")]` */
export function LoginProps(inner: LoginProps): LoginProps {
  return inner;
}
/**
 * What kind of login URL?
 *
 * `#[codegen(svelte_enum, tags = "login-page")]`
 */
export type LoginURL = {
  label: string;
  url: string;
};
/**
 * What kind of login URL?
 *
 * `#[codegen(svelte_enum, tags = "login-page")]`
 */
export function LoginURL(inner: LoginURL): LoginURL {
  return inner;
}