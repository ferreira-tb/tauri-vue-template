/* eslint-disable no-inner-declarations */
/* eslint-disable @typescript-eslint/naming-convention */

declare global {
  var __DEBUG_ASSERTIONS__: boolean;
  var __DESKTOP__: boolean;
  var __MOBILE__: boolean;
  var __VERSION__: string;

  interface ErrorConstructor {
    throw: (message: string) => never;
    todo: (message?: Option<string>) => never;
  }

  interface Promise<T> {
    err: (message?: Option<string>) => void;
  }
}

export {};
