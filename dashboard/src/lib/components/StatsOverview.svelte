<script lang="ts">
  import type { DashboardStats } from '$lib/types';
  export let stats: DashboardStats;
</script>

<div class="grid grid-cols-2 lg:grid-cols-5 gap-5">
  <div class="card-stat">
    <span class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block">Total Revenue</span>
    <span class="font-serif text-2xl font-bold text-[var(--accent-gold)] block mt-1">৳ {Math.round(stats.total_revenue).toLocaleString()}</span>
    <span class="text-[11px] text-emerald-600 font-medium">Updated live</span>
  </div>

  <div class="card-stat">
    <span class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block">Total Orders</span>
    <span class="font-serif text-2xl font-bold text-slate-900 block mt-1">{stats.total_orders}</span>
    <span class="text-[11px] text-slate-500">All recorded sales</span>
  </div>

  <div class="card-stat">
    <span class="text-[10px] font-bold text-amber-600 uppercase tracking-wider block">Pending</span>
    <span class="font-serif text-2xl font-bold text-amber-700 block mt-1">{stats.pending_orders}</span>
    <span class="text-[11px] text-amber-800">Requires confirmation</span>
  </div>

  <div class="card-stat">
    <span class="text-[10px] font-bold text-purple-600 uppercase tracking-wider block">Shipped</span>
    <span class="font-serif text-2xl font-bold text-purple-700 block mt-1">{stats.shipped_orders}</span>
    <span class="text-[11px] text-purple-800">In transit</span>
  </div>

  <div class="card-stat">
    <span class="text-[10px] font-bold text-emerald-600 uppercase tracking-wider block">Delivered</span>
    <span class="font-serif text-2xl font-bold text-emerald-700 block mt-1">{stats.delivered_orders}</span>
    <span class="text-[11px] text-emerald-800">Completed orders</span>
  </div>
</div>
