<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { ApiService } from '$lib/services/api';
  import type { OrderWithItems } from '$lib/types';

  let token = '';
  let isLoading = true;
  let activeFilter = 'all';
  let searchQuery = '';
  let selectedOrderModal: OrderWithItems | null = null;

  let orders: OrderWithItems[] = [
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
    },
    {
      order: {
        id: "103",
        order_number: "LXR-762145",
        customer_name: "Shahriar Ahmed",
        customer_email: "shahriar@example.com",
        customer_phone: "01922334455",
        delivery_address: "Sector 7, Uttara",
        city: "Dhaka",
        shoe_size: 43,
        selected_color: "Cognac Suede",
        total_amount: 2950,
        status: "shipped",
        payment_method: "Cash on Delivery",
        created_at: "2026-10-01T09:15:00Z",
        updated_at: "2026-10-01T09:15:00Z"
      },
      items: [
        { id: "3", order_id: "103", product_id: "p3", product_name: "Vanguard Phantom Runner", quantity: 1, unit_price: 2950 }
      ]
    }
  ];

  onMount(async () => {
    token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
      return;
    }
    await loadOrders();
    isLoading = false;
  });

  async function loadOrders() {
    try {
      const res = await ApiService.fetchOrders(token, activeFilter, searchQuery);
      if (res.success && res.data && res.data.length > 0) {
        orders = res.data;
      }
    } catch (e) {
      console.warn("Using offline fallback:", e);
    }
  }

  async function updateStatus(orderId: string, newStatus: any) {
    try {
      const res = await ApiService.updateOrderStatus(token, orderId, newStatus);
      if (res.success) {
        orders = orders.map(o => {
          if (o.order.id === orderId) {
            o.order.status = newStatus;
          }
          return o;
        });
        await loadOrders();
      }
    } catch (e) {
      orders = orders.map(o => {
        if (o.order.id === orderId) {
          o.order.status = newStatus;
        }
        return o;
      });
    }
  }

  $: filteredOrders = orders.filter(item => {
    const o = item.order;
    const matchesStatus = activeFilter === 'all' || o.status.toLowerCase() === activeFilter.toLowerCase();
    const query = searchQuery.toLowerCase();
    const matchesSearch = !query || 
      o.customer_name.toLowerCase().includes(query) || 
      o.customer_phone.includes(query) || 
      o.order_number.toLowerCase().includes(query);
    
    return matchesStatus && matchesSearch;
  });
</script>

