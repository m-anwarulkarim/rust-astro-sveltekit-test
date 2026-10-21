import { writable } from 'svelte/store';

export const isSidebarOpen = writable<boolean>(false);
export const isSidebarCollapsed = writable<boolean>(false);

export function toggleSidebar() {
  isSidebarOpen.update(v => !v);
}

export function closeSidebar() {
  isSidebarOpen.set(false);
}

export function toggleCollapse() {
  isSidebarCollapsed.update(v => !v);
}
