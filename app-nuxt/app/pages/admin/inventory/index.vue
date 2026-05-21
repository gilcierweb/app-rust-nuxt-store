<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ t('admin.inventory.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.inventory.description') }}</p>
      </div>
      <button class="btn btn-outline btn-sm" type="button" :disabled="pending" @click="refresh">
        <i class="icon-[tabler--refresh] size-4"></i>
        {{ t('admin.inventory.actions.refresh') }}
      </button>
    </div>

    <div class="mb-6 grid grid-cols-1 gap-4 md:grid-cols-4">
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 flex flex-row items-center gap-4">
          <div class="size-12 rounded-full flex items-center justify-center bg-primary/10 text-primary">
            <i class="icon-[tabler--box] size-6"></i>
          </div>
          <div>
            <p class="text-sm text-base-content/60">{{ t('admin.inventory.metrics.variants') }}</p>
            <h3 class="text-2xl font-bold">{{ totalVariants }}</h3>
          </div>
        </div>
      </div>
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 flex flex-row items-center gap-4">
          <div class="size-12 rounded-full flex items-center justify-center bg-success/10 text-success">
            <i class="icon-[tabler--package] size-6"></i>
          </div>
          <div>
            <p class="text-sm text-base-content/60">{{ t('admin.inventory.metrics.onHand') }}</p>
            <h3 class="text-2xl font-bold">{{ totalStock }}</h3>
          </div>
        </div>
      </div>
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 flex flex-row items-center gap-4">
          <div class="size-12 rounded-full flex items-center justify-center bg-info/10 text-info">
            <i class="icon-[tabler--lock] size-6"></i>
          </div>
          <div>
            <p class="text-sm text-base-content/60">{{ t('admin.inventory.metrics.reserved') }}</p>
            <h3 class="text-2xl font-bold">{{ totalReserved }}</h3>
          </div>
        </div>
      </div>
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 flex flex-row items-center gap-4">
          <div class="size-12 rounded-full flex items-center justify-center bg-warning/10 text-warning">
            <i class="icon-[tabler--alert-triangle] size-6"></i>
          </div>
          <div>
            <p class="text-sm text-base-content/60">{{ t('admin.inventory.metrics.alerts') }}</p>
            <h3 class="text-2xl font-bold text-warning">{{ alertCount }}</h3>
          </div>
        </div>
      </div>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.filters.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.inventory.filters.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.filters.status') }}</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">{{ t('admin.inventory.filters.allStatuses') }}</option>
              <option value="out">{{ t('admin.inventory.status.out') }}</option>
              <option value="low">{{ t('admin.inventory.status.low') }}</option>
              <option value="reserved">{{ t('admin.inventory.status.reserved') }}</option>
              <option value="healthy">{{ t('admin.inventory.status.healthy') }}</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.filters.lowThreshold') }}</span>
            </label>
            <input v-model.number="lowStockThreshold" class="input input-bordered w-full" min="0" type="number" />
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            {{ t('admin.inventory.filters.clear') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="pending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.inventory.loading') }}</p>
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.inventory.error', { message: error.message }) }}</span>
    </div>

    <div v-else class="card shadow-base-300/10 w-full shadow-md overflow-hidden">
      <div class="card-body p-0">
        <AdminDataTable
          :data="inventory"
          :columns="columns"
          :total="data?.total || 0"
          :page-index="currentPage - 1"
          :page-size="pageSize"
          @update:page-index="currentPage = $event + 1"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminPaginatedResponse } from '~/types'
import { createColumnHelper } from '@tanstack/vue-table'
import { h, resolveComponent } from 'vue'

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

interface InventorySummary {
  total_variants: number
  total_stock: number
  total_reserved: number
  alert_count: number
}

interface InventoryListResponse extends AdminPaginatedResponse<InventoryItem> {
  summary: InventorySummary
}

const NuxtLinkLocale = resolveComponent('NuxtLinkLocale')

const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()

const searchQuery = ref('')
const debouncedSearchQuery = ref('')
const selectedStatus = ref('')
const lowStockThreshold = ref(5)
const currentPage = ref(1)
const pageSize = ref(20)
const savingVariantIds = ref<number[]>([])
const draftQuantities = reactive<Record<number, number>>({})

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
    case 'out': return t('admin.inventory.status.out')
    case 'low': return t('admin.inventory.status.low')
    case 'reserved': return t('admin.inventory.status.reserved')
    default: return t('admin.inventory.status.healthy')
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

const apiQuery = reactive({
  page: currentPage,
  page_size: pageSize,
  search: computed(() => debouncedSearchQuery.value || undefined),
  status: computed(() => selectedStatus.value || undefined),
  low_stock_threshold: lowStockThreshold
})

const { pending, data, error, refresh } = await useApiFetch<InventoryListResponse>(
  '/api/admin/inventory',
  {
    key: 'admin-inventory-list',
    query: apiQuery
  }
)

const inventory = computed(() => data.value?.items || [])
const summary = computed(() => data.value?.summary)

watch(
  inventory,
  (items) => {
    for (const item of items) {
      draftQuantities[item.variant_id] = stockQuantity(item)
    }
  },
  { immediate: true }
)

const totalStock = computed(() => summary.value?.total_stock ?? 0)
const totalReserved = computed(() => summary.value?.total_reserved ?? 0)
const alertCount = computed(() => summary.value?.alert_count ?? 0)
const totalVariants = computed(() => summary.value?.total_variants ?? 0)

let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null

watch(searchQuery, (value) => {
  currentPage.value = 1
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer)
  searchDebounceTimer = setTimeout(() => {
    debouncedSearchQuery.value = value.trim()
  }, 250)
})

