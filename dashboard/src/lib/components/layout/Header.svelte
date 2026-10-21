<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { toggleSidebar, isSidebarCollapsed } from '$lib/stores/sidebarStore';
  import { env } from '$lib/env';

  export let adminUser: string = 'Master Administrator';

  const dispatch = createEventDispatcher();

  function logout() {
    dispatch('logout');
  }
</script>

<header 
  class="sticky top-0 z-30 bg-white/80 backdrop-blur-md border-b border-[var(--border-color)]"
>
  <div class="px-6 h-20 flex items-center justify-between gap-4">
    
    <!-- Left: Mobile Menu Toggle & Title -->
    <div class="flex items-center gap-3">
      <button 
        on:click={toggleSidebar}
        class="lg:hidden p-2 rounded-xl border border-slate-200 text-slate-700 hover:bg-slate-100 transition-colors"
        aria-label="Toggle Navigation Sidebar"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
        </svg>
      </button>

      <div>
        <h2 class="font-serif text-lg font-bold text-slate-900 leading-tight">Control Center</h2>
        <span class="text-[10px] uppercase tracking-widest text-[var(--accent-gold)] font-bold block">Aurelia Enterprise</span>
      </div>
    </div>

    <!-- Right: Quick Store Link & Admin Profile -->
    <div class="flex items-center gap-4 sm:gap-6">
      <a 
        href={env.PUBLIC_LANDING_URL} 
        target="_blank" 
        class="hidden sm:flex items-center gap-1.5 px-3.5 py-1.5 rounded-full bg-slate-50 border border-slate-200 text-xs font-semibold text-slate-700 hover:border-[var(--accent-gold)] hover:text-slate-900 transition-colors"
      >
        <span>View Public Store</span>
        ↗
      </a>

      <div class="flex items-center gap-3 pl-4 border-l border-slate-200">
        <div class="text-right hidden md:block">
          <span class="text-xs font-bold text-slate-900 block">{adminUser}</span>
          <span class="text-[10px] text-emerald-600 font-semibold block">● Active Master</span>
        </div>

        <button 
          on:click={logout} 
          class="px-3.5 py-2 rounded-xl bg-slate-900 text-amber-300 text-xs font-bold hover:bg-slate-800 transition-colors shadow-sm"
        >
          Sign Out
        </button>
      </div>
    </div>

  </div>
</header>
