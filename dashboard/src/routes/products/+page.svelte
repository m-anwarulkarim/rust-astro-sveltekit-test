<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  interface ProductItem {
    id: string;
    title: string;
    subtitle: string;
    price: number;
    original_price: number;
    category: string;
    image_url: string;
    badge: string;
    stock: number;
  }

  let products: ProductItem[] = [
    {
      id: "1",
      title: "Aurelia Royal Gold Sneaker",
      subtitle: "Handcrafted Italian Calfskin & Gold Trim",
      price: 3500,
      original_price: 4200,
      category: "Luxury Sneakers",
      image_url: "/shoes/aurelia.jpg",
      badge: "Bestseller",
      stock: 30
    },
    {
      id: "2",
      title: "Monarch Imperial Oxford",
      subtitle: "Hand-Burnished Burgundy Leather",
      price: 4500,
      original_price: 5500,
      category: "Formal Dress",
      image_url: "/shoes/monarch.jpg",
      badge: "Limited Edition",
      stock: 18
    },
    {
      id: "3",
      title: "Vanguard Phantom Runner",
      subtitle: "Cognac Suede & Sculpted Sole",
      price: 2950,
      original_price: 3600,
      category: "Urban Athleisure",
      image_url: "/shoes/vanguard.jpg",
      badge: "New Release",
      stock: 25
    },
    {
      id: "4",
      title: "Sovereign Velvet Loafer",
      subtitle: "Royal Indigo Velvet with Embroidered Crest",
      price: 3800,
      original_price: 4600,
      category: "Velvet Loafers",
      image_url: "/shoes/sovereign.jpg",
      badge: "Premium Choice",
      stock: 15
    }
  ];

  let editingProduct: ProductItem | null = null;
  let isAddModal = false;
  let saveNotification = false;

  onMount(() => {
    const token = localStorage.getItem('admin_token') || '';
    if (!token) {
      goto('/login');
    }
  });

  function openEditModal(shoe: ProductItem) {
    editingProduct = { ...shoe };
    isAddModal = false;
  }

  function openAddModal() {
    editingProduct = {
      id: String(Date.now()),
      title: '',
      subtitle: '',
      price: 3500,
      original_price: 4200,
      category: 'Luxury Sneakers',
      image_url: '/shoes/aurelia.jpg',
      badge: 'New Edition',
      stock: 20
    };
    isAddModal = true;
  }

  function closeModal() {
    editingProduct = null;
  }

  function handleSaveProduct() {
    if (!editingProduct || !editingProduct.title) return;

    if (isAddModal) {
      products = [editingProduct, ...products];
    } else {
      products = products.map(p => p.id === editingProduct!.id ? { ...editingProduct! } : p);
    }

    closeModal();
    saveNotification = true;
    setTimeout(() => {
      saveNotification = false;
    }, 3000);
  }
</script>

