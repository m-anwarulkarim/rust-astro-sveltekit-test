export interface DashboardEnvConfig {
  PUBLIC_API_BASE_URL: string;
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

export const env: DashboardEnvConfig = {
  PUBLIC_API_BASE_URL: getApiBaseUrl()
};

export default env;
