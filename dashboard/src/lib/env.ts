export interface DashboardEnvConfig {
  PUBLIC_API_BASE_URL: string;
  PUBLIC_LANDING_URL: string;
}

const getApiBaseUrl = (): string => {
  try {
    const envVal = import.meta.env?.PUBLIC_API_BASE_URL || import.meta.env?.VITE_PUBLIC_API_BASE_URL;
    if (envVal) return envVal;
  } catch {
    // Fallback if import.meta.env is unavailable
  }
  return 'https://aureliaapi.haqplus.com/api/v1';
};

const getLandingUrl = (): string => {
  try {
    const envVal = import.meta.env?.PUBLIC_LANDING_URL || import.meta.env?.VITE_PUBLIC_LANDING_URL;
    if (envVal) return envVal;
  } catch {
    // Fallback
  }
  return 'https://haqplus.com';
};

export const env: DashboardEnvConfig = {
  PUBLIC_API_BASE_URL: getApiBaseUrl(),
  PUBLIC_LANDING_URL: getLandingUrl()
};

export default env;