watch([selectedStatus, lowStockThreshold], () => {
  currentPage.value = 1
})

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
  debouncedSearchQuery.value = ''
  selectedStatus.value = ''
  lowStockThreshold.value = 5
  currentPage.value = 1
}

onBeforeUnmount(() => {
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer)
})

const columnHelper = createColumnHelper<InventoryItem>()

const columns = computed(() => {
  lowStockThreshold.value
  return [
  columnHelper.accessor('product_name', {
    header: () => t('admin.inventory.table.item'),
    cell: (info) => {
      const item = info.row.original
      return h('div', [
        h('div', { class: 'font-bold' }, item.product_name || `Product #${item.product_id}`),
        h('div', { class: 'mt-1 text-sm text-base-content/60' }, item.variant_name || `Variant #${item.variant_id}`),
        h(NuxtLinkLocale, {
          to: `/admin/products/${item.product_id}/variants/${item.variant_id}/edit`,
          class: 'link link-primary text-xs'
        }, () => t('admin.inventory.actions.editVariant'))
      ])
    }
  }),
  columnHelper.accessor('sku', {
    header: () => t('admin.inventory.table.sku'),
    cell: (info) => h('span', { class: 'font-mono text-sm' }, info.getValue() || '-')
  }),
  columnHelper.display({
    id: 'status',
    header: () => t('admin.inventory.table.status'),
    cell: (info) => {
      const item = info.row.original
      const children = [
        h('span', { class: ['badge badge-soft badge-sm', statusBadgeClass(item)] }, statusLabel(item))
      ]
      if (item.active === false) {
        children.push(h('div', { class: 'mt-1 text-xs text-base-content/50' }, t('common.status.inactive')))
      }
      return h('div', children)
    }
  }),
  columnHelper.accessor('inventory_quantity', {
    header: () => h('div', { class: 'text-right' }, t('admin.inventory.table.onHand')),
    cell: (info) => h('div', { class: 'text-right font-bold' }, String(stockQuantity(info.row.original)))
  }),
  columnHelper.accessor('reserved_quantity', {
    header: () => h('div', { class: 'text-right' }, t('admin.inventory.table.reserved')),
    cell: (info) => h('div', { class: 'text-right' }, String(info.getValue()))
  }),
  columnHelper.display({
    id: 'available',
    header: () => h('div', { class: 'text-right' }, t('admin.inventory.table.available')),
    cell: (info) => {
      const item = info.row.original
      const qty = availableQuantity(item)
      return h('div', {
        class: ['text-right', qty <= lowStockThreshold.value ? 'text-warning font-bold' : 'font-medium']
      }, String(qty))
    }
  }),
  columnHelper.display({
    id: 'update',
    header: () => h('div', { class: 'text-right' }, t('admin.inventory.table.update')),
    cell: (info) => {
      const item = info.row.original
      return h('div', { class: 'flex gap-2 justify-end' }, [
        h('input', {
          value: draftQuantities[item.variant_id] ?? stockQuantity(item),
          type: 'number',
          min: 0,
          class: 'input input-bordered input-sm w-24 text-right',
          onInput: (e: Event) => {
            draftQuantities[item.variant_id] = Number((e.target as HTMLInputElement).value)
          }
        }),
        h('button', {
          class: 'btn btn-primary btn-sm',
          type: 'button',
          disabled: isSaving(item),
          onClick: () => saveQuantity(item)
        }, isSaving(item)
          ? h('span', { class: 'loading loading-spinner loading-xs' })
          : h('i', { class: 'icon-[tabler--device-floppy] size-4' }))
      ])
    }
  })
]})
</script>
