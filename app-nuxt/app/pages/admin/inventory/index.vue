<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">Inventory</h1>
        <p class="text-sm text-base-content/60">Track variant stock, cart reservations, and low-stock exposure.</p>
      </div>
      <button class="btn btn-outline btn-sm" type="button" :disabled="pending" @click="refresh">
        <i class="icon-[tabler--refresh] size-4"></i>
        Refresh
      </button>
    </div>

    <div class="mb-6 grid grid-cols-1 gap-4 md:grid-cols-4">
      <div class="rounded-box bg-base-100 border p-4 shadow-sm">
        <p class="text-xs uppercase text-base-content/50">Variants</p>
        <p class="mt-2 text-2xl font-bold">{{ inventory.length }}</p>
      </div>
      <div class="rounded-box bg-base-100 border p-4 shadow-sm">
        <p class="text-xs uppercase text-base-content/50">On hand</p>
        <p class="mt-2 text-2xl font-bold">{{ totalStock }}</p>
      </div>
      <div class="rounded-box bg-base-100 border p-4 shadow-sm">
        <p class="text-xs uppercase text-base-content/50">Reserved</p>
        <p class="mt-2 text-2xl font-bold">{{ totalReserved }}</p>
      </div>
      <div class="rounded-box bg-base-100 border p-4 shadow-sm">
        <p class="text-xs uppercase text-base-content/50">Alerts</p>
        <p class="mt-2 text-2xl font-bold text-warning">{{ alertCount }}</p>
      </div>
    </div>

    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Search</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                placeholder="Product, variant, SKU..."
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Status</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">All statuses</option>
              <option value="out">Out of stock</option>
              <option value="low">Low stock</option>
              <option value="reserved">Reserved</option>
              <option value="healthy">Healthy</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Low threshold</span>
            </label>
            <input v-model.number="lowStockThreshold" class="input input-bordered w-full" min="0" type="number" />
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            Clear
          </button>
        </div>
      </div>
    </div>

    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">Loading inventory...</p>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Failed to load inventory: {{ error.message }}</span>
    </div>

    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th>Item</th>
              <th>SKU</th>
              <th>Status</th>
              <th class="text-right">On hand</th>
              <th class="text-right">Reserved</th>
              <th class="text-right">Available</th>
              <th class="text-right">Update</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in filteredInventory" :key="item.variant_id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <div class="font-bold">{{ item.product_name || `Product #${item.product_id}` }}</div>
                <div class="mt-1 text-sm text-base-content/60">{{ item.variant_name || `Variant #${item.variant_id}` }}</div>
                <NuxtLinkLocale :to="`/admin/products/${item.product_id}/variants/${item.variant_id}/edit`" class="link link-primary text-xs">
                  Edit variant
                </NuxtLinkLocale>
              </td>
              <td class="font-mono text-sm">{{ item.sku || '-' }}</td>
              <td>
                <span :class="['badge badge-soft badge-sm', statusBadgeClass(item)]">
                  {{ statusLabel(item) }}
                </span>
                <div v-if="item.active === false" class="mt-1 text-xs text-base-content/50">Inactive</div>
              </td>
              <td class="text-right font-bold">{{ stockQuantity(item) }}</td>
              <td class="text-right">{{ item.reserved_quantity }}</td>
              <td class="text-right" :class="availableQuantity(item) <= lowStockThreshold ? 'text-warning font-bold' : 'font-medium'">
                {{ availableQuantity(item) }}
              </td>
              <td class="text-right">
                <div class="flex justify-end gap-2">
                  <input
                    v-model.number="draftQuantities[item.variant_id]"
                    class="input input-bordered input-sm w-24 text-right"
                    min="0"
                    type="number"
                  />
                  <button class="btn btn-primary btn-sm" type="button" :disabled="isSaving(item)" @click="saveQuantity(item)">
                    <span v-if="isSaving(item)" class="loading loading-spinner loading-xs"></span>
                    <i v-else class="icon-[tabler--device-floppy] size-4"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredInventory.length === 0">
              <td colspan="7" class="py-20 text-center text-base-content/50 italic">
                No inventory items found.
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface InventoryItem {
  variant_id: number
  product_id: number
  product_name?: string | null
  variant_name?: string | null
  sku?: string | null
  active?: boolean | null
  inventory_quantity?: number | null
  reserved_quantity: number
}

const { apiFetch, useApiFetch } = useApi()

const searchQuery = ref('')
const selectedStatus = ref('')
const lowStockThreshold = ref(5)
const savingVariantIds = ref<number[]>([])
const draftQuantities = reactive<Record<number, number>>({})

const { pending, data, error, refresh } = await useApiFetch<InventoryItem[]>(
  '/api/admin/inventory/',
  { key: 'admin-inventory-list' }
)

const inventory = computed(() => data.value || [])

watch(
  inventory,
  (items) => {
    for (const item of items) {
      draftQuantities[item.variant_id] = stockQuantity(item)
    }
  },
  { immediate: true }
)

const totalStock = computed(() => inventory.value.reduce((sum, item) => sum + stockQuantity(item), 0))
const totalReserved = computed(() => inventory.value.reduce((sum, item) => sum + Number(item.reserved_quantity || 0), 0))
const alertCount = computed(() => inventory.value.filter(item => statusKey(item) === 'out' || statusKey(item) === 'low').length)

const filteredInventory = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return inventory.value.filter((item) => {
    const matchesSearch = !query ||
      (item.product_name || '').toLowerCase().includes(query) ||
      (item.variant_name || '').toLowerCase().includes(query) ||
      (item.sku || '').toLowerCase().includes(query)

    const matchesStatus = !selectedStatus.value || statusKey(item) === selectedStatus.value
    return matchesSearch && matchesStatus
  })
})

const stockQuantity = (item: InventoryItem) => Number(item.inventory_quantity || 0)

const availableQuantity = (item: InventoryItem) => {
  return stockQuantity(item) - Number(item.reserved_quantity || 0)
}

const statusKey = (item: InventoryItem) => {
  const available = availableQuantity(item)
  const stock = stockQuantity(item)

  if (stock <= 0 || available <= 0) return 'out'
  if (Number(item.reserved_quantity || 0) > 0) return 'reserved'
  if (available <= lowStockThreshold.value) return 'low'
  return 'healthy'
}

const statusLabel = (item: InventoryItem) => {
  switch (statusKey(item)) {
    case 'out': return 'Out of stock'
    case 'low': return 'Low stock'
    case 'reserved': return 'Reserved'
    default: return 'Healthy'
  }
}

const statusBadgeClass = (item: InventoryItem) => {
  switch (statusKey(item)) {
    case 'out': return 'badge-error'
    case 'low': return 'badge-warning'
    case 'reserved': return 'badge-info'
    default: return 'badge-success'
  }
}

const isSaving = (item: InventoryItem) => savingVariantIds.value.includes(item.variant_id)

const saveQuantity = async (item: InventoryItem) => {
  const quantity = Number(draftQuantities[item.variant_id] || 0)
  if (quantity < 0) return

  savingVariantIds.value = [...savingVariantIds.value, item.variant_id]
  try {
    await apiFetch(`/api/admin/inventory/${item.variant_id}`, {
      method: 'PATCH',
      body: { inventory_quantity: quantity }
    })
    await refresh()
  } finally {
    savingVariantIds.value = savingVariantIds.value.filter(id => id !== item.variant_id)
  }
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedStatus.value = ''
  lowStockThreshold.value = 5
}
</script>
