/// <reference types="@sveltejs/kit" />

declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

declare module '$env/static/public' {
  export const PUBLIC_API_BASE_URL: string;
}

declare module '$env/static/private' {
  // Add private env variables if needed
}

export {};
