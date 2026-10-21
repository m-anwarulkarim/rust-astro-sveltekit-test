/// <reference path="../env.d.ts" />

export interface AstroEnvConfig {
  PUBLIC_API_BASE_URL: string;
  PUBLIC_DASHBOARD_URL: string;
}

const parseEnv = (): AstroEnvConfig => {
  const publicApiUrl =
    (import.meta as any).env?.PUBLIC_API_BASE_URL ||
    'https://aureliaapi.haqplus.com/api/v1';

  const publicDashboardUrl =
    (import.meta as any).env?.PUBLIC_DASHBOARD_URL ||
    'https://aureliadashboard.haqplus.com';

  return {
    PUBLIC_API_BASE_URL: publicApiUrl,
    PUBLIC_DASHBOARD_URL: publicDashboardUrl,
  };
};

export const env: AstroEnvConfig = parseEnv();
export default env;
