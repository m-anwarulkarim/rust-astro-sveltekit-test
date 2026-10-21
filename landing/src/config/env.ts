export interface AstroEnvConfig {
  PUBLIC_API_BASE_URL: string;
}

const parseEnv = (): AstroEnvConfig => {
  const publicApiUrl = import.meta.env.PUBLIC_API_BASE_URL || 'http://localhost:8085/api/v1';

  return {
    PUBLIC_API_BASE_URL: publicApiUrl,
  };
};

export const env: AstroEnvConfig = parseEnv();
export default env;
