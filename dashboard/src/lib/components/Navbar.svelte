<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { env } from '$lib/env';
  export let adminUser: string = 'Master Administrator';

  const dispatch = createEventDispatcher();

  function logout() {
    dispatch('logout');
  }
</script>

<header class="glass-header sticky top-0 z-30">
  <div class="max-w-7xl mx-auto px-6 h-20 flex items-center justify-between">
    <div class="flex items-center gap-3">
      <img src="/favicon.svg" alt="Aurelia Logo" class="w-9 h-9" />
      <div>
        <span class="font-serif text-xl font-bold text-slate-900 block leading-tight">AURELIA Control</span>
        <span class="text-[10px] uppercase tracking-widest text-[var(--accent-gold)] font-bold">Admin Management</span>
      </div>
    </div>

    <div class="flex items-center gap-6">
      <a href={env.PUBLIC_LANDING_URL} target="_blank" class="text-xs font-semibold text-slate-600 hover:text-slate-900 flex items-center gap-1">
        <span>View Public Store</span>
        ↗
      </a>
      <div class="flex items-center gap-3 pl-4 border-l border-slate-200">
        <div class="text-right hidden sm:block">
          <span class="text-xs font-bold text-slate-900 block">{adminUser}</span>
          <span class="text-[10px] text-emerald-600 font-semibold block">● Active Concierge</span>
        </div>
        <button on:click={logout} class="px-3 py-1.5 rounded-lg border border-slate-200 text-xs font-semibold text-slate-600 hover:bg-slate-100">
          Sign Out
        </button>
      </div>
    </div>
  </div>
</header>
