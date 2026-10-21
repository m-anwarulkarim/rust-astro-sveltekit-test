<script lang="ts">
  import { page } from '$app/stores';
  import { isSidebarOpen, isSidebarCollapsed, closeSidebar, toggleCollapse } from '$lib/stores/sidebarStore';

  const navItems = [
    {
      name: 'Overview',
      path: '/',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/></svg>`
    },
    {
      name: 'Orders',
      path: '/orders',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"/></svg>`
    },
    {
      name: 'Shoes Inventory',
      path: '/products',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>`
    },
    {
      name: 'Customers',
      path: '/customers',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/></svg>`
    },
    {
      name: 'API & Integrations',
      path: '/api-settings',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/></svg>`
    },
    {
      name: 'Settings',
      path: '/settings',
      icon: `<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>`
    }
  ];
</script>

<!-- Mobile Overlay Backdrop -->
{#if $isSidebarOpen}
  <button 
    on:click={closeSidebar}
    class="fixed inset-0 z-40 bg-slate-900/60 backdrop-blur-sm lg:hidden transition-opacity"
    aria-label="Close Mobile Sidebar"
  ></button>
{/if}

<!-- Sidebar Container -->
<aside 
  class={`fixed top-0 left-0 bottom-0 z-50 bg-[#0F172A] text-slate-200 transition-all duration-300 flex flex-col border-r border-slate-800 shadow-2xl ${$isSidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'} ${$isSidebarCollapsed ? 'lg:w-20' : 'w-64'}`}
>
  <!-- Sidebar Brand Header -->
  <div class="h-20 px-5 flex items-center justify-between border-b border-slate-800/80">
    <a href="/" class="flex items-center gap-3 text-decoration-none group overflow-hidden">
      <img src="/favicon.svg" alt="Logo" class="w-9 h-9 flex-shrink-0 group-hover:scale-105 transition-transform" />
      {#if !$isSidebarCollapsed}
        <div class="transition-opacity duration-200">
          <span class="font-serif text-lg font-bold text-white tracking-wide block leading-tight">AURELIA</span>
          <span class="text-[9px] uppercase tracking-widest text-[#D4AF37] font-bold block">Control Panel</span>
        </div>
      {/if}
    </a>

    <!-- Mobile Close Button -->
    <button on:click={closeSidebar} class="lg:hidden text-slate-400 hover:text-white p-1">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>

    <!-- Desktop Collapse Toggle -->
    <button 
      on:click={toggleCollapse} 
      class="hidden lg:flex items-center justify-center w-7 h-7 rounded-lg bg-slate-800 text-slate-300 hover:text-amber-400 transition-colors border border-slate-700"
      title={$isSidebarCollapsed ? 'Expand Sidebar' : 'Collapse Sidebar'}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={$isSidebarCollapsed ? "M13 5l7 7-7 7M5 5l7 7-7 7" : "M11 19l-7-7 7-7M19 19l-7-7 7-7"} />
      </svg>
    </button>
  </div>

  <!-- Navigation Menu -->
  <nav class="flex-1 py-6 px-3 space-y-2 overflow-y-auto">
    {#each navItems as item}
      {@const isActive = $page.url.pathname === item.path}
      <a 
        href={item.path}
        data-sveltekit-preload-code="eager"
        data-sveltekit-noscroll
        on:click={closeSidebar}
        class={`flex items-center gap-3 px-4 py-3 rounded-xl text-xs font-bold transition-all ${isActive ? 'bg-[#D4AF37] text-slate-950 shadow-lg shadow-amber-500/25 ring-1 ring-amber-300' : 'text-slate-300 hover:bg-slate-800/80 hover:text-white'}`}
        title={$isSidebarCollapsed ? item.name : ''}
      >
        <span class={`flex-shrink-0 ${isActive ? 'text-slate-950' : 'text-slate-400'}`}>
          {@html item.icon}
        </span>
        {#if !$isSidebarCollapsed}
          <span class="truncate tracking-wide">{item.name}</span>
        {/if}
      </a>
    {/each}
  </nav>

  <!-- Sidebar Footer Concierge Info -->
  <div class="p-4 border-t border-slate-800/80 bg-slate-950/60">
    {#if !$isSidebarCollapsed}
      <div class="space-y-1">
        <span class="text-[10px] uppercase font-bold text-slate-400 tracking-wider block">Logged as</span>
        <span class="text-xs font-bold text-white block truncate">Master Administrator</span>
        <div class="flex items-center gap-1.5 pt-0.5">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          <span class="text-[10px] text-emerald-400 font-bold block">Online Concierge</span>
        </div>
      </div>
    {:else}
      <div class="w-3 h-3 bg-emerald-400 rounded-full mx-auto" title="Online Concierge"></div>
    {/if}
  </div>
</aside>
