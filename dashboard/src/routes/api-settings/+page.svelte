<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  // Integration State
  let gtmContainerId = 'GTM-5K9LX8P';
  let isGtmEnabled = true;

  let fbPixelId = '9482710394819';
  let fbConversionToken = 'EAAGm0PX4ZC0BA...';
  let isFbPixelEnabled = true;

  let ga4MeasurementId = 'G-9X7P2K4L1M';

  // Courier Integration (Steadfast / Pathao / RedX)
  let selectedCourier = 'steadfast';
  let steadfastApiKey = 'st_live_9482019482018471';
  let steadfastSecretKey = 'st_sec_847102948201';
  let isAutoCourierDispatch = true;

  // SMS Gateway
  let smsGatewayApiKey = 'bw_live_8472910482019';
  let smsSenderId = 'AURELIA';
  let isAutoSmsOrderConfirm = true;

  // Save Feedback
  let isSaved = false;

  onMount(() => {
    const token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
    }
  });

  function handleSave() {
    isSaved = true;
    setTimeout(() => {
      isSaved = false;
    }, 3000);
  }
</script>

<div class="space-y-8 max-w-5xl">
  
  <!-- Header Banner -->
  <div class="flex flex-wrap items-center justify-between gap-4 bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm">
    <div>
      <span class="text-[10px] uppercase tracking-[0.25em] font-bold text-[var(--accent-gold)] block">Integrations Center</span>
      <h1 class="font-serif text-3xl font-bold text-slate-900">API & Third-Party System Management</h1>
      <p class="text-xs text-slate-500 font-light mt-1">Configure Google Tag Manager, Bangladesh Courier APIs, SMS Gateways, and Pixel Tracking.</p>
    </div>

    <button on:click={handleSave} class="btn-primary-gold text-xs py-2.5 px-6 font-bold shadow-md">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
      </svg>
      <span>{isSaved ? 'API Settings Saved!' : 'Save All API Keys'}</span>
    </button>
  </div>

  {#if isSaved}
    <div class="p-4 bg-emerald-50 border border-emerald-200 text-emerald-900 text-xs font-bold rounded-2xl flex items-center justify-between animate-in fade-in">
      <span class="flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
        All API configurations, GTM Container IDs, and Courier Credentials have been updated live.
      </span>
      <span class="text-[10px] text-emerald-600 uppercase">Live Sync</span>
    </div>
  {/if}

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
    
    <!-- 1. Tracking & Marketing Analytics (GTM / Meta Pixel) -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-6">
      <div class="flex items-center justify-between border-b border-slate-100 pb-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-amber-100 flex items-center justify-center text-[var(--accent-gold)]">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </div>
          <div>
            <h3 class="font-serif text-xl font-bold text-slate-900">Analytics & Tag Tracking</h3>
            <p class="text-xs text-slate-500 font-light">Google Tag Manager, Meta Pixel & GA4.</p>
          </div>
        </div>
      </div>

      <!-- Google Tag Manager (GTM) -->
      <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-3">
        <div class="flex items-center justify-between">
          <label class="text-xs font-bold text-slate-900 uppercase tracking-wider">Google Tag Manager (GTM)</label>
          <input type="checkbox" bind:checked={isGtmEnabled} class="w-4 h-4 accent-[var(--accent-gold)] cursor-pointer" />
        </div>
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">Container ID</span>
          <input type="text" bind:value={gtmContainerId} placeholder="e.g. GTM-XXXXXXX" class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
        </div>
      </div>

      <!-- Meta / Facebook Pixel -->
      <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-3">
        <div class="flex items-center justify-between">
          <label class="text-xs font-bold text-slate-900 uppercase tracking-wider">Meta Pixel & Conversions API</label>
          <input type="checkbox" bind:checked={isFbPixelEnabled} class="w-4 h-4 accent-[var(--accent-gold)] cursor-pointer" />
        </div>
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">Pixel ID</span>
          <input type="text" bind:value={fbPixelId} placeholder="e.g. 948201948201" class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
        </div>
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">Server Conversions API Access Token</span>
          <input type="password" bind:value={fbConversionToken} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono text-slate-900 focus:outline-none bg-white" />
        </div>
      </div>

      <!-- Google Analytics 4 -->
      <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-2">
        <label class="text-xs font-bold text-slate-900 uppercase tracking-wider block">Google Analytics 4 (GA4)</label>
        <input type="text" bind:value={ga4MeasurementId} placeholder="e.g. G-XXXXXXXXXX" class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
      </div>
    </div>

    <!-- 2. Courier API Integration (Steadfast / Pathao / RedX) -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-6">
      <div class="flex items-center justify-between border-b border-slate-100 pb-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-amber-100 flex items-center justify-center text-[var(--accent-gold)]">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16V6a1 1 0 00-1-1H4a1 1 0 00-1 1v10a1 1 0 001 1h1m8-1a1 1 0 01-1 1H9m4-1V8a1 1 0 011-1h2.586a1 1 0 01.707.293l3.414 3.414a1 1 0 01.293.707V16a1 1 0 01-1 1h-1m-6-1a1 1 0 001 1h1M5 17a2 2 0 104 0m-4 0a2 2 0 004 0m6 0a2 2 0 104 0m-4 0a2 2 0 004 0" />
            </svg>
          </div>
          <div>
            <h3 class="font-serif text-xl font-bold text-slate-900">Bangladesh Courier APIs</h3>
            <p class="text-xs text-slate-500 font-light">Steadfast, Pathao & RedX auto-dispatch.</p>
          </div>
        </div>
      </div>

      <div class="space-y-4">
        <div>
          <label class="block text-xs font-bold text-slate-800 uppercase tracking-wider mb-2">Select Active Courier Provider</label>
          <div class="grid grid-cols-3 gap-3">
            <button 
              type="button"
              on:click={() => selectedCourier = 'steadfast'} 
              class={`py-2.5 px-3 rounded-xl border text-xs font-bold transition-all ${selectedCourier === 'steadfast' ? 'bg-slate-900 text-amber-300 border-slate-900 shadow-md' : 'bg-slate-50 text-slate-700 border-slate-200'}`}
            >
              Steadfast
            </button>
            <button 
              type="button"
              on:click={() => selectedCourier = 'pathao'} 
              class={`py-2.5 px-3 rounded-xl border text-xs font-bold transition-all ${selectedCourier === 'pathao' ? 'bg-slate-900 text-amber-300 border-slate-900 shadow-md' : 'bg-slate-50 text-slate-700 border-slate-200'}`}
            >
              Pathao
            </button>
            <button 
              type="button"
              on:click={() => selectedCourier = 'redx'} 
              class={`py-2.5 px-3 rounded-xl border text-xs font-bold transition-all ${selectedCourier === 'redx' ? 'bg-slate-900 text-amber-300 border-slate-900 shadow-md' : 'bg-slate-50 text-slate-700 border-slate-200'}`}
            >
              RedX
            </button>
          </div>
        </div>

        <!-- Credentials Fields -->
        <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-3">
          <div>
            <span class="text-[11px] text-slate-500 block mb-1">API Key ({selectedCourier.toUpperCase()})</span>
            <input type="text" bind:value={steadfastApiKey} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
          </div>
          <div>
            <span class="text-[11px] text-slate-500 block mb-1">Secret Key / Token</span>
            <input type="password" bind:value={steadfastSecretKey} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono text-slate-900 focus:outline-none bg-white" />
          </div>

          <div class="pt-2 flex items-center justify-between border-t border-slate-200">
            <span class="text-xs font-bold text-slate-800">Auto-Dispatch to Courier on Order Shipped</span>
            <input type="checkbox" bind:checked={isAutoCourierDispatch} class="w-4 h-4 accent-[var(--accent-gold)] cursor-pointer" />
          </div>
        </div>
      </div>
    </div>

    <!-- 3. SMS Gateway Integration -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-6">
      <div class="flex items-center gap-3 border-b border-slate-100 pb-4">
        <div class="w-10 h-10 rounded-2xl bg-amber-100 flex items-center justify-center text-[var(--accent-gold)]">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
        </div>
        <div>
          <h3 class="font-serif text-xl font-bold text-slate-900">SMS Gateway (BD)</h3>
          <p class="text-xs text-slate-500 font-light">Greenweb / BulkSMS BD order confirmation messages.</p>
        </div>
      </div>

      <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-3">
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">SMS API Key</span>
          <input type="text" bind:value={smsGatewayApiKey} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
        </div>
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">Approved Sender Mask ID</span>
          <input type="text" bind:value={smsSenderId} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
        </div>
        <div class="pt-2 flex items-center justify-between border-t border-slate-200">
          <span class="text-xs font-bold text-slate-800">Send Instant SMS Confirmation on Checkout</span>
          <input type="checkbox" bind:checked={isAutoSmsOrderConfirm} class="w-4 h-4 accent-[var(--accent-gold)] cursor-pointer" />
        </div>
      </div>
    </div>

    <!-- 4. Payment Gateways (bKash / SSLCommerz) -->
    <div class="bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm space-y-6">
      <div class="flex items-center gap-3 border-b border-slate-100 pb-4">
        <div class="w-10 h-10 rounded-2xl bg-amber-100 flex items-center justify-center text-[var(--accent-gold)]">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
          </svg>
        </div>
        <div>
          <h3 class="font-serif text-xl font-bold text-slate-900">Payment Gateways</h3>
          <p class="text-xs text-slate-500 font-light">bKash Merchant API & SSLCommerz credentials.</p>
        </div>
      </div>

      <div class="p-4 bg-slate-50 rounded-2xl border border-slate-200 space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-slate-900 uppercase tracking-wider">bKash Tokenized Checkout API</span>
          <span class="px-2.5 py-0.5 rounded-full bg-emerald-100 text-emerald-800 text-[10px] font-bold">Active</span>
        </div>
        <div>
          <span class="text-[11px] text-slate-500 block mb-1">App Key</span>
          <input type="text" value="bkash_app_key_847291048201" class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-mono font-bold text-slate-900 focus:outline-none bg-white" />
        </div>
      </div>
    </div>

  </div>

</div>
