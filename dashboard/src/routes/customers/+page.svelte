<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let customers = [
    {
      id: "c1",
      name: "Mahmudul Hasan",
      phone: "01711223344",
      email: "mahmud@example.com",
      city: "Dhaka",
      address: "House 45, Road 11, Banani",
      totalOrders: 3,
      totalSpent: 10500,
      lastOrder: "2026-10-03"
    },
    {
      id: "c2",
      name: "Kazi Nusrat",
      phone: "01899887766",
      email: "nusrat@example.com",
      city: "Chittagong",
      address: "Apartment 4B, Nasirabad Properties",
      totalOrders: 2,
      totalSpent: 9000,
      lastOrder: "2026-10-02"
    },
    {
      id: "c3",
      name: "Shahriar Ahmed",
      phone: "01922334455",
      email: "shahriar@example.com",
      city: "Dhaka",
      address: "Sector 7, Uttara",
      totalOrders: 1,
      totalSpent: 2950,
      lastOrder: "2026-10-01"
    }
  ];

  onMount(() => {
    const token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
    }
  });
</script>

<div class="space-y-6">
  
  <div class="flex items-center justify-between">
    <div>
      <h1 class="font-serif text-3xl font-bold text-slate-900">Client & Customer Directory</h1>
      <p class="text-xs text-slate-500 font-light mt-1">Recorded client profiles, contact numbers, and purchase histories.</p>
    </div>
  </div>

  <div class="bg-white rounded-2xl border border-[var(--border-color)] shadow-sm overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead class="bg-[#FAF7F2] text-slate-700 uppercase font-bold border-b border-[var(--border-color)]">
          <tr>
            <th class="p-4">Client Name</th>
            <th class="p-4">Contact Phone</th>
            <th class="p-4">Location</th>
            <th class="p-4">Total Orders</th>
            <th class="p-4">Lifetime Spend</th>
            <th class="p-4 text-right">Last Purchase</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100">
          {#each customers as cust}
            <tr class="hover:bg-slate-50 transition-colors">
              <td class="p-4">
                <span class="font-bold text-slate-900 block">{cust.name}</span>
                <span class="text-[11px] text-slate-400 block">{cust.email}</span>
              </td>
              <td class="p-4 font-mono font-semibold text-slate-800">
                <span class="flex items-center gap-1.5">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                  {cust.phone}
                </span>
              </td>
              <td class="p-4 text-slate-600">
                <span class="flex items-center gap-1.5">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                  {cust.address}, {cust.city}
                </span>
              </td>
              <td class="p-4 font-bold text-slate-900">{cust.totalOrders} Pairs</td>
              <td class="p-4 font-serif font-bold text-[var(--accent-gold)] text-sm">৳ {cust.totalSpent.toLocaleString()}</td>
              <td class="p-4 text-right text-slate-500">{cust.lastOrder}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

</div>