<div class="space-y-6">
  
  <!-- Header Banner -->
  <div class="flex flex-wrap items-center justify-between gap-4 bg-white p-6 rounded-3xl border border-[var(--border-color)] shadow-sm">
    <div>
      <span class="text-[10px] uppercase tracking-[0.25em] font-bold text-[var(--accent-gold)] block">Inventory Control</span>
      <h1 class="font-serif text-3xl font-bold text-slate-900">Footwear Catalog & Stock</h1>
      <p class="text-xs text-slate-500 font-light mt-1">Manage shoe details, prices in BDT (৳), stock levels, and badges.</p>
    </div>

    <button on:click={openAddModal} class="btn-primary-gold text-xs py-2.5 px-5 font-bold shadow-md flex items-center gap-1.5">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      <span>Add New Footwear Pair</span>
    </button>
  </div>

  {#if saveNotification}
    <div class="p-4 bg-emerald-50 border border-emerald-200 text-emerald-900 text-xs font-bold rounded-2xl flex items-center justify-between animate-in fade-in">
      <span class="flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
        Product inventory details updated successfully!
      </span>
      <span class="text-[10px] text-emerald-600 uppercase">Live Update</span>
    </div>
  {/if}

  <!-- Products Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
    {#each products as shoe}
      <div class="bg-white rounded-2xl border border-[var(--border-color)] overflow-hidden shadow-sm hover:shadow-md transition-shadow flex flex-col justify-between">
        
        <div class="relative h-56 bg-slate-100 overflow-hidden">
          <span class="absolute top-3 left-3 bg-white/90 backdrop-blur-md px-2.5 py-1 rounded-full text-[10px] font-bold text-slate-900 shadow-sm border border-slate-200">
            {shoe.badge}
          </span>
          <span class={`absolute top-3 right-3 px-2.5 py-1 rounded-full text-[10px] font-bold ${shoe.stock > 20 ? 'bg-emerald-100 text-emerald-800' : 'bg-amber-100 text-amber-900'}`}>
            {shoe.stock} Left
          </span>
          <img src={shoe.image_url} alt={shoe.title} class="w-full h-full object-cover" />
        </div>

        <div class="p-5 space-y-3 flex-1 flex flex-col justify-between">
          <div>
            <span class="text-[10px] uppercase font-bold text-[var(--accent-gold)] tracking-wider block mb-0.5">{shoe.category}</span>
            <h3 class="font-serif font-bold text-slate-900 text-base leading-snug">{shoe.title}</h3>
            <p class="text-xs text-slate-500 line-clamp-1 mt-1">{shoe.subtitle}</p>
          </div>

          <div class="pt-3 border-t border-slate-100 flex items-center justify-between">
            <div>
              <span class="font-serif text-lg font-bold text-slate-900 block">৳ {shoe.price.toLocaleString()}</span>
              <span class="text-xs text-slate-400 line-through">৳ {shoe.original_price.toLocaleString()}</span>
            </div>

            <button 
              on:click={() => openEditModal(shoe)}
              class="px-3.5 py-1.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-800 hover:bg-slate-900 hover:text-amber-300 hover:border-slate-900 transition-colors flex items-center gap-1 shadow-sm"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              <span>Edit Pair</span>
            </button>
          </div>
        </div>

      </div>
    {/each}
  </div>

</div>

<!-- Product Edit & Add Modal -->
{#if editingProduct}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/60 backdrop-blur-sm p-4 overflow-y-auto">
    <div class="bg-white rounded-3xl max-w-lg w-full p-6 space-y-6 shadow-2xl border border-[var(--border-color)]">
      
      <div class="flex items-center justify-between border-b border-slate-100 pb-4">
        <div>
          <span class="text-[10px] uppercase font-bold text-[var(--accent-gold)] tracking-wider block">Inventory Management</span>
          <h3 class="font-serif text-2xl font-bold text-slate-900">{isAddModal ? 'Add New Footwear Pair' : 'Edit Footwear Details'}</h3>
        </div>
        <button on:click={closeModal} class="text-slate-400 hover:text-slate-900 font-bold p-1">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <form on:submit|preventDefault={handleSaveProduct} class="space-y-4 text-xs">
        <div>
          <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Product Title *</label>
          <input type="text" bind:value={editingProduct.title} required class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none" />
        </div>

        <div>
          <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Subtitle / Material Spec</label>
          <input type="text" bind:value={editingProduct.subtitle} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-medium text-slate-800 focus:border-[var(--accent-gold)] focus:outline-none" />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Regular Price (BDT ৳) *</label>
            <input type="number" bind:value={editingProduct.price} required class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none" />
          </div>
          <div>
            <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Original Price (BDT ৳)</label>
            <input type="number" bind:value={editingProduct.original_price} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none" />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Stock Pairs *</label>
            <input type="number" bind:value={editingProduct.stock} required class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none" />
          </div>
          <div>
            <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Badge Tag</label>
            <input type="text" bind:value={editingProduct.badge} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none" />
          </div>
        </div>

        <div>
          <label class="block font-bold text-slate-800 uppercase tracking-wider mb-1">Category</label>
          <select bind:value={editingProduct.category} class="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-xs font-bold text-slate-900 focus:border-[var(--accent-gold)] focus:outline-none bg-white">
            <option value="Luxury Sneakers">Luxury Sneakers</option>
            <option value="Formal Dress">Formal Dress</option>
            <option value="Urban Athleisure">Urban Athleisure</option>
            <option value="Velvet Loafers">Velvet Loafers</option>
          </select>
        </div>

        <div class="pt-3 flex gap-3">
          <button type="button" on:click={closeModal} class="w-1/2 btn-outline justify-center py-3 text-xs">
            Cancel
          </button>
          <button type="submit" class="w-1/2 btn-primary-gold justify-center py-3 text-xs uppercase font-bold shadow-lg">
            Save Footwear
          </button>
        </div>
      </form>

    </div>
  </div>
{/if}
