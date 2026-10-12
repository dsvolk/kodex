export type KodexConfigValue = string | number | boolean | KodexConfigValue[] | KodexConfigObject;

export type KodexConfigObject = { [key: string]: KodexConfigValue };

export type KodexOptions = {
  kodexPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the Kodex CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: KodexConfigObject;
  /**
   * Raw `--config key=value` overrides to pass unchanged to the Kodex CLI after
   * structured configuration and before SDK-managed or thread-specific overrides.
   */
  configOverrides?: string[];
  /**
   * Environment variables passed to the Kodex CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
