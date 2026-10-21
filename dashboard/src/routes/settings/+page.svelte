<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { env } from '$lib/env';

  let apiBase = env.PUBLIC_API_BASE_URL;
  let brandName = 'AURELIA';

  onMount(() => {
    const token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
    }
  });
</script>

<div class="space-y-6 max-w-4xl">
  
  <div>
    <h1 class="font-serif text-3xl font-bold text-slate-900">System & Concierge Settings</h1>
    <p class="text-xs text-slate-500 font-light mt-1">Configure backend API connections, brand defaults, and security rules.</p>
  </div>

  <div class="space-y-6">
    <!-- Brand Identity Card -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-4">
      <h3 class="font-serif text-xl font-bold text-slate-900">Brand Identity</h3>
      
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1">Brand Name</label>
          <input type="text" bind:value={brandName} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:outline-none" />
        </div>
        <div>
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1">Default Currency</label>
          <input type="text" value="Bangladeshi Taka (BDT ৳)" disabled class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-500 bg-slate-50" />
        </div>
      </div>
    </div>

    <!-- API & Server Config -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-4">
      <h3 class="font-serif text-xl font-bold text-slate-900">Backend API Integration</h3>
      
      <div>
        <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1">Actix-web REST API Base URL</label>
        <input type="text" bind:value={apiBase} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono text-slate-900 focus:outline-none" />
      </div>

      <div class="p-3 bg-emerald-50 border border-emerald-200 rounded-xl text-xs text-emerald-900 flex items-center justify-between font-semibold">
        <span>● Connected to Rust PostgreSQL Pool (Port 8085)</span>
        <span>Latency: &lt; 2ms</span>
      </div>
    </div>

    <!-- Admin Account Security -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-4">
      <h3 class="font-serif text-xl font-bold text-slate-900">Concierge Security</h3>
      
      <div class="space-y-3">
        <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 border border-slate-200 text-xs">
          <div>
            <span class="font-bold text-slate-900 block">Master Admin Token</span>
            <span class="text-slate-500 text-[10px]">Argon2id JWT Authentication Active</span>
          </div>
          <span class="px-2.5 py-1 rounded-full bg-emerald-100 text-emerald-800 text-[10px] font-bold">Encrypted</span>
        </div>
      </div>
    </div>
  </div>

</div>
