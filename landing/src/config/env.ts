/// <reference path="../env.d.ts" />

export interface AstroEnvConfig {
  PUBLIC_API_BASE_URL: string;
}

const parseEnv = (): AstroEnvConfig => {
  const publicApiUrl =
    (import.meta as any).env?.PUBLIC_API_BASE_URL ||
    'https://aureliaapi.haqplus.com/api/v1';

  return {
    PUBLIC_API_BASE_URL: publicApiUrl,
  };
};

export const env: AstroEnvConfig = parseEnv();
export default env;