<div class="space-y-6">
  
  <!-- Page Header -->
  <div class="flex flex-wrap items-center justify-between gap-4">
    <div>
      <h1 class="font-serif text-3xl font-bold text-slate-900">Order Dispatch & Management</h1>
      <p class="text-xs text-slate-500 font-light mt-1">Review customer delivery details, update dispatch status, and handle orders.</p>
    </div>

    <button on:click={loadOrders} class="px-4 py-2.5 rounded-xl bg-slate-900 text-amber-300 text-xs font-bold hover:bg-slate-800 transition-colors flex items-center gap-1.5 shadow-sm">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
      <span>Refresh Orders</span>
    </button>
  </div>

  <!-- Filters & Search Toolbar -->
  <div class="bg-white p-4 rounded-2xl border border-[var(--border-color)] shadow-sm flex flex-wrap items-center justify-between gap-4">
    <div class="flex flex-wrap items-center gap-2">
      {#each ['all', 'pending', 'processing', 'shipped', 'delivered', 'cancelled'] as status}
        <button 
          on:click={() => { activeFilter = status; loadOrders(); }}
          class={`px-3.5 py-2 rounded-xl text-xs font-bold capitalize transition-all ${activeFilter === status ? 'bg-slate-900 text-amber-300 shadow-sm' : 'bg-slate-50 text-slate-600 hover:bg-slate-100'}`}
        >
          {status}
        </button>
      {/each}
    </div>

    <div class="w-full sm:w-72">
      <input 
        type="text" 
        bind:value={searchQuery}
        placeholder="Search by customer name, phone, order #..."
        class="w-full px-4 py-2 rounded-xl border border-slate-200 text-xs focus:border-[var(--accent-gold)] focus:outline-none"
      />
    </div>
  </div>

  <!-- Orders Table -->
  <div class="bg-white rounded-2xl border border-[var(--border-color)] shadow-sm overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead class="bg-[#FAF7F2] text-slate-700 uppercase tracking-wider font-bold border-b border-[var(--border-color)]">
          <tr>
            <th class="p-4">Ref #</th>
            <th class="p-4">Customer Details</th>
            <th class="p-4">Shoe & Finish</th>
            <th class="p-4">Size</th>
            <th class="p-4">Amount</th>
            <th class="p-4">Status Dispatch</th>
            <th class="p-4 text-right">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100">
          {#each filteredOrders as item}
            <tr class="hover:bg-slate-50/80 transition-colors">
              <td class="p-4 font-mono font-bold text-[var(--accent-gold)]">
                {item.order.order_number}
              </td>

              <td class="p-4">
                <span class="font-bold text-slate-900 block">{item.order.customer_name}</span>
                <span class="text-[11px] text-slate-500 flex items-center gap-1 mt-0.5">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                  {item.order.customer_phone}
                </span>
                <span class="text-[11px] text-slate-400 flex items-center gap-1 mt-0.5 truncate max-w-xs">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                  {item.order.delivery_address}, {item.order.city}
                </span>
              </td>

              <td class="p-4">
                <span class="font-semibold text-slate-800 block">
                  {item.items && item.items.length > 0 ? item.items[0].product_name : 'Aurelia Footwear'}
                </span>
                <span class="text-[11px] text-amber-700 font-medium flex items-center gap-1 mt-0.5">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-amber-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/></svg>
                  {item.order.selected_color}
                </span>
              </td>

              <td class="p-4">
                <span class="px-2.5 py-1 rounded-md bg-slate-100 font-bold text-slate-800">
                  EU {item.order.shoe_size}
                </span>
              </td>

              <td class="p-4 font-serif font-bold text-slate-900 text-sm">
                ৳ {Math.round(item.order.total_amount).toLocaleString()}
              </td>

              <td class="p-4">
                <select 
                  value={item.order.status}
                  on:change={(e) => updateStatus(item.order.id, e.currentTarget.value)}
                  class={`badge-status cursor-pointer focus:outline-none ${item.order.status === 'pending' ? 'badge-pending' : item.order.status === 'processing' ? 'badge-processing' : item.order.status === 'shipped' ? 'badge-shipped' : item.order.status === 'delivered' ? 'badge-delivered' : 'badge-cancelled'}`}
                >
                  <option value="pending">Pending</option>
                  <option value="processing">Processing</option>
                  <option value="shipped">Shipped</option>
                  <option value="delivered">Delivered</option>
                  <option value="cancelled">Cancelled</option>
                </select>
              </td>

              <td class="p-4 text-right">
                <button 
                  on:click={() => selectedOrderModal = item}
                  class="px-3 py-1.5 rounded-lg border border-slate-200 text-[11px] font-semibold text-slate-700 hover:bg-slate-100"
                >
                  View Details
                </button>
              </td>
            </tr>
          {/each}

          {#if filteredOrders.length === 0}
            <tr>
              <td colspan="7" class="p-8 text-center text-slate-400 font-light">
                No orders found matching the filter criteria.
              </td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  </div>

</div>

<!-- Detailed Order View Modal -->
{#if selectedOrderModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/50 backdrop-blur-sm p-4">
    <div class="bg-white rounded-3xl max-w-lg w-full p-6 space-y-6 shadow-2xl border border-[var(--border-color)]">
      <div class="flex items-center justify-between border-b border-slate-100 pb-4">
        <div>
          <span class="text-[10px] uppercase font-bold text-[var(--accent-gold)] tracking-wider block">Order Reference</span>
          <h3 class="font-serif text-2xl font-bold text-slate-900">{selectedOrderModal.order.order_number}</h3>
        </div>
        <button on:click={() => selectedOrderModal = null} class="text-slate-400 hover:text-slate-900 font-bold">✕</button>
      </div>

      <div class="space-y-4 text-xs">
        <div class="p-3 bg-[#FAF7F2] rounded-xl border border-[var(--border-color)] space-y-1">
          <span class="font-bold text-slate-900 block">Customer Information</span>
          <p><strong>Name:</strong> {selectedOrderModal.order.customer_name}</p>
          <p><strong>Phone:</strong> {selectedOrderModal.order.customer_phone}</p>
          <p><strong>Email:</strong> {selectedOrderModal.order.customer_email}</p>
          <p><strong>Address:</strong> {selectedOrderModal.order.delivery_address}, {selectedOrderModal.order.city}</p>
        </div>

        <div class="p-3 bg-slate-50 rounded-xl border border-slate-200 space-y-1">
          <span class="font-bold text-slate-900 block">Footwear Specifications</span>
          <p><strong>Shoe Size:</strong> EU {selectedOrderModal.order.shoe_size}</p>
          <p><strong>Selected Color Finish:</strong> {selectedOrderModal.order.selected_color}</p>
          <p><strong>Payment Method:</strong> {selectedOrderModal.order.payment_method}</p>
          <p><strong>Total Price:</strong> <span class="font-serif text-sm font-bold text-[var(--accent-gold)]">৳ {Math.round(selectedOrderModal.order.total_amount).toLocaleString()}</span></p>
        </div>
      </div>

      <button on:click={() => selectedOrderModal = null} class="w-full btn-primary-gold justify-center py-2.5 text-xs uppercase font-bold">
        Close Details
      </button>
    </div>
  </div>
{/if}
