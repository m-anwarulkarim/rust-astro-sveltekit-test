<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import { isSidebarCollapsed } from '$lib/stores/sidebarStore';

  $: isLoginPage = $page.url.pathname === '/login';

  function handleLogout() {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('admin_token');
      localStorage.removeItem('admin_user');
    }
    goto('/login');
  }
</script>

{#if isLoginPage}
  <slot />
{:else}
  <div class="min-h-screen bg-[#FAF7F2] text-slate-800 flex">
    <!-- Collapsible Responsive Sidebar -->
    <Sidebar />

    <!-- Main Content Area -->
    <div class={`flex-1 flex flex-col min-w-0 transition-[margin] duration-200 ease-in-out ${$isSidebarCollapsed ? 'lg:ml-20' : 'lg:ml-64'}`}>
      <Header on:logout={handleLogout} />

      <main class="p-6 md:p-8 flex-1 max-w-7xl w-full mx-auto space-y-8">
        <slot />
      </main>

      <footer class="p-6 border-t border-[var(--border-color)] bg-white/40 text-center text-xs text-slate-500 font-light">
        © 2026 Aurelia Concierge System. Enterprise Leather Footwear Management.
      </footer>
    </div>
  </div>
{/if}
