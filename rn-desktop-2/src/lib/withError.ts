export const withError =
  (
    message: string | ((err: unknown) => string),
    errClass: { new (message: string, options?: { cause: unknown }): Error } = Error,
  ) =>
  (error: unknown) =>
    Promise.reject(
      new errClass(
        typeof message === "function" ? message(error) : message + " <cause>" + stringifyError(error) + "</cause>",
        { cause: error },
      ),
    );

const stringifyError = (error: unknown) => {
  if (error instanceof Error) {
    return error.message + "\n" + error.stack;
  }
  return JSON.stringify(error);
};
