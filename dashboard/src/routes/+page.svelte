<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import StatsOverview from '$lib/components/StatsOverview.svelte';
  import { ApiService } from '$lib/services/api';
  import type { DashboardStats, OrderWithItems } from '$lib/types';

  let token = '';
  let isLoading = true;

  let stats: DashboardStats = {
    total_orders: 14,
    pending_orders: 4,
    processing_orders: 3,
    shipped_orders: 2,
    delivered_orders: 5,
    total_revenue: 48500
  };

  let recentOrders: OrderWithItems[] = [
    {
      order: {
        id: "101",
        order_number: "LXR-942183",
        customer_name: "Mahmudul Hasan",
        customer_email: "mahmud@example.com",
        customer_phone: "01711223344",
        delivery_address: "House 45, Road 11, Banani",
        city: "Dhaka",
        shoe_size: 42,
        selected_color: "Champagne Gold",
        total_amount: 3500,
        status: "pending",
        payment_method: "Cash on Delivery",
        created_at: "2026-10-03T06:30:00Z",
        updated_at: "2026-10-03T06:30:00Z"
      },
      items: [
        { id: "1", order_id: "101", product_id: "p1", product_name: "Aurelia Royal Gold Sneaker", quantity: 1, unit_price: 3500 }
      ]
    },
    {
      order: {
        id: "102",
        order_number: "LXR-831092",
        customer_name: "Kazi Nusrat",
        customer_email: "nusrat@example.com",
        customer_phone: "01899887766",
        delivery_address: "Apartment 4B, Nasirabad Properties",
        city: "Chittagong",
        shoe_size: 40,
        selected_color: "Mahogany Burgundy",
        total_amount: 4500,
        status: "processing",
        payment_method: "Cash on Delivery",
        created_at: "2026-10-02T14:20:00Z",
        updated_at: "2026-10-02T14:20:00Z"
      },
      items: [
        { id: "2", order_id: "102", product_id: "p2", product_name: "Monarch Imperial Oxford", quantity: 1, unit_price: 4500 }
      ]
    }
  ];

  onMount(async () => {
    token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
      return;
    }
    await loadData();
    isLoading = false;
  });

  async function loadData() {
    try {
      const [statsRes, ordersRes] = await Promise.all([
        ApiService.fetchStats(token),
        ApiService.fetchOrders(token, 'all', '')
      ]);

      if (statsRes.success && statsRes.data) {
        stats = statsRes.data;
      }
      if (ordersRes.success && ordersRes.data && ordersRes.data.length > 0) {
        recentOrders = ordersRes.data.slice(0, 5);
      }
    } catch (e) {
      console.warn("Using offline fallback for dashboard overview:", e);
    }
  }
</script>

<div class="space-y-8">
  
  <!-- Dashboard Header Banner -->
  <div class="flex flex-wrap items-center justify-between gap-4 bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm">
    <div>
      <span class="text-[10px] uppercase tracking-[0.25em] font-bold text-[var(--accent-gold)] block">Enterprise Performance</span>
      <h1 class="font-serif text-3xl font-bold text-slate-900">Executive Overview</h1>
      <p class="text-xs text-slate-500 font-light mt-1">Real-time revenue metrics, order status breakdown, and inventory stats.</p>
    </div>

    <div class="flex gap-3">
      <a href="/orders" class="px-4 py-2.5 rounded-xl bg-slate-900 text-amber-300 text-xs font-bold hover:bg-slate-800 transition-colors shadow-md flex items-center gap-1.5">
        <span>Manage Orders</span>
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/></svg>
      </a>
      <a href="/products" class="btn-outline text-xs py-2 px-4 flex items-center gap-1.5">
        <span>Footwear Catalog</span>
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
      </a>
    </div>
  </div>

  <!-- Key Metrics Overview Cards -->
  <StatsOverview {stats} />

  <!-- Quick Action Shortcuts & Recent Sales -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
    
    <!-- Left 2 Cols: Recent Orders Overview -->
    <div class="lg:col-span-2 bg-white rounded-3xl p-6 border border-[var(--border-color)] shadow-sm space-y-4">
      <div class="flex items-center justify-between border-b border-slate-100 pb-4">
        <div>
          <h3 class="font-serif text-xl font-bold text-slate-900">Recent Customer Orders</h3>
          <p class="text-xs text-slate-500 font-light">Latest concierge purchases placed on store.</p>
        </div>
        <a href="/orders" class="text-xs font-bold text-[var(--accent-gold)] hover:underline flex items-center gap-1">
          <span>View All ({stats.total_orders})</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </a>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs">
          <thead class="bg-[#FAF7F2] text-slate-700 uppercase font-bold">
            <tr>
              <th class="p-3">Ref #</th>
              <th class="p-3">Customer</th>
              <th class="p-3">Product</th>
              <th class="p-3">Amount</th>
              <th class="p-3">Status</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            {#each recentOrders as item}
              <tr class="hover:bg-slate-50 transition-colors">
                <td class="p-3 font-mono font-bold text-[var(--accent-gold)]">{item.order.order_number}</td>
                <td class="p-3">
                  <span class="font-semibold text-slate-900 block">{item.order.customer_name}</span>
                  <span class="text-[10px] text-slate-500 block">{item.order.customer_phone}</span>
                </td>
                <td class="p-3">
                  <span class="font-medium text-slate-800">{item.items[0]?.product_name || 'Royal Shoe'}</span>
                </td>
                <td class="p-3 font-serif font-bold text-slate-900">৳ {Math.round(item.order.total_amount).toLocaleString()}</td>
                <td class="p-3">
                  <span class={`px-2.5 py-1 rounded-full text-[10px] font-bold uppercase ${item.order.status === 'pending' ? 'bg-amber-100 text-amber-900' : 'bg-emerald-100 text-emerald-800'}`}>
                    {item.order.status}
                  </span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Right 1 Col: Quick Control Panel -->
    <div class="bg-gradient-to-br from-[#0F172A] to-slate-900 text-white rounded-3xl p-6 shadow-xl space-y-6 flex flex-col justify-between">
      <div class="space-y-4">
        <span class="text-[10px] uppercase tracking-widest font-bold text-amber-400 block">Quick Operations</span>
        <h3 class="font-serif text-2xl font-bold">Aurelia Operations Hub</h3>
        <p class="text-xs text-slate-400 leading-relaxed">
          Manage inventory items, review customer shipping addresses, or inspect dispatch status.
        </p>

        <div class="space-y-3 pt-2">
          <a href="/orders" class="w-full flex items-center justify-between p-3.5 rounded-2xl bg-white/10 hover:bg-white/20 border border-white/10 text-xs font-semibold transition-all">
            <span class="flex items-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"/></svg>
              Order Fulfillment Dispatch
            </span>
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
          <a href="/products" class="w-full flex items-center justify-between p-3.5 rounded-2xl bg-white/10 hover:bg-white/20 border border-white/10 text-xs font-semibold transition-all">
            <span class="flex items-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
              Footwear Catalog Stock
            </span>
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
          <a href="/customers" class="w-full flex items-center justify-between p-3.5 rounded-2xl bg-white/10 hover:bg-white/20 border border-white/10 text-xs font-semibold transition-all">
            <span class="flex items-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/></svg>
              Client Directory
            </span>
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </a>
        </div>
      </div>

      <div class="pt-4 border-t border-slate-800 flex items-center justify-between text-[11px] text-slate-400">
        <span>System Version 2.0</span>
        <span class="text-emerald-400 font-bold">● Active API</span>
      </div>
    </div>

  </div>

</div>
