import { AxiosError } from "axios";

export namespace ConstantSource {
  export const myGithubRepo = "https://github.com/startdusk/filebox";
}

export namespace Route {
  export const homePath = "/";
  export const storePath = "/store";
  export const pickupPath = "/pickup";
}

export function assertIsError(err: any): asserts err is Error {
  if (!(err instanceof Error)) throw new Error(`Not an error: ${err}`);
}

export function assertIsAxiosError(err: any): asserts err is AxiosError {
  if (!(err instanceof AxiosError))
    throw new Error(`Not an axios error: ${err}`);
}

export const MaxFileSize = 5 * 1024 * 1024;
